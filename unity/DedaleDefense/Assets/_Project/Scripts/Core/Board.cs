using System;
using System.Collections.Generic;

namespace DedaleDefense.Core
{
    /// <summary>Plateau : murs fixes, tours posées et champs de distance (Dijkstra 8 directions, sans coupe de coin).</summary>
    public sealed class Board
    {
        public const int Inf = int.MaxValue;
        const int C = Grid.Cols, R = Grid.Rows;
        static readonly int[,] Nb =
        {
            { 1, 0, 10 }, { -1, 0, 10 }, { 0, 1, 10 }, { 0, -1, 10 },
            { 1, 1, 14 }, { 1, -1, 14 }, { -1, 1, 14 }, { -1, -1, 14 },
        };

        public static readonly bool[] Wall = BuildWalls();
        public static readonly int[] ExitR = MapRows(y => y * C + C - 1);
        public static readonly int[] ExitB = MapCols(x => (R - 1) * C + x);
        public static readonly int[] EntryL = MapRows(y => y * C);
        public static readonly int[] EntryT = MapCols(x => x);

        public readonly Tower[] TowerAt = new Tower[C * R];

        static int[] MapRows(Func<int, int> f)
        {
            var a = new int[Grid.OpenRows.Length];
            for (int i = 0; i < a.Length; i++) a[i] = f(Grid.OpenRows[i]);
            return a;
        }

        static int[] MapCols(Func<int, int> f)
        {
            var a = new int[Grid.OpenCols.Length];
            for (int i = 0; i < a.Length; i++) a[i] = f(Grid.OpenCols[i]);
            return a;
        }

        static bool[] BuildWalls()
        {
            var w = new bool[C * R];
            for (int y = 0; y < R; y++)
                for (int x = 0; x < C; x++)
                {
                    bool edge = x == 0 || y == 0 || x == C - 1 || y == R - 1;
                    bool openLR = (x == 0 || x == C - 1) && Array.IndexOf(Grid.OpenRows, y) >= 0;
                    bool openTB = (y == 0 || y == R - 1) && Array.IndexOf(Grid.OpenCols, x) >= 0;
                    if (edge && !openLR && !openTB) w[y * C + x] = true;
                }
            return w;
        }

        public bool IsFree(int x, int y, HashSet<int> blocked = null)
        {
            if (x < 0 || y < 0 || x >= C || y >= R) return false;
            int i = y * C + x;
            if (Wall[i] || TowerAt[i] != null) return false;
            if (blocked != null && blocked.Contains(i)) return false;
            return true;
        }

        public int[] ComputeField(int[] targets, HashSet<int> blocked = null)
        {
            var dist = new int[C * R];
            for (int i = 0; i < dist.Length; i++) dist[i] = Inf;
            var heap = new MinHeap();
            foreach (int t in targets) { dist[t] = 0; heap.Push(0, t); }
            while (heap.Count > 0)
            {
                int d, i;
                heap.Pop(out d, out i);
                if (d > dist[i]) continue;
                int x = i % C, y = i / C;
                for (int n = 0; n < 8; n++)
                {
                    int dx = Nb[n, 0], dy = Nb[n, 1], cost = Nb[n, 2];
                    int nx = x + dx, ny = y + dy;
                    if (!IsFree(nx, ny, blocked)) continue;
                    if (dx != 0 && dy != 0 && (!IsFree(x + dx, y, blocked) || !IsFree(x, y + dy, blocked))) continue;
                    int ni = ny * C + nx, nd = d + cost;
                    if (nd < dist[ni]) { dist[ni] = nd; heap.Push(nd, ni); }
                }
            }
            return dist;
        }

        /// <summary>Cellule voisine de plus faible coût, ou -1.</summary>
        public int NextStep(int[] field, int x, int y)
        {
            int best = -1;
            long bestCost = long.MaxValue;
            for (int n = 0; n < 8; n++)
            {
                int dx = Nb[n, 0], dy = Nb[n, 1], c = Nb[n, 2];
                int nx = x + dx, ny = y + dy;
                if (!IsFree(nx, ny)) continue;
                if (dx != 0 && dy != 0 && (!IsFree(x + dx, y) || !IsFree(x, y + dy))) continue;
                int ni = ny * C + nx;
                if (field[ni] == Inf) continue;
                long cost = (long)field[ni] + c;
                if (cost < bestCost) { bestCost = cost; best = ni; }
            }
            return best;
        }

        sealed class MinHeap
        {
            readonly List<int> d = new List<int>(), v = new List<int>();
            public int Count { get { return d.Count; } }

            public void Push(int dist, int idx)
            {
                d.Add(dist); v.Add(idx);
                int k = d.Count - 1;
                while (k > 0)
                {
                    int p = (k - 1) >> 1;
                    if (d[p] <= d[k]) break;
                    Swap(p, k); k = p;
                }
            }

            public void Pop(out int dist, out int idx)
            {
                dist = d[0]; idx = v[0];
                int last = d.Count - 1;
                d[0] = d[last]; v[0] = v[last];
                d.RemoveAt(last); v.RemoveAt(last);
                int k = 0;
                for (;;)
                {
                    int l = 2 * k + 1, r = l + 1, m = k;
                    if (l < d.Count && d[l] < d[m]) m = l;
                    if (r < d.Count && d[r] < d[m]) m = r;
                    if (m == k) break;
                    Swap(m, k); k = m;
                }
            }

            void Swap(int a, int b)
            {
                int t = d[a]; d[a] = d[b]; d[b] = t;
                t = v[a]; v[a] = v[b]; v[b] = t;
            }
        }
    }
}
