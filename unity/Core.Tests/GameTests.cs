using System;
using System.Linq;
using DedaleDefense.Core;
using Xunit;

public class DataTests
{
    [Fact]
    public void Modes_MatchUserGuide()
    {
        Assert.Equal(30, GameData.Modes[ModeId.Easy].Lives);
        Assert.Equal(60, GameData.Modes[ModeId.Normal].Gold);
        Assert.Equal(1.35, GameData.Modes[ModeId.Hard].Hp);
    }

    [Fact]
    public void Wave1_IsWalkers_AndHpFormula()
    {
        WaveDef w = GameData.Wave(1);
        Assert.Equal(CreepType.Walker, w.Type);
        Assert.Equal(16.0, w.Hp, 6);                       // 12*1.115^0 + 4
        Assert.Equal(2.0, w.Bounty, 6);                    // 1 * (2 + 0)
        Assert.Equal(CreepType.Boss, GameData.Wave(10).Type);
        Assert.Equal(25.0 * (2 + 10 / 6), GameData.Wave(10).Bounty, 6);
        Assert.Equal(12 * Math.Pow(1.115, 49) + 200, GameData.Wave(50).Hp, 6);
    }
}

public class PlacementTests
{
    static Game NewGame(ModeId m = ModeId.Normal) { return new Game(m, 1, new MemoryBestStore()); }

    [Fact]
    public void EmptyBoard_HasPathsOnBothLanes()
    {
        var g = NewGame();
        Assert.All(Board.EntryL, i => Assert.NotEqual(Board.Inf, g.FieldR[i]));
        Assert.All(Board.EntryT, i => Assert.NotEqual(Board.Inf, g.FieldB[i]));
        Assert.Equal(2, g.Path.Count);
        Assert.True(g.Path[0].Count > 20);
    }

    [Fact]
    public void Place_DeductsGold_AndOccupiesFourCells()
    {
        var g = NewGame();
        Assert.Equal(PlaceResult.Ok, g.TryPlace(0, 5, 5));
        Assert.Equal(55, g.Gold);
        Assert.All(Game.CellsOf(5, 5), c => Assert.NotNull(g.Board.TowerAt[c]));
        Assert.Equal(PlaceResult.Taken, g.PlaceCheck(5, 5));
        Assert.Equal(PlaceResult.Taken, g.PlaceCheck(4, 4));
    }

    [Fact]
    public void Place_OutsideBoardAndWithoutGold_Refused()
    {
        var g = NewGame();
        Assert.Equal(PlaceResult.Out, g.PlaceCheck(0, 5));
        Assert.Equal(PlaceResult.Out, g.PlaceCheck(Grid.Cols - 2, 5));
        g.Gold = 4;
        Assert.Equal(PlaceResult.NoGold, g.TryPlace(0, 5, 5));
        Assert.Empty(g.Towers);
    }

    [Fact]
    public void NeverClosesTheLastPassage()
    {
        var g = NewGame();
        g.Gold = 100000;
        // Remplit tout le plateau tour par tour : à chaque pose, un chemin doit rester ouvert.
        for (int cy = 1; cy <= Grid.Rows - 3; cy += 2)
            for (int cx = 1; cx <= Grid.Cols - 3; cx += 2)
            {
                g.TryPlace(0, cx, cy);
                Assert.All(Board.EntryL, i => Assert.NotEqual(Board.Inf, g.FieldR[i]));
                Assert.All(Board.EntryT, i => Assert.NotEqual(Board.Inf, g.FieldB[i]));
            }
        Assert.True(g.Towers.Count > 20);
        // Au moins une pose a dû être refusée pour cause de blocage.
        int total = ((Grid.Rows - 3) / 2 + 1) * ((Grid.Cols - 3) / 2 + 1);
        Assert.True(g.Towers.Count < total);
    }

    [Fact]
    public void Sell_RefundsFully_BeforeFirstWave_ThenSeventyFivePercent()
    {
        var g = NewGame();
        g.TryPlace(2, 5, 5);                    // Aiguille 25
        Tower t = g.Towers[0];
        Assert.Equal(25, Game.SellValue(t));
        g.SendWave();
        g.TryPlace(2, 9, 9);
        Tower t2 = g.Towers[1];
        Assert.Equal(18, Game.SellValue(t2));   // floor(25*0.75)
        int gold = g.Gold;
        Assert.True(g.Sell(t2));
        Assert.Equal(gold + 18, g.Gold);
        Assert.Null(g.Board.TowerAt[Game.CellsOf(9, 9)[0]]);
    }

    [Fact]
    public void Upgrade_CostsGold_AndRaisesStats()
    {
        var g = NewGame();
        g.TryPlace(0, 5, 5);                    // Caillou
        Tower t = g.Towers[0];
        double r0 = Game.TowerStats(t).Range;
        Assert.True(g.Upgrade(t));
        Assert.Equal(1, t.Level);
        Assert.Equal(50, g.Gold);               // 60 - 5 - 5
        Assert.Equal(10, Game.TowerStats(t).Dmg);
        Assert.True(Game.TowerStats(t).Range > r0);
        for (int i = 0; i < 3; i++) { g.Gold = 1000; g.Upgrade(t); }
        Assert.Equal(4, t.Level);
        Assert.False(g.Upgrade(t));             // niveau max
    }
}

public class SimulationTests
{
    static void Run(Game g, double seconds)
    {
        for (double t = 0; t < seconds && !g.Over; t += Game.Step) g.Update(Game.Step);
    }

    [Fact]
    public void Wave1_Spawns10Walkers_WhoLeakAndCostLives()
    {
        var g = new Game(ModeId.Normal, 42, new MemoryBestStore());
        g.SendWave();
        g.Timer = 1e9;                          // pas de 2e vague automatique
        Run(g, 3);
        Assert.True(g.Creeps.Count > 0);
        Run(g, 60);
        Assert.Empty(g.Creeps);
        Assert.Equal(10, 20 - g.Lives);
    }

    [Fact]
    public void Towers_KillEnemies_AndGiveGoldAndScore()
    {
        var g = new Game(ModeId.Easy, 7, new MemoryBestStore());
        g.Gold = 200;
        for (int i = 0; i < 6; i++) g.TryPlace(0, 3 + i * 2, 8);
        g.SendWave();
        Run(g, 40);
        Assert.True(g.Kills > 0);
        Assert.True(g.Score >= g.Kills * 10);
    }

    [Fact]
    public void EarlySend_GivesBonus()
    {
        var g = new Game(ModeId.Normal, 1, new MemoryBestStore());
        g.SendWave();                           // timer = 24
        Run(g, 4);
        int s = g.Score;
        g.SendWave();
        Assert.True(g.Score - s >= 20 * 2);     // ceil(~20) * (wave 1 + 1)
        Assert.Equal(2, g.Wave);
    }

    [Fact]
    public void Defeat_StoresBest_AndEndsOnce()
    {
        var store = new MemoryBestStore();
        var g = new Game(ModeId.Hard, 3, store);
        int ended = 0;
        g.Ended += () => ended++;
        g.Score = 123;
        for (int i = 0; i < 8 && !g.Over; i++) g.SendWave();
        Run(g, 600);
        Assert.True(g.Over);
        Assert.False(g.Won);
        Assert.Equal(1, ended);
        Assert.Equal(g.Best, store.Get("dd-best-hard"));
        Assert.True(g.NewBest);
    }

    [Fact]
    public void Simulation_IsDeterministicForAGivenSeed()
    {
        Func<int, string> run = seed =>
        {
            var g = new Game(ModeId.Normal, seed, new MemoryBestStore());
            g.Gold = 300;
            g.TryPlace(1, 10, 6); g.TryPlace(4, 12, 10); g.TryPlace(5, 14, 8);
            for (int i = 0; i < 4; i++) g.SendWave();
            for (int i = 0; i < 60 * 90; i++) g.Update(Game.Step);
            return g.Score + "/" + g.Gold + "/" + g.Lives + "/" + g.Kills;
        };
        Assert.Equal(run(5), run(5));
    }

    [Fact]
    public void FullGame_50Waves_CanBeWon_WithGodMode()
    {
        var g = new Game(ModeId.Easy, 11, new MemoryBestStore());
        g.Lives = 1000000;
        g.Gold = 100000000;
        // Quelques tours de chaque type, au maximum.
        int x = 3;
        for (int type = 0; type < 6; type++)
        {
            g.TryPlace(type, x, 7); g.TryPlace(type, x, 12);
            x += 3;
        }
        foreach (Tower t in g.Towers.ToArray()) for (int i = 0; i < 4; i++) g.Upgrade(t);
        g.SendWave();
        for (int i = 0; i < 60 * 3000 && !g.Over; i++)
        {
            if (g.Timer > 2) g.SendWave();
            g.Update(Game.Step);
        }
        Assert.True(g.Over);
        Assert.Equal(Grid.TotalWaves, g.Wave);
        Assert.True(g.Won);
    }
}
