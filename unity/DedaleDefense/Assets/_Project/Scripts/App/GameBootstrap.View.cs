using System;
using System.Text;
using DedaleDefense.Core;
using UnityEngine;

namespace DedaleDefense.App
{
    public sealed partial class GameBootstrap
    {
        static readonly Color Bg = new Color(0.07f, 0.09f, 0.13f);
        static readonly Color Panel = new Color(0.11f, 0.16f, 0.23f);
        static readonly Color Text = new Color(0.9f, 0.93f, 0.97f);
        static readonly Color Dim = new Color(0.6f, 0.67f, 0.76f);
        static readonly Color Gold = new Color(0.98f, 0.77f, 0.08f);
        static readonly Color Danger = new Color(0.92f, 0.21f, 0.32f);
        static readonly Color Ok = new Color(0.24f, 0.8f, 0.5f);

        void OnGUI()
        {
            Event e = Event.current;
            if (e.type == EventType.Repaint) Gfx.Rect(0, 0, UnityEngine.Screen.width, UnityEngine.Screen.height, Bg);

            float s = Mathf.Min(UnityEngine.Screen.width / VW, UnityEngine.Screen.height / VH);
            float ox = (UnityEngine.Screen.width - VW * s) / 2f, oy = (UnityEngine.Screen.height - VH * s) / 2f;
            GUI.matrix = Matrix4x4.TRS(new Vector3(ox, oy, 0), Quaternion.identity, new Vector3(s, s, 1));

            if (e.type == EventType.KeyDown) OnKey(e);

            if (screen == Screen_.Menu || game == null) { DrawMenu(e); return; }

            UpdateHover(e);
            DrawHud();
            DrawBoard();
            DrawSidePanel(e);
            DrawWavePreview();
            if (game.Over) DrawEnd(e);
            else if (game.Paused) Overlay("Pause", "Espace pour reprendre");
        }

        // ---------- Entrées souris ----------
        void UpdateHover(Event e)
        {
            Vector2 p = e.mousePosition - new Vector2(BX, BY);
            bool inside = p.x >= 0 && p.y >= 0 && p.x < Grid.Width && p.y < Grid.Height;
            int cx = Mathf.RoundToInt(p.x / Grid.Cell) - 1, cy = Mathf.RoundToInt(p.y / Grid.Cell) - 1;
            if (inside != hoverBoard || cx != hoverCx || cy != hoverCy || e.type == EventType.MouseDown)
            {
                hoverBoard = inside; hoverCx = cx; hoverCy = cy;
                RefreshHover();
            }
            if (e.type == EventType.MouseDown)
            {
                if (e.button == 1) { build = -1; selected = null; e.Use(); }
                else if (e.button == 0 && inside && !game.Over && !game.Paused) { OnBoardClick(p, e.shift); e.Use(); }
            }
        }

        bool Btn(Rect r, string label, Event e, bool enabled = true, Color? tint = null, int size = 13)
        {
            bool hot = enabled && r.Contains(e.mousePosition);
            if (e.type == EventType.Repaint)
            {
                Color c = tint ?? new Color(0.18f, 0.26f, 0.38f);
                if (!enabled) c = new Color(0.14f, 0.18f, 0.24f);
                else if (hot) c = Color.Lerp(c, Color.white, 0.18f);
                Gfx.Rect(r.x, r.y, r.width, r.height, c);
                Gfx.Label(r.x + 6, r.y, r.width - 12, r.height, label, size, enabled ? Text : Dim, TextAnchor.MiddleCenter, true);
            }
            if (e.type == EventType.MouseDown && e.button == 0 && hot) { e.Use(); return true; }
            return false;
        }

        // ---------- HUD ----------
        void DrawHud()
        {
            Gfx.Rect(0, 0, VW, 54, Panel);
            Gfx.Label(12, 4, 120, 46, "OR  " + game.Gold, 20, Gold, TextAnchor.MiddleLeft, true);
            Gfx.Label(140, 4, 130, 46, "VIES  " + game.Lives, 20, game.Lives <= 5 ? Danger : Ok, TextAnchor.MiddleLeft, true);
            Gfx.Label(280, 4, 190, 46, "VAGUE  " + game.Wave + " / " + Grid.TotalWaves, 20, Text, TextAnchor.MiddleLeft, true);
            string next = game.Started && game.Wave < Grid.TotalWaves ? "  (suivante " + Mathf.CeilToInt((float)game.Timer) + " s)" : "";
            Gfx.Label(470, 4, 140, 46, "SCORE  " + game.Score, 16, Text, TextAnchor.MiddleLeft, true);
            Gfx.Label(620, 4, 205, 46, game.ModeDef.Name + "  record " + store.Get("dd-best-" + GameData.ModeKey(game.Mode)) + next, 11, Dim, TextAnchor.MiddleRight);
        }

        // ---------- Plateau ----------
        static float X(double v) { return BX + (float)v; }
        static float Y(double v) { return BY + (float)v; }

        void DrawBoard()
        {
            if (Event.current.type != EventType.Repaint) return;
            const int C = Grid.Cell;
            Gfx.Rect(BX, BY, Grid.Width, Grid.Height, new Color(0.08f, 0.13f, 0.2f));
            for (int x = 0; x <= Grid.Cols; x++)
                Gfx.Rect(BX + x * C, BY, 1, Grid.Height, new Color(0.47f, 0.63f, 0.86f, x % 2 != 0 ? 0.06f : 0.13f));
            for (int y = 0; y <= Grid.Rows; y++)
                Gfx.Rect(BX, BY + y * C, Grid.Width, 1, new Color(0.47f, 0.63f, 0.86f, y % 2 != 0 ? 0.06f : 0.13f));
            for (int i = 0; i < Board.Wall.Length; i++)
                if (Board.Wall[i]) Gfx.Rect(BX + (i % Grid.Cols) * C, BY + (i / Grid.Cols) * C, C, C, new Color(0.04f, 0.08f, 0.13f));

            var green = new Color(0.24f, 0.8f, 0.5f, 0.16f);
            var red = new Color(0.92f, 0.21f, 0.32f, 0.16f);
            Gfx.Rect(BX, BY + Grid.OpenRows[0] * C, C, Grid.OpenRows.Length * C, green);
            Gfx.Rect(BX + Grid.OpenCols[0] * C, BY, Grid.OpenCols.Length * C, C, green);
            Gfx.Rect(BX + Grid.Width - C, BY + Grid.OpenRows[0] * C, C, Grid.OpenRows.Length * C, red);
            Gfx.Rect(BX + Grid.OpenCols[0] * C, BY + Grid.Height - C, Grid.OpenCols.Length * C, C, red);

            // Aperçu des deux routes
            foreach (var route in game.Path)
                for (int i = 0; i < route.Count; i += 2)
                    Gfx.Circle(BX + (route[i] % Grid.Cols + 0.5f) * C, BY + (route[i] / Grid.Cols + 0.5f) * C, 1.6f, new Color(1, 1, 1, 0.16f));

            foreach (Tower t in game.Towers) DrawTower(t.Type, t.Level, X(t.X), Y(t.Y), (float)t.Angle, 1f);
            if (selected != null)
            {
                Core.Game.Stats st = Core.Game.TowerStats(selected);
                Gfx.Ring(X(selected.X), Y(selected.Y), (float)st.Range, new Color(1, 1, 1, 0.6f));
            }

            foreach (Creep k in game.Creeps) DrawCreep(k);
            foreach (Shot sh in game.Shots) DrawShot(sh);
            foreach (Fx f in game.Fx)
            {
                float p = 1f - (float)(f.Life / f.MaxLife);
                if (f.Kind == FxKind.Ring)
                    Gfx.Ring(X(f.X), Y(f.Y), Mathf.Lerp((float)f.R, (float)f.Max, p), Gfx.Hex(f.Color, 1f - p));
                else
                    Gfx.Circle(X(f.X), Y(f.Y), (float)f.R * (1f + p), Gfx.Hex(f.Color, 0.6f * (1f - p)));
            }

            // Fantôme de la tour à poser
            if (build >= 0 && hoverBoard && !game.Over)
            {
                TowerDef d = GameData.Towers[build];
                bool ok = hoverResult == PlaceResult.Ok && game.Gold >= d.Cost;
                float gx = BX + (hoverCx + 1) * C, gy = BY + (hoverCy + 1) * C;
                Gfx.Ring(gx, gy, (float)d.Range, new Color(1, 1, 1, 0.35f));
                Gfx.Rect(gx - C, gy - C, 2 * C, 2 * C, ok ? new Color(0.24f, 0.8f, 0.5f, 0.35f) : new Color(0.92f, 0.21f, 0.32f, 0.35f));
                DrawTower(build, 0, gx, gy, -Mathf.PI / 2f, 1f);
            }

            foreach (FloatText f in game.Floats)
            {
                float a = Mathf.Clamp01((float)(f.Life / f.Max) * 2f);
                var c = Gfx.Hex(f.Color, a);
                Gfx.Label(X(f.X) - 90, Y(f.Y) - 10, 180, 20, f.Text, 12, c, TextAnchor.MiddleCenter, true);
            }
        }

        static void DrawTower(int type, int level, float x, float y, float angle, float size)
        {
            TowerDef d = GameData.Towers[type];
            Color body = Gfx.Hex(d.Color), accent = Gfx.Hex(d.Accent);
            Gfx.Rect(x - 18 * size, y - 18 * size, 36 * size, 36 * size, new Color(0.15f, 0.21f, 0.3f));
            Gfx.Rect(x - 14 * size, y - 14 * size, 28 * size, 28 * size, new Color(0.1f, 0.15f, 0.22f));
            switch (d.Id)
            {
                case "stone": Gfx.Circle(x, y, 9 * size, body); break;
                case "burst": Gfx.Circle(x, y, 10 * size, body); Gfx.Circle(x, y, 5 * size, accent); break;
                case "needle": Gfx.Triangle(x, y, 12 * size, angle, body); break;
                case "volley": Gfx.RotRect(x, y, 16 * size, 16 * size, Mathf.PI / 4f, body); break;
                case "frost": Gfx.RotRect(x, y, 14 * size, 14 * size, 0, body); Gfx.RotRect(x, y, 14 * size, 14 * size, Mathf.PI / 4f, body); break;
                default: Gfx.Ring(x, y, 11 * size, body); Gfx.Circle(x, y, 5 * size, body); break;
            }
            if (d.Shot != ShotKind.Pulse)
                Gfx.RotRect(x + Mathf.Cos(angle) * 7 * size, y + Mathf.Sin(angle) * 7 * size, 12 * size, 3.5f * size, angle, accent);
            for (int i = 0; i < level; i++)
                Gfx.Circle(x - 12 * size + i * 6 * size, y + 14 * size, 1.8f * size, Gold);
        }

        void DrawCreep(Creep k)
        {
            CreepDef c = GameData.Creeps[k.Type];
            float x = X(k.X), y = Y(k.Y) + (k.Flying ? Mathf.Sin((float)game.Time * 6f + (float)k.Wob) * 1.5f : 0f);
            float r = (float)k.R;
            if (k.Flying) Gfx.Circle(x + 3, y + 7, r * 0.8f, new Color(0, 0, 0, 0.25f));
            bool slowed = game.Time < k.SlowUntil;
            Color col = Gfx.Hex(c.Color);
            if (slowed) Gfx.Circle(x, y, r + 2.5f, new Color(0.37f, 0.88f, 0.88f, 0.55f));
            float dir = k.Flying ? (k.Target == Lane.R ? 0f : Mathf.PI / 2f) : Mathf.Atan2((float)(k.Wy - k.Y), (float)(k.Wx - k.X));
            switch (c.Shape)
            {
                case CreepShape.Tri: Gfx.Triangle(x, y, r + 2, dir, col); break;
                case CreepShape.Wing: Gfx.Triangle(x, y, r + 3, dir, col); break;
                case CreepShape.Square: Gfx.RotRect(x, y, r * 2, r * 2, 0, col); break;
                case CreepShape.Diamond: Gfx.RotRect(x, y, r * 1.5f, r * 1.5f, Mathf.PI / 4f, col); break;
                case CreepShape.Boss:
                    Gfx.Circle(x, y, r, col);
                    Gfx.Ring(x, y, r + 2, Gfx.Hex("#ffd1cc", 0.8f));
                    Gfx.Circle(x, y, r * 0.45f, new Color(0.3f, 0.05f, 0.05f));
                    break;
                default: Gfx.Circle(x, y, r, col); break;
            }
            if (game.Time < k.StunUntil) Gfx.Label(x - 10, y - r - 16, 20, 12, "*", 14, Gfx.Hex("#fff0c2"), TextAnchor.MiddleCenter, true);
            if (k.Hp < k.MaxHp)
            {
                float w = Mathf.Max(12f, r * 2 + 2), p = Mathf.Max(0f, (float)(k.Hp / k.MaxHp));
                Gfx.Rect(x - w / 2, y - r - 6, w, 3, new Color(0, 0, 0, 0.6f));
                Gfx.Rect(x - w / 2, y - r - 6, w * p, 3, p > 0.5f ? Ok : p > 0.25f ? Gold : Danger);
            }
        }

        static void DrawShot(Shot s)
        {
            Color c = Gfx.Hex(s.Color);
            float x = X(s.X), y = Y(s.Y);
            float ang = Mathf.Atan2((float)(s.Ty - s.Y), (float)(s.Tx - s.X));
            switch (s.Kind)
            {
                case ShotKind.Shell: Gfx.Circle(x, y, 3.5f, c); break;
                case ShotKind.Dart: Gfx.RotRect(x, y, 10, 2, ang, c); break;
                case ShotKind.Missile: Gfx.RotRect(x, y, 7, 3, ang, c); Gfx.Circle(x - Mathf.Cos(ang) * 4, y - Mathf.Sin(ang) * 4, 1.5f, Gfx.Hex("#fdb022")); break;
                case ShotKind.Ice: Gfx.Circle(x, y, 3f, c); Gfx.Ring(x, y, 4.5f, new Color(c.r, c.g, c.b, 0.6f)); break;
                default: Gfx.Circle(x, y, 2.5f, c); break;
            }
        }

        // ---------- Panneau latéral ----------
        void DrawSidePanel(Event e)
        {
            Gfx.Rect(PX, 62, PW, 440, Panel);
            Gfx.Label(PX + 8, 66, PW - 16, 20, "TOURS", 12, Dim, TextAnchor.MiddleLeft, true);
            for (int i = 0; i < GameData.Towers.Length; i++)
            {
                TowerDef d = GameData.Towers[i];
                var r = new Rect(PX + 8, 88 + i * 36, PW - 16, 32);
                bool afford = game.Gold >= d.Cost;
                Color tint = build == i ? new Color(0.25f, 0.4f, 0.65f) : new Color(0.16f, 0.23f, 0.34f);
                if (Btn(r, "[" + d.Key + "] " + d.Name + "   " + d.Cost, e, true, tint) && !game.Over) SetBuild(i);
                if (e.type == EventType.Repaint)
                {
                    Gfx.Rect(r.x, r.y, 5, r.height, Gfx.Hex(d.Color));
                    if (!afford) Gfx.Rect(r.x, r.y, r.width, r.height, new Color(0, 0, 0, 0.35f));
                }
            }

            // Détail : tour à poser ou tour sélectionnée
            float iy = 308;
            Gfx.Rect(PX + 8, iy, PW - 16, 118, new Color(0.08f, 0.12f, 0.18f));
            if (selected != null)
            {
                TowerDef d = GameData.Towers[selected.Type];
                Core.Game.Stats st = Core.Game.TowerStats(selected);
                Gfx.Label(PX + 14, iy + 2, PW - 28, 20, d.Name + "  niv. " + (selected.Level + 1) + "/5", 14, Gfx.Hex(d.Color), TextAnchor.MiddleLeft, true);
                Gfx.Label(PX + 14, iy + 22, PW - 28, 36,
                    "Dégâts " + st.Dmg + "   Portée " + Mathf.RoundToInt((float)st.Range) + "\nCadence " + st.Rate.ToString("0.00") + " s", 11, Text);
                bool max = selected.Level >= d.Ups.Length;
                int cost = max ? 0 : d.Ups[selected.Level];
                if (Btn(new Rect(PX + 14, iy + 62, 100, 24), max ? "Maximum" : "Améliorer " + cost + " (U)", e, !max && game.Gold >= cost, null, 11))
                    game.Upgrade(selected);
                if (Btn(new Rect(PX + 120, iy + 62, 96, 24), "Vendre +" + Core.Game.SellValue(selected) + " (S)", e, true, new Color(0.45f, 0.2f, 0.2f), 11))
                { game.Sell(selected); selected = null; }
                Gfx.Label(PX + 14, iy + 90, PW - 28, 24, d.Desc, 10, Dim);
            }
            else if (build >= 0)
            {
                TowerDef d = GameData.Towers[build];
                Gfx.Label(PX + 14, iy + 2, PW - 28, 20, d.Name + "  (" + d.Cost + " or)", 14, Gfx.Hex(d.Color), TextAnchor.MiddleLeft, true);
                Gfx.Label(PX + 14, iy + 24, PW - 28, 90,
                    d.Desc + "\nDégâts " + d.Dmg[0] + "   Portée " + d.Range + "\nMaj : poser plusieurs tours", 11, Text);
            }
            else
            {
                Gfx.Label(PX + 14, iy + 4, PW - 28, 110,
                    "Choisissez une tour (1-6) puis cliquez sur le plateau. Cliquez une tour pour l'améliorer ou la vendre.\n\nUne tour ne peut jamais fermer le dernier passage.", 11, Dim);
            }

            bool canSend = !game.Over && game.Wave < Grid.TotalWaves;
            string sendLabel = game.Started ? "Vague suivante (N)" : "Lancer la vague 1 (N)";
            if (Btn(new Rect(PX + 8, 432, PW - 16, 30), sendLabel, e, canSend, new Color(0.15f, 0.5f, 0.35f))) game.SendWave();
            if (Btn(new Rect(PX + 8, 466, 110, 30), game.Paused ? "Reprendre" : "Pause (Esp.)", e, !game.Over)) TogglePause();
            if (Btn(new Rect(PX + 122, 466, 100, 30), "Vitesse x" + speed + " (F)", e)) ToggleSpeed();
        }

        void DrawWavePreview()
        {
            var sb = new StringBuilder("Prochaines vagues :  ");
            for (int n = game.Wave + 1; n <= Math.Min(Grid.TotalWaves, game.Wave + 4); n++)
            {
                WaveDef w = GameData.Wave(n);
                CreepDef c = GameData.Creeps[w.Type];
                sb.Append(n).Append(' ').Append(c.Name).Append(" x").Append(w.Count).Append(c.Flying ? " (air)" : "").Append("    ");
            }
            Gfx.Label(BX, BY + Grid.Height + 4, Grid.Width, 22, sb.ToString(), 11, Dim);
            Gfx.Label(PX + 8, 506, PW, 40, "Clic droit / Échap : annuler", 10, Dim);
        }

        // ---------- Écrans ----------
        void Overlay(string title, string sub)
        {
            Gfx.Rect(BX, BY, Grid.Width, Grid.Height, new Color(0, 0, 0, 0.55f));
            Gfx.Label(BX, BY + 170, Grid.Width, 50, title, 34, Text, TextAnchor.MiddleCenter, true);
            Gfx.Label(BX, BY + 220, Grid.Width, 30, sub, 14, Dim, TextAnchor.MiddleCenter);
        }

        void DrawEnd(Event e)
        {
            if (e.type == EventType.Repaint)
            {
                Overlay(game.Won ? "Victoire !" : "Défaite", "Score " + game.Score + "   Record " + game.Best + (game.NewBest ? "  (nouveau !)" : "") + "   Ennemis vaincus " + game.Kills);
            }
            if (Btn(new Rect(BX + 150, BY + 270, 120, 36), "Rejouer", e, true, new Color(0.15f, 0.5f, 0.35f))) NewGame(game.Mode);
            if (Btn(new Rect(BX + 290, BY + 270, 120, 36), "Menu", e)) { screen = Screen_.Menu; game = null; }
        }

        void DrawMenu(Event e)
        {
            Gfx.Label(0, 40, VW, 60, "DÉDALE DÉFENSE", 44, Text, TextAnchor.MiddleCenter, true);
            Gfx.Label(0, 100, VW, 30, "Vos tours forment le labyrinthe. Tenez 50 vagues.", 15, Dim, TextAnchor.MiddleCenter);
            ModeId[] modes = { ModeId.Easy, ModeId.Normal, ModeId.Hard };
            for (int i = 0; i < modes.Length; i++)
            {
                ModeDef m = GameData.Modes[modes[i]];
                var r = new Rect(VW / 2 - 150, 170 + i * 80, 300, 64);
                if (Btn(r, m.Name + "\n" + m.Lives + " vies - " + m.Gold + " or - PV x" + m.Hp, e, true, null, 15)) NewGame(modes[i]);
                Gfx.Label(r.x + r.width + 12, r.y, 200, r.height, "Record " + store.Get("dd-best-" + GameData.ModeKey(modes[i])), 13, Dim);
            }
            Gfx.Label(0, 430, VW, 100,
                "1-6 : choisir une tour    U : améliorer    S : vendre    Maj : poser plusieurs tours\nEspace : pause    N : vague suivante    F : vitesse    Clic droit / Échap : annuler",
                12, Dim, TextAnchor.UpperCenter);
        }
    }
}
