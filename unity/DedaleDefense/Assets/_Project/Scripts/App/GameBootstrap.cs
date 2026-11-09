using System;
using DedaleDefense.Core;
using UnityEngine;

namespace DedaleDefense.App
{
    /// <summary>
    /// Point d'entrée du jeu : crée tout seul son GameObject au lancement (aucune scène à configurer),
    /// pilote la simulation à pas fixe 1/60 s et gère les entrées. Le rendu est dans GameBootstrap.View.cs.
    /// </summary>
    public sealed partial class GameBootstrap : MonoBehaviour
    {
        // Mise en page virtuelle (mise à l'échelle automatique selon la fenêtre).
        const float VW = 830f, VH = 560f;
        const float BX = 10f, BY = 62f;                 // coin du plateau
        const float PX = 590f, PW = 230f;               // panneau latéral

        enum Screen_ { Menu, Playing }

        Screen_ screen = Screen_.Menu;
        Core.Game game;
        readonly IBestStore store = new PrefsBestStore();
        int build = -1;                 // tour en cours de pose (-1 : aucune)
        Tower selected;
        int speed = 1;
        double acc;
        int hoverCx = -100, hoverCy = -100;
        PlaceResult hoverResult = PlaceResult.Out;
        bool hoverBoard;
        int seedCounter = Environment.TickCount;

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        static void Boot()
        {
            if (FindFirstObjectByType<GameBootstrap>() != null) return;
            var go = new GameObject("DedaleDefense");
            go.AddComponent<GameBootstrap>();
            Application.targetFrameRate = 60;
            Application.runInBackground = true;
        }

        // ---------- Cycle de vie ----------
        public void NewGame(ModeId mode)
        {
            game = new Core.Game(mode, seedCounter++, store);
            game.Ended += () => selected = null;
            build = -1;
            selected = null;
            speed = 1;
            acc = 0;
            screen = Screen_.Playing;
        }

        void Update()
        {
            if (screen != Screen_.Playing || game == null || game.Over) return;
            acc += Math.Min(Time.deltaTime, 0.25f) * speed;
            int guard = 0;
            while (acc >= Core.Game.Step && guard++ < 20)
            {
                game.Update(Core.Game.Step);
                acc -= Core.Game.Step;
            }
            if (selected != null && !game.Towers.Contains(selected)) selected = null;
        }

        // ---------- Actions ----------
        void SetBuild(int i)
        {
            build = build == i ? -1 : i;
            if (build >= 0) selected = null;
            RefreshHover();
        }

        void TogglePause() { if (game != null && !game.Over) game.Paused = !game.Paused; }
        void ToggleSpeed() { speed = speed == 1 ? 2 : speed == 2 ? 3 : 1; }

        void RefreshHover()
        {
            hoverResult = build >= 0 && hoverBoard ? game.PlaceCheck(hoverCx, hoverCy) : PlaceResult.Out;
        }

        void OnBoardClick(Vector2 p, bool shift)
        {
            int cx = Mathf.RoundToInt(p.x / Grid.Cell) - 1, cy = Mathf.RoundToInt(p.y / Grid.Cell) - 1;
            if (build >= 0)
            {
                if (game.TryPlace(build, cx, cy) == PlaceResult.Ok && !shift) build = -1;
                RefreshHover();
                return;
            }
            int ix = Mathf.Clamp((int)(p.x / Grid.Cell), 0, Grid.Cols - 1);
            int iy = Mathf.Clamp((int)(p.y / Grid.Cell), 0, Grid.Rows - 1);
            selected = game.Board.TowerAt[iy * Grid.Cols + ix];
        }

        void OnKey(Event e)
        {
            if (screen != Screen_.Playing || game == null) return;
            switch (e.keyCode)
            {
                case KeyCode.Alpha1: case KeyCode.Keypad1: SetBuild(0); break;
                case KeyCode.Alpha2: case KeyCode.Keypad2: SetBuild(1); break;
                case KeyCode.Alpha3: case KeyCode.Keypad3: SetBuild(2); break;
                case KeyCode.Alpha4: case KeyCode.Keypad4: SetBuild(3); break;
                case KeyCode.Alpha5: case KeyCode.Keypad5: SetBuild(4); break;
                case KeyCode.Alpha6: case KeyCode.Keypad6: SetBuild(5); break;
                case KeyCode.Alpha0: case KeyCode.Keypad0: case KeyCode.Escape: build = -1; selected = null; break;
                case KeyCode.U: game.Upgrade(selected); break;
                case KeyCode.S: if (game.Sell(selected)) selected = null; break;
                case KeyCode.Space: TogglePause(); break;
                case KeyCode.N: game.SendWave(); break;
                case KeyCode.F: ToggleSpeed(); break;
                default: return;
            }
            e.Use();
        }
    }
}
