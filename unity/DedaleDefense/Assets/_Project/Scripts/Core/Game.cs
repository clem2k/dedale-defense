using System;
using System.Collections.Generic;

namespace DedaleDefense.Core
{
    /// <summary>
    /// Simulation complète d'une partie. Pas de dépendance à UnityEngine : déterministe avec un Random à graine fixe.
    /// Port fidèle de app/src/game.js (blocs « Game state » à « Main update »).
    /// </summary>
    public sealed class Game
    {
        public const double Step = 1.0 / 60.0;
        const int W = Grid.Width, H = Grid.Height, Cell = Grid.Cell, C = Grid.Cols;

        public readonly ModeId Mode;
        public readonly Board Board = new Board();
        public int Lives, Gold, Score, Kills, Wave;
        public bool Started, Paused, Over, Won, NewBest;
        public double Timer, Time;
        public int Best;
        public readonly List<Tower> Towers = new List<Tower>();
        public List<Creep> Creeps = new List<Creep>();
        public List<Spawner> Spawners = new List<Spawner>();
        public List<Shot> Shots = new List<Shot>();
        public List<Fx> Fx = new List<Fx>();
        public List<FloatText> Floats = new List<FloatText>();
        public int[] FieldR, FieldB;
        public List<List<int>> Path = new List<List<int>>();
        public event Action Changed;   // l'interface se rafraîchit
        public event Action Ended;

        readonly Random rng;
        readonly IBestStore store;
        int nextId = 1;

        public Game(ModeId mode, int seed, IBestStore store)
        {
            Mode = mode;
            rng = new Random(seed);
            this.store = store ?? new MemoryBestStore();
            ModeDef m = GameData.Modes[mode];
            Lives = m.Lives;
            Gold = m.Gold;
            RefreshFields();
        }

        public ModeDef ModeDef { get { return GameData.Modes[Mode]; } }

        double Rand() { return rng.NextDouble(); }
        void NotifyChanged() { var h = Changed; if (h != null) h(); }

        // ---------- Chemins ----------
        public void RefreshFields()
        {
            FieldR = Board.ComputeField(Board.ExitR);
            FieldB = Board.ComputeField(Board.ExitB);
            Path = new List<List<int>>();
            var routes = new[] { new KeyValuePair<int[], int>(FieldR, Board.EntryL[1]), new KeyValuePair<int[], int>(FieldB, Board.EntryT[1]) };
            foreach (var route in routes)
            {
                var pts = new List<int>();
                int i = route.Value, guard = 0;
                while (i >= 0 && route.Key[i] > 0 && route.Key[i] != Board.Inf && guard++ < 800)
                {
                    pts.Add(i);
                    i = Board.NextStep(route.Key, i % C, i / C);
                }
                if (i >= 0) pts.Add(i);
                Path.Add(pts);
            }
        }

        // ---------- Placement ----------
        public static int[] CellsOf(int cx, int cy)
        {
            return new[] { cy * C + cx, cy * C + cx + 1, (cy + 1) * C + cx, (cy + 1) * C + cx + 1 };
        }

        /// <summary>Vérifie qu'une tour 2×2 peut être posée en (cx,cy) sans fermer le dernier passage.</summary>
        public PlaceResult PlaceCheck(int cx, int cy)
        {
            if (cx < 1 || cy < 1 || cx + 1 > Grid.Cols - 2 || cy + 1 > Grid.Rows - 2) return PlaceResult.Out;
            int[] cells = CellsOf(cx, cy);
            foreach (int c in cells) if (Board.TowerAt[c] != null || Board.Wall[c]) return PlaceResult.Taken;
            var set = new HashSet<int>(cells);
            foreach (Creep k in Creeps)
            {
                if (k.Flying) continue;
                if (set.Contains(k.Cell) || set.Contains(k.Wcell)) return PlaceResult.Creep;
            }
            int[] fr = Board.ComputeField(Board.ExitR, set);
            int[] fb = Board.ComputeField(Board.ExitB, set);
            if (!Array.Exists(Board.EntryL, i => fr[i] != Board.Inf) || !Array.Exists(Board.EntryT, i => fb[i] != Board.Inf)) return PlaceResult.Block;
            if (Array.Exists(Board.EntryL, i => fr[i] == Board.Inf) || Array.Exists(Board.EntryT, i => fb[i] == Board.Inf)) return PlaceResult.Block;
            foreach (Creep k in Creeps)
            {
                if (k.Flying || k.Wcell < 0) continue;
                int[] f = k.Target == Lane.R ? fr : fb;
                if (f[k.Wcell] == Board.Inf) return PlaceResult.Block;
            }
            return PlaceResult.Ok;
        }

        public static string Message(PlaceResult r)
        {
            switch (r)
            {
                case PlaceResult.Block: return "Ne bloquez pas le passage !";
                case PlaceResult.Creep: return "Un ennemi est là";
                case PlaceResult.Taken: return "Place occupée";
                case PlaceResult.Out: return "Hors du plateau";
                case PlaceResult.NoGold: return "Pas assez d'or";
                default: return "";
            }
        }

        public PlaceResult TryPlace(int type, int cx, int cy)
        {
            if (Over) return PlaceResult.Over;
            TowerDef def = GameData.Towers[type];
            if (Gold < def.Cost) { Flash(Message(PlaceResult.NoGold), cx, cy); return PlaceResult.NoGold; }
            PlaceResult r = PlaceCheck(cx, cy);
            if (r != PlaceResult.Ok) { Flash(Message(r), cx, cy); return r; }
            var t = new Tower
            {
                Id = nextId++, Type = type, Cx = cx, Cy = cy, Level = 0,
                X = (cx + 1) * Cell, Y = (cy + 1) * Cell, Cool = 0, Angle = -Math.PI / 2,
                Spent = def.Cost, BuiltWave = Wave, BuiltStarted = Started,
            };
            foreach (int c in CellsOf(cx, cy)) Board.TowerAt[c] = t;
            Towers.Add(t);
            Gold -= def.Cost;
            RefreshFields();
            NotifyChanged();
            return PlaceResult.Ok;
        }

        void Flash(string text, int cx, int cy)
        {
            Floats.Add(new FloatText { X = (cx + 1) * Cell, Y = (cy + 1) * Cell - 6, Text = text, Color = "#fda29b", Life = 1.2, Max = 1.2 });
        }

        public struct Stats { public double Dmg, Range, Rate, Slow; }

        public static Stats TowerStats(Tower t)
        {
            TowerDef d = GameData.Towers[t.Type];
            int L = t.Level;
            return new Stats
            {
                Dmg = d.Dmg[L],
                Range = d.Range * (1 + 0.08 * L),
                Rate = d.Rate * (1 - 0.04 * L),
                Slow = d.Slow != null ? d.Slow[L] : 0,
            };
        }

        /// <summary>Remboursement intégral avant la 1re vague, 75 % ensuite.</summary>
        public static int SellValue(Tower t)
        {
            if (!t.BuiltStarted) return t.Spent;
            return (int)Math.Floor(t.Spent * 0.75);
        }

        public bool Upgrade(Tower t)
        {
            if (t == null || Over) return false;
            TowerDef d = GameData.Towers[t.Type];
            if (t.Level >= d.Ups.Length) return false;
            int cost = d.Ups[t.Level];
            if (Gold < cost) { Flash(Message(PlaceResult.NoGold), t.Cx, t.Cy); return false; }
            Gold -= cost;
            t.Spent += cost;
            t.Level++;
            Fx.Add(new Fx { Kind = FxKind.Ring, X = t.X, Y = t.Y, R = 4, Max = 22, Color = d.Accent, Life = 0.4, MaxLife = 0.4 });
            NotifyChanged();
            return true;
        }

        public bool Sell(Tower t)
        {
            if (t == null || Over) return false;
            int v = SellValue(t);
            Gold += v;
            foreach (int c in CellsOf(t.Cx, t.Cy)) Board.TowerAt[c] = null;
            Towers.Remove(t);
            Floats.Add(new FloatText { X = t.X, Y = t.Y, Text = "+" + v, Color = "#fac515", Life = 1, Max = 1 });
            RefreshFields();
            NotifyChanged();
            return true;
        }

        // ---------- Vagues ----------
        public void SendWave()
        {
            if (Over || Wave >= Grid.TotalWaves) return;
            if (Started && Timer > 0)
            {
                int bonus = (int)Math.Ceiling(Timer) * (Wave + 1);
                Score += bonus;
                Floats.Add(new FloatText { X = W / 2.0, Y = 24, Text = "Envoi anticipé +" + bonus + " pts", Color = "#fac515", Life = 1.6, Max = 1.6 });
            }
            Started = true;
            Wave++;
            WaveDef def = GameData.Wave(Wave);
            Spawners.Add(new Spawner { Def = def, Left = def.Count, T = 0, K = 0 });
            Timer = ModeDef.Gap;
            NotifyChanged();
        }

        void Spawn(WaveDef def, int k)
        {
            CreepDef c = GameData.Creeps[def.Type];
            Lane target = def.N >= 15 ? (k % 2 != 0 ? Lane.B : Lane.R) : (def.N % 2 != 0 ? Lane.R : Lane.B);
            MakeCreep(def.Type, def.Hp * c.Hp * ModeDef.Hp, def.Bounty, target, null);
        }

        Creep MakeCreep(CreepType type, double hp, double bounty, Lane target, Creep from)
        {
            CreepDef c = GameData.Creeps[type];
            int lane = (int)(Rand() * 4);
            double x, y;
            int wcell;
            if (from != null)
            {
                x = from.X + (Rand() - 0.5) * 6; y = from.Y + (Rand() - 0.5) * 6;
                wcell = from.Wcell;
            }
            else if (target == Lane.R)
            {
                x = -Cell * 0.6; y = (Grid.OpenRows[lane] + 0.5) * Cell; wcell = Board.EntryL[lane];
            }
            else
            {
                x = (Grid.OpenCols[lane] + 0.5) * Cell; y = -Cell * 0.6; wcell = Board.EntryT[lane];
            }
            var k = new Creep
            {
                Id = nextId++, Type = type, X = x, Y = y, Hp = hp, MaxHp = hp, Bounty = bounty, Target = target,
                Speed = c.Speed, Flying = c.Flying, R = c.R, Wcell = wcell, Wob = Rand() * Math.PI * 2,
            };
            if (k.Flying)
            {
                k.Wcell = -1;
                k.Wx = target == Lane.R ? W + Cell : x;
                k.Wy = target == Lane.R ? y : H + Cell;
            }
            else if (from != null && from.Exiting)
            {
                k.Exiting = true; k.Wx = from.Wx; k.Wy = from.Wy; k.Wcell = -1;
            }
            else
            {
                k.Wx = (wcell % C + 0.5) * Cell;
                k.Wy = (wcell / C + 0.5) * Cell;
            }
            Creeps.Add(k);
            return k;
        }

        /// <summary>Distance restante jusqu'à la sortie (sert à choisir la cible la plus avancée).</summary>
        public double Remaining(Creep k)
        {
            if (k.Flying) return k.Target == Lane.R ? W - k.X : H - k.Y;
            if (k.Exiting) return 0;
            int[] f = k.Target == Lane.R ? FieldR : FieldB;
            return (f[k.Wcell] / 10.0) * Cell + Math.Sqrt((k.Wx - k.X) * (k.Wx - k.X) + (k.Wy - k.Y) * (k.Wy - k.Y));
        }

        static double Dist(double ax, double ay, double bx, double by)
        {
            double dx = ax - bx, dy = ay - by;
            return Math.Sqrt(dx * dx + dy * dy);
        }

        void MoveCreep(Creep k, double dt)
        {
            double sp = k.Speed;
            if (Time < k.StunUntil) return;
            if (Time < k.SlowUntil) sp *= 1 - k.SlowF;
            double step = sp * dt;
            while (step > 0)
            {
                double dx = k.Wx - k.X, dy = k.Wy - k.Y;
                double d = Math.Sqrt(dx * dx + dy * dy);
                if (d > step)
                {
                    k.X += dx / d * step; k.Y += dy / d * step;
                    break;
                }
                k.X = k.Wx; k.Y = k.Wy; step -= d;
                if (k.Flying || k.Exiting) { Leak(k); return; }
                k.Cell = k.Wcell;
                int[] f = k.Target == Lane.R ? FieldR : FieldB;
                if (f[k.Cell] == 0)
                {
                    k.Exiting = true; k.Wcell = -1;
                    if (k.Target == Lane.R) k.Wx = W + Cell; else k.Wy = H + Cell;
                    continue;
                }
                int n = Board.NextStep(f, k.Cell % C, k.Cell / C);
                if (n < 0) break; // ne doit jamais arriver : le placement garde un chemin ouvert
                k.Wcell = n;
                k.Wx = (n % C + 0.5) * Cell;
                k.Wy = (n / C + 0.5) * Cell;
            }
        }

        void Leak(Creep k)
        {
            k.Dead = true;
            int loss = k.Type == CreepType.Boss ? 2 : 1;
            Lives = Math.Max(0, Lives - loss);
            Floats.Add(new FloatText
            {
                X = Math.Min(W - 20, Math.Max(20, k.X)), Y = Math.Min(H - 10, Math.Max(14, k.Y)),
                Text = "-" + loss + " vie", Color = "#eb3751", Life = 1.2, Max = 1.2,
            });
            if (Lives <= 0) EndGame(false);
        }

        void Hurt(Creep k, double dmg)
        {
            if (k.Dead) return;
            k.Hp -= dmg;
            if (k.Hp > 0) return;
            k.Dead = true;
            int g = (int)Math.Max(1, Math.Round(k.Bounty, MidpointRounding.AwayFromZero));
            Gold += g;
            Score += g * 10;
            Kills++;
            CreepDef cd = GameData.Creeps[k.Type];
            Floats.Add(new FloatText { X = k.X, Y = k.Y - 6, Text = "+" + g, Color = "#fac515", Life = 0.8, Max = 0.8 });
            Fx.Add(new Fx { Kind = FxKind.Pop, X = k.X, Y = k.Y, Color = cd.Color, Life = 0.35, MaxLife = 0.35, R = k.R });
            if (cd.Splits)
                for (int i = 0; i < 2; i++) MakeCreep(CreepType.Mini, k.MaxHp * 0.3, k.Bounty * 0.5, k.Target, k);
        }

        // ---------- Tir ----------
        static bool CanHit(TowerDef d, Creep k) { return k.Flying ? d.Air : d.Ground; }

        bool Fire(Tower t)
        {
            TowerDef d = GameData.Towers[t.Type];
            Stats st = TowerStats(t);
            var inRange = new List<Creep>();
            foreach (Creep k in Creeps)
                if (!k.Dead && CanHit(d, k) && Dist(k.X, k.Y, t.X, t.Y) <= st.Range + k.R) inRange.Add(k);
            if (inRange.Count == 0) return false;
            if (d.Shot == ShotKind.Pulse)
            {
                Fx.Add(new Fx { Kind = FxKind.Ring, X = t.X, Y = t.Y, R = 8, Max = st.Range, Color = d.Color, Life = 0.35, MaxLife = 0.35 });
                foreach (Creep k in inRange)
                {
                    Hurt(k, st.Dmg);
                    if (!GameData.Creeps[k.Type].Immune && Rand() < d.Stun) k.StunUntil = Time + 0.6;
                }
                return true;
            }
            // Tri stable par distance restante (Array.Sort de .NET n'est pas stable).
            var order = new List<KeyValuePair<double, Creep>>();
            foreach (Creep k in inRange) order.Add(new KeyValuePair<double, Creep>(Remaining(k), k));
            for (int i = 1; i < order.Count; i++)
            {
                var cur = order[i];
                int j = i - 1;
                while (j >= 0 && order[j].Key > cur.Key) { order[j + 1] = order[j]; j--; }
                order[j + 1] = cur;
            }
            int n = d.Multi > 0 ? Math.Min(d.Multi, order.Count) : 1;
            Creep first = order[0].Value;
            t.Angle = Math.Atan2(first.Y - t.Y, first.X - t.X);
            for (int i = 0; i < n; i++)
            {
                Creep k = order[i].Value;
                Shots.Add(new Shot
                {
                    Kind = d.Shot, X = t.X + Math.Cos(t.Angle) * 8, Y = t.Y + Math.Sin(t.Angle) * 8,
                    Target = k, Tx = k.X, Ty = k.Y, Speed = d.Speed, Dmg = st.Dmg, Splash = d.Splash,
                    Slow = st.Slow, Color = d.Accent, Wob = Rand() * 6,
                });
            }
            return true;
        }

        void UpdateShots(double dt)
        {
            // Hurt() peut ajouter des ennemis mais jamais de tirs : on peut itérer sur la liste.
            foreach (Shot s in Shots)
            {
                if (s.Target != null && !s.Target.Dead) { s.Tx = s.Target.X; s.Ty = s.Target.Y; }
                double dx = s.Tx - s.X, dy = s.Ty - s.Y;
                double d = Math.Sqrt(dx * dx + dy * dy);
                double step = s.Speed * dt;
                if (d <= step + 2)
                {
                    s.Done = true;
                    if (s.Splash > 0)
                    {
                        Fx.Add(new Fx { Kind = FxKind.Ring, X = s.Tx, Y = s.Ty, R = 4, Max = s.Splash, Color = "#4ea1ff", Life = 0.3, MaxLife = 0.3 });
                        foreach (Creep k in Creeps.ToArray())
                            if (!k.Dead && !k.Flying && Dist(k.X, k.Y, s.Tx, s.Ty) <= s.Splash + k.R) Hurt(k, s.Dmg);
                    }
                    else if (s.Target != null && !s.Target.Dead)
                    {
                        Creep k = s.Target;
                        if (s.Slow > 0 && !GameData.Creeps[k.Type].Immune)
                        {
                            k.SlowF = Math.Max(s.Slow, Time < k.SlowUntil ? k.SlowF : 0);
                            k.SlowUntil = Time + 2;
                        }
                        Hurt(k, s.Dmg);
                    }
                    continue;
                }
                s.X += dx / d * step; s.Y += dy / d * step;
            }
            Shots.RemoveAll(s => s.Done);
        }

        // ---------- Boucle ----------
        /// <summary>Avance la simulation de dt secondes (appeler avec Step).</summary>
        public void Update(double dt)
        {
            if (Over || Paused) return;
            Time += dt;

            if (Started && Wave < Grid.TotalWaves)
            {
                Timer -= dt;
                if (Timer <= 0) { Timer = 0; SendWave(); }
            }

            foreach (Spawner sp in Spawners)
            {
                sp.T -= dt;
                while (sp.Left > 0 && sp.T <= 0)
                {
                    Spawn(sp.Def, sp.K++);
                    sp.Left--;
                    sp.T += GameData.Creeps[sp.Def.Type].Gap;
                }
            }
            Spawners.RemoveAll(sp => sp.Left <= 0);

            foreach (Creep k in Creeps.ToArray()) if (!k.Dead) MoveCreep(k, dt);
            if (Over) return;

            foreach (Tower t in Towers)
            {
                t.Cool -= dt;
                if (t.Cool <= 0) t.Cool = Fire(t) ? TowerStats(t).Rate : 0;
            }
            UpdateShots(dt);
            Creeps.RemoveAll(k => k.Dead);

            foreach (Fx f in Fx) f.Life -= dt;
            Fx.RemoveAll(f => f.Life <= 0);
            foreach (FloatText f in Floats) { f.Life -= dt; f.Y -= 18 * dt; }
            Floats.RemoveAll(f => f.Life <= 0);

            if (Wave >= Grid.TotalWaves && Spawners.Count == 0 && Creeps.Count == 0) EndGame(true);
        }

        void EndGame(bool won)
        {
            if (Over) return;
            Over = true;
            Won = won;
            if (won) Score += Lives * 100;
            string key = "dd-best-" + GameData.ModeKey(Mode);
            int best = store.Get(key);
            Best = Math.Max(best, Score);
            NewBest = Score > best;
            store.Set(key, Best);
            NotifyChanged();
            var h = Ended; if (h != null) h();
        }
    }
}
