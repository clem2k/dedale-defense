using System;
using System.Collections.Generic;

namespace DedaleDefense.Core
{
    /// <summary>Constantes du plateau (1 unité Core = 1 pixel de la version web ; 1 cellule = 20).</summary>
    public static class Grid
    {
        public const int Cell = 20;
        public const int Cols = 28;
        public const int Rows = 22;
        public const int Width = Cols * Cell;
        public const int Height = Rows * Cell;
        public static readonly int[] OpenRows = { 9, 10, 11, 12 };   // lignes ouvertes à gauche et à droite
        public static readonly int[] OpenCols = { 12, 13, 14, 15 };  // colonnes ouvertes en haut et en bas
        public const int TotalWaves = 50;
    }

    public enum ModeId { Easy, Normal, Hard }

    public enum Lane { R, B } // R : de gauche à droite, B : de haut en bas

    public enum CreepType { Walker, Runner, Swarm, Flyer, Armored, Splitter, Mini, Boss }

    public enum CreepShape { Circle, Tri, Wing, Square, Diamond, Boss }

    public enum ShotKind { Pellet, Shell, Dart, Missile, Ice, Pulse }

    public sealed class ModeDef
    {
        public string Name; public double Hp; public int Lives; public int Gold; public double Gap;
    }

    public sealed class TowerDef
    {
        public string Id, Name, Key, Color, Accent, Desc;
        public int Cost;
        public int[] Ups, Dmg;
        public double Range, Rate;
        public bool Ground, Air;
        public ShotKind Shot;
        public double Speed, Splash, Stun;
        public int Multi;
        public double[] Slow;
    }

    public sealed class CreepDef
    {
        public string Name, Color, Desc;
        public double Hp, Speed, Gap, Bounty, R;
        public int Count;
        public CreepShape Shape;
        public bool Flying, Immune, Splits;
    }

    public sealed class WaveDef
    {
        public int N; public CreepType Type; public double Hp; public int Count; public double Bounty;
    }

    public static class GameData
    {
        public static readonly Dictionary<ModeId, ModeDef> Modes = new Dictionary<ModeId, ModeDef>
        {
            { ModeId.Easy,   new ModeDef { Name = "Facile",    Hp = 0.75, Lives = 30, Gold = 70, Gap = 28 } },
            { ModeId.Normal, new ModeDef { Name = "Normal",    Hp = 1,    Lives = 20, Gold = 60, Gap = 24 } },
            { ModeId.Hard,   new ModeDef { Name = "Difficile", Hp = 1.35, Lives = 10, Gold = 50, Gap = 20 } },
        };

        public static string ModeKey(ModeId m) { return m.ToString().ToLowerInvariant(); }

        public static readonly TowerDef[] Towers =
        {
            new TowerDef { Id = "stone", Name = "Caillou", Key = "1", Color = "#9aa4b2", Accent = "#e4e7ec",
                Desc = "Tir rapide et bon marché. Touche le sol et les airs.",
                Cost = 5, Ups = new[] { 5, 10, 20, 40 }, Dmg = new[] { 5, 10, 20, 40, 80 }, Range = 70, Rate = 0.5,
                Ground = true, Air = true, Shot = ShotKind.Pellet, Speed = 340 },
            new TowerDef { Id = "burst", Name = "Éclat", Key = "2", Color = "#4ea1ff", Accent = "#b9dcff",
                Desc = "Obus qui explose et blesse les ennemis proches. Sol uniquement.",
                Cost = 15, Ups = new[] { 15, 30, 60, 120 }, Dmg = new[] { 8, 16, 32, 64, 128 }, Range = 62, Rate = 0.9,
                Ground = true, Air = false, Shot = ShotKind.Shell, Speed = 240, Splash = 30 },
            new TowerDef { Id = "needle", Name = "Aiguille", Key = "3", Color = "#c084fc", Accent = "#ead5ff",
                Desc = "Longue portée et gros dégâts, mais lente. Sol et airs.",
                Cost = 25, Ups = new[] { 25, 50, 100, 200 }, Dmg = new[] { 25, 50, 100, 200, 400 }, Range = 130, Rate = 1.4,
                Ground = true, Air = true, Shot = ShotKind.Dart, Speed = 520 },
            new TowerDef { Id = "volley", Name = "Volée", Key = "4", Color = "#f97066", Accent = "#ffd1cc",
                Desc = "Lance jusqu'à quatre missiles à la fois. Airs uniquement.",
                Cost = 30, Ups = new[] { 30, 60, 120, 240 }, Dmg = new[] { 12, 24, 48, 96, 192 }, Range = 105, Rate = 0.85,
                Ground = false, Air = true, Shot = ShotKind.Missile, Speed = 300, Multi = 4 },
            new TowerDef { Id = "frost", Name = "Givre", Key = "5", Color = "#5fe0e0", Accent = "#d6fbfb",
                Desc = "Ralentit sa cible pendant deux secondes. Sol et airs.",
                Cost = 20, Ups = new[] { 20, 40, 80, 160 }, Dmg = new[] { 2, 4, 8, 16, 32 }, Range = 72, Rate = 0.6,
                Ground = true, Air = true, Shot = ShotKind.Ice, Speed = 320, Slow = new[] { 0.35, 0.4, 0.45, 0.5, 0.6 } },
            new TowerDef { Id = "pulse", Name = "Onde", Key = "6", Color = "#fdb022", Accent = "#fff0c2",
                Desc = "Onde de choc autour d'elle, peut étourdir. Sol uniquement.",
                Cost = 40, Ups = new[] { 40, 80, 160, 320 }, Dmg = new[] { 20, 40, 80, 160, 320 }, Range = 46, Rate = 1.2,
                Ground = true, Air = false, Shot = ShotKind.Pulse, Stun = 0.15 },
        };

        public static readonly Dictionary<CreepType, CreepDef> Creeps = new Dictionary<CreepType, CreepDef>
        {
            { CreepType.Walker,   new CreepDef { Name = "Marcheurs", Hp = 1,    Speed = 40, Count = 10, Gap = 0.65, Bounty = 1,   Color = "#a6ef67", R = 6,  Shape = CreepShape.Circle,  Desc = "Ennemis ordinaires." } },
            { CreepType.Runner,   new CreepDef { Name = "Coureurs",  Hp = 0.7,  Speed = 78, Count = 10, Gap = 0.55, Bounty = 1,   Color = "#fef08a", R = 5,  Shape = CreepShape.Tri,     Desc = "Très rapides." } },
            { CreepType.Swarm,    new CreepDef { Name = "Essaim",    Hp = 0.4,  Speed = 46, Count = 22, Gap = 0.3,  Bounty = 0.5, Color = "#fda29b", R = 4,  Shape = CreepShape.Circle,  Desc = "Nombreux mais fragiles." } },
            { CreepType.Flyer,    new CreepDef { Name = "Volants",   Hp = 0.85, Speed = 46, Count = 8,  Gap = 0.9,  Bounty = 1.5, Color = "#7cd4fd", R = 6,  Shape = CreepShape.Wing,    Flying = true, Desc = "Survolent les tours." } },
            { CreepType.Armored,  new CreepDef { Name = "Blindés",   Hp = 1.5,  Speed = 34, Count = 8,  Gap = 0.9,  Bounty = 1.5, Color = "#bdb4fe", R = 7,  Shape = CreepShape.Square,  Immune = true, Desc = "Insensibles au givre et au choc." } },
            { CreepType.Splitter, new CreepDef { Name = "Scindeurs", Hp = 1.05, Speed = 40, Count = 8,  Gap = 0.85, Bounty = 1,   Color = "#f9a8d4", R = 7,  Shape = CreepShape.Diamond, Splits = true, Desc = "Se divisent en deux en mourant." } },
            { CreepType.Mini,     new CreepDef { Name = "Éclats",    Hp = 0.3,  Speed = 52, Count = 0,  Gap = 0,    Bounty = 0.5, Color = "#f9a8d4", R = 4,  Shape = CreepShape.Diamond } },
            { CreepType.Boss,     new CreepDef { Name = "Colosse",   Hp = 14,   Speed = 28, Count = 1,  Gap = 1,    Bounty = 25,  Color = "#f04438", R = 11, Shape = CreepShape.Boss,    Desc = "Un seul, mais énorme." } },
        };

        public static readonly CreepType[] Cycle =
        {
            CreepType.Walker, CreepType.Runner, CreepType.Walker, CreepType.Flyer, CreepType.Swarm,
            CreepType.Armored, CreepType.Walker, CreepType.Splitter, CreepType.Runner, CreepType.Boss,
        };

        /// <summary>Définition de la vague n (1-based).</summary>
        public static WaveDef Wave(int n)
        {
            CreepType type = n == 1 ? CreepType.Walker : Cycle[(n - 1) % Cycle.Length];
            double baseHp = 12 * Math.Pow(1.115, n - 1) + 4 * n;
            CreepDef c = Creeps[type];
            return new WaveDef { N = n, Type = type, Hp = baseHp, Count = c.Count, Bounty = c.Bounty * (2 + n / 6) };
        }
    }
}
