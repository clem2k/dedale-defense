using System.Collections.Generic;

namespace DedaleDefense.Core
{
    public sealed class Tower
    {
        public int Id, Type, Cx, Cy, Level, Spent, BuiltWave;
        public double X, Y, Cool, Angle;
        public bool BuiltStarted;
    }

    public sealed class Creep
    {
        public int Id;
        public CreepType Type;
        public double X, Y, Hp, MaxHp, Bounty, Speed, R, SlowUntil, SlowF, StunUntil, Wob, Wx, Wy;
        public Lane Target;
        public bool Flying, Exiting, Dead;
        public int Cell = -1, Wcell;
    }

    public sealed class Shot
    {
        public ShotKind Kind;
        public double X, Y, Tx, Ty, Speed, Dmg, Splash, Slow, Wob;
        public Creep Target;
        public bool Done;
        public string Color;
    }

    public sealed class Spawner
    {
        public WaveDef Def; public int Left, K; public double T;
    }

    public enum FxKind { Ring, Pop }

    public sealed class Fx
    {
        public FxKind Kind;
        public double X, Y, R, Max, Life, MaxLife;
        public string Color;
    }

    public sealed class FloatText
    {
        public double X, Y, Life, Max;
        public string Text, Color;
    }

    public enum PlaceResult { Ok, Out, Taken, Creep, Block, NoGold, Over }

    /// <summary>Stockage du record par mode (PlayerPrefs côté Unity, mémoire dans les tests).</summary>
    public interface IBestStore
    {
        int Get(string key);
        void Set(string key, int value);
    }

    public sealed class MemoryBestStore : IBestStore
    {
        readonly Dictionary<string, int> d = new Dictionary<string, int>();
        public int Get(string key) { int v; return d.TryGetValue(key, out v) ? v : 0; }
        public void Set(string key, int value) { d[key] = value; }
    }
}
