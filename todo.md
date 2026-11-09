# TODO — Migration de Dédale Défense vers Unity (build natif Linux)

Ce document liste **toutes** les tâches nécessaires pour convertir le jeu actuel
(JavaScript natif + `<canvas>`, voir `app/src/game.js`, `index.html`, `style.css`)
en un projet **Unity** produisant un **exécutable natif Linux x86_64**.

Les sections sont triées par ordre logique : chaque section ne dépend que des
sections qui la précèdent. À l'intérieur d'une section, les tâches sont aussi
ordonnées pour éviter les blocages.

Légende : `[ ]` à faire · `[~]` en cours · `[x]` terminé · **(B)** tâche bloquante
pour la suite · *(réf.)* renvoie au code JS existant.

---

## 0. État des lieux du projet actuel (référence de migration)

Résumé de ce qui existe et doit être reproduit à l'identique.

- [ ] Relire `app/src/game.js` (1144 lignes) et noter le découpage en 8 blocs :
      constantes/données, état de jeu, pathfinding, placement, vagues/ennemis,
      tir/projectiles, rendu canvas, interface DOM, entrées, boucle.
- [ ] Relire `user.md` (guide joueur) : source de vérité des valeurs attendues
      (modes, tours, ennemis, raccourcis).
- [ ] Relire `app/src/index.html` + `style.css` : layout (plateau 560×440 à gauche,
      panneau latéral 240 px à droite, barre des vagues, overlay), palette de
      couleurs (variables CSS `--frame`, `--panel`, `--gold`, `--danger`, `--ok`…).
- [ ] Noter les invariants de gameplay à conserver absolument :
  - plateau 28×22 cellules de 20 px, bords murés sauf 4 entrées gauche
    (lignes 9-12) / haut (colonnes 12-15) et 4 sorties droite / bas ;
  - tours 2×2 cellules, 5 niveaux (base + 4 améliorations) ;
  - **une tour ne peut jamais fermer le dernier passage** (`placeCheck`) ;
  - 50 vagues, cycle de 10 types, formule de PV `12·1,115^(n-1) + 4n`,
    prime `bounty·(2 + ⌊n/6⌋)` ;
  - à partir de la vague 15, les ennemis alternent entre les deux entrées ;
  - pas fixe de simulation 1/60 s, vitesses ×1 / ×2 / ×3 ;
  - revente : 100 % avant la 1re vague, 75 % ensuite ;
  - score : 10 × prime par kill, bonus `⌈timer⌉ × (vague+1)` pour envoi anticipé,
    `vies × 100` à la victoire ; record par mode persisté.
- [ ] Lister le hook de test `window.__game` (`state`, `newGame`, `sendWave`,
      `setBuild`, `tryPlace`, `upgrade`, `sell`, `placeCheck`, `step`) : il devra
      avoir un équivalent C# pour les tests automatisés.

---

## 1. Décisions d'architecture et pré-requis **(B)**

- [ ] Choisir la version Unity : **Unity 6 LTS (6000.x)** recommandée (support
      Linux éditeur + player stable, IL2CPP Linux disponible). Fixer la version
      exacte dans `ProjectSettings/ProjectVersion.txt`.
- [ ] Choisir le pipeline de rendu : **URP 2D** (Universal Render Pipeline avec
      2D Renderer) — suffisant pour un jeu 2D vectoriel, léger, compatible Linux.
- [ ] Choisir la stratégie de rendu des formes (tours, ennemis, projectiles) :
  - option A : sprites générés à la main dans un outil vectoriel (SVG → PNG) ;
  - option B : génération procédurale des meshes/sprites en C# au démarrage
    (reproduit fidèlement `drawTowerShape` / `drawCreep`) ;
  - **recommandé : B** pour rester fidèle au rendu actuel sans assets externes.
- [ ] Choisir le système d'UI : **UI Toolkit (UXML/USS)** — mapping naturel
      depuis l'HTML/CSS existant (classes, flexbox, variables). Alternative : uGUI.
- [ ] Choisir le système d'entrées : **Input System (nouveau)** avec actions
      nommées (Tour1..6, Annuler, Améliorer, Vendre, Pause, VagueSuivante,
      Vitesse, Multi-placement).
- [ ] Choisir le backend de scripting pour le build Linux : **IL2CPP** pour la
      release (performances, obfuscation), Mono pour les builds de dev rapides.
- [ ] Décider de la persistance du record : `PlayerPrefs` (équivalent direct de
      `localStorage`, clé `dd-best-<mode>`) ou fichier JSON dans
      `Application.persistentDataPath`. **Recommandé : PlayerPrefs** pour le
      record, migration JSON possible plus tard.
- [ ] Décider du traitement du site web existant (`app/`) : le conserver tel quel
      (version web) et ajouter le projet Unity à côté dans `unity/`. Pas de
      suppression du JS tant que la parité n'est pas validée (§ 11).
- [ ] Valider le nom du produit, l'identifiant (`com.clem2k.dedaledefense`),
      l'icône (reprendre le favicon SVG : deux carrés jaune/bleu sur fond
      `#1d2939`) et la licence des assets.

---

## 2. Environnement de développement et outillage

- [ ] Installer **Unity Hub** sur Linux et la version Unity choisie avec les
      modules **Linux Build Support (IL2CPP)** et **Linux Build Support (Mono)**.
- [ ] Installer les dépendances système IL2CPP Linux : `clang`, `lld`,
      `libc6-dev`, `build-essential` (vérifier la liste exacte dans la doc Unity
      de la version choisie).
- [ ] Installer un IDE C# (VS Code + extension C# Dev Kit, ou Rider) et
      générer les `.csproj` depuis Unity.
- [ ] Ajouter `.gitignore` Unity standard (Library/, Temp/, Logs/, obj/,
      Build/, UserSettings/, `*.csproj`, `*.sln`…) en complément du `.gitignore`
      actuel.
- [ ] Configurer **Git LFS** pour les binaires (`*.png`, `*.wav`, `*.ogg`,
      `*.ttf`, `*.otf`, `*.asset` volumineux si besoin).
- [ ] Activer dans Unity : `Asset Serialization = Force Text`,
      `Version Control = Visible Meta Files` (diffs lisibles, meta versionnés).
- [ ] Mettre en place un script shell `unity/build-linux.sh` lançant Unity en
      mode batch (`-batchmode -nographics -executeMethod … -buildTarget Linux64`).
- [ ] Documenter dans `README.md` la procédure d'installation (section
      « Build natif Linux »).

---

## 3. Création du projet Unity et squelette

- [ ] Créer le projet `unity/DedaleDefense/` avec le template **2D (URP)**.
- [ ] Installer les packages : `com.unity.inputsystem`,
      `com.unity.render-pipelines.universal`, `com.unity.ui` (UI Toolkit),
      `com.unity.test-framework`, `com.unity.textmeshpro` (si uGUI de secours).
- [ ] Définir la structure `Assets/` :
  ```
  Assets/
    _Project/
      Scenes/        (Boot, Game)
      Scripts/
        Core/        (données, constantes, état)
        Simulation/  (grille, pathfinding, placement, vagues, tir)
        Rendering/   (génération des formes, VFX, textes flottants)
        UI/          (UXML/USS, contrôleurs)
        Input/
        Persistence/
        Testing/     (hook d'automatisation)
      Data/          (ScriptableObjects tours/ennemis/modes)
      UI/            (fichiers .uxml / .uss / thème)
      Art/           (icône, polices)
      Settings/      (URP assets, Input Actions)
    Tests/
      EditMode/
      PlayMode/
  ```
- [ ] Créer les **Assembly Definitions** : `DedaleDefense.Core` (simulation pure,
      sans dépendance UnityEngine côté logique si possible),
      `DedaleDefense.Game` (MonoBehaviours, rendu, UI), `DedaleDefense.Tests.*`.
- [ ] Configurer la caméra orthographique : taille adaptée à un plateau 560×440
      unités (1 unité = 1 px, ou 1 unité = 1 cellule de 20 px — fixer la
      convention et la documenter dans un `GridConstants.cs`).
- [ ] Configurer `Project Settings` : résolution par défaut 1280×720 fenêtré
      redimensionnable, `Run in Background = true`, VSync activé, cible 60 FPS.
- [ ] Configurer `Time.fixedDeltaTime = 1/60` **(B)** pour aligner la simulation
      sur le `STEP` actuel.

---

## 4. Données de jeu (ScriptableObjects) **(B)**

*(réf. `MODES`, `TOWERS`, `CREEPS`, `CYCLE`, `waveDef`)*

- [ ] Créer `GameModeData` (ScriptableObject) : `id`, `displayName`,
      `hpMultiplier`, `lives`, `startGold`, `waveGapSeconds`.
      Instancier **Facile** (0,75 / 30 / 70 / 28), **Normal** (1 / 20 / 60 / 24),
      **Difficile** (1,35 / 10 / 50 / 20).
- [ ] Créer `TowerData` : `id`, `displayName`, `hotkey`, `color`, `accentColor`,
      `description`, `cost`, `upgradeCosts[4]`, `damage[5]`, `baseRange`,
      `baseRate`, `hitsGround`, `hitsAir`, `shotKind` (enum Pellet/Shell/Dart/
      Missile/Ice/Pulse), `projectileSpeed`, `splashRadius`, `multiShot`,
      `slowFactors[5]`, `stunChance`.
      Instancier les 6 tours **Caillou, Éclat, Aiguille, Volée, Givre, Onde**
      avec les valeurs exactes de `TOWERS`.
- [ ] Créer `CreepData` : `id`, `displayName`, `hpFactor`, `speed`, `count`,
      `spawnGap`, `bountyFactor`, `color`, `radius`, `shape` (enum Circle/Tri/
      Square/Diamond/Wing/Boss), `isFlying`, `isImmune`, `splitsOnDeath`,
      `description`.
      Instancier les 8 types **walker, runner, swarm, flyer, armored, splitter,
      mini, boss**.
- [ ] Créer `WaveCycleData` : liste ordonnée des 10 types (`CYCLE`) + constantes
      `TotalWaves = 50`, formule de PV et de prime dans une classe statique
      `WaveMath` (fonction pure, testable).
- [ ] Créer `GameBalanceData` regroupant les constantes globales : `CELL = 20`,
      `COLS = 28`, `ROWS = 22`, `OPEN_R = {9,10,11,12}`, `OPEN_C = {12,13,14,15}`,
      bonus de revente 0,75, durée de ralentissement 2 s, durée d'étourdissement
      0,6 s, coefficients d'amélioration (portée +8 %/niv., cadence −4 %/niv.),
      limite `dt` max 0,1 s.
- [ ] Écrire un **test EditMode** qui compare chaque ScriptableObject aux valeurs
      du tableau de `user.md` (empêche toute dérive des données).
- [ ] Vérifier que toutes les chaînes affichables (noms, descriptions) sont
      en français identique à l'original ; préparer les clés pour une future
      localisation (§ 10).

---

## 5. Simulation : grille, pathfinding et placement **(B)**

*(réf. `wall`, `computeField`, `free`, `nextStep`, `refreshFields`, `cellsOf`,
`placeCheck`)*

- [ ] Implémenter `Grid` : tableau `byte[COLS*ROWS]` des murs initiaux
      (bords sauf ouvertures), tableau `Tower[] towerAt`, helpers `Index(x,y)`,
      `IsFree(x,y, blockedSet)`.
- [ ] Implémenter `FlowField` : Dijkstra 8 directions (coûts 10 / 14),
      **sans coupe de coin** (les deux cases orthogonales doivent être libres),
      min-heap binaire maison (reproduire `computeField`), retour
      `float[] dist` avec `+∞` pour inaccessible.
- [ ] Implémenter `NextStep(field, x, y)` : choix du voisin minimisant
      `field[n] + coût`, même règle de coin.
- [ ] Implémenter `RefreshFields()` : recalcul des deux champs (sorties droite
      `EXIT_R`, sorties bas `EXIT_B`) + aperçu des deux routes depuis le milieu
      de chaque entrée (`ENTRY_L[1]`, `ENTRY_T[1]`), garde de 800 pas.
- [ ] Implémenter `PlacementValidator.Check(cx, cy)` retournant un enum
      `Out / Taken / Creep / Block / Ok` avec exactement les mêmes règles :
  - hors des cases 1..COLS-3 / 1..ROWS-3 → `Out` ;
  - une des 4 cases occupée (tour ou mur) → `Taken` ;
  - un ennemi terrestre a sa `cell` ou `wcell` dans les 4 cases → `Creep` ;
  - champs recalculés avec les 4 cases bloquées : si **une** entrée devient
    inaccessible ou si **un** ennemi terrestre en chemin perd sa route → `Block`.
- [ ] Écrire les tests EditMode : grille vide → toutes entrées accessibles ;
      mur complet sauf une case → `Block` ; coin diagonal interdit ; parité
      avec quelques positions calculées à la main depuis la version JS.
- [ ] Mesurer le coût du `placeCheck` (2 Dijkstra par frame au survol) et,
      si nécessaire, mettre en cache le résultat par cellule survolée.

---

## 6. Simulation : état de jeu, tours, ennemis, vagues, tir **(B)**

*(réf. `newGame`, `tryPlace`, `upgrade`, `sell`, `sellValue`, `towerStats`,
`sendWave`, `spawn`, `makeCreep`, `moveCreep`, `leak`, `hurt`, `fire`,
`updateShots`, `update`, `endGame`)*

- [ ] Créer `GameState` (classe C# pure) : `mode`, `lives`, `gold`, `score`,
      `kills`, `wave`, `started`, `paused`, `over`, `won`, `timer`, `time`,
      listes `towers`, `creeps`, `spawners`, `shots`, `fx`, `floats`, `nextId`.
- [ ] Créer `Tower` : `id`, `data`, `cx`, `cy`, `level`, position monde,
      `cooldown`, `angle`, `spent`, `builtWave`, `builtStarted`.
      Méthode `Stats()` → dégâts, portée `(1+0,08·L)`, cadence `(1−0,04·L)`, slow.
- [ ] Créer `Creep` : `id`, `data`, position, `hp`, `maxHp`, `bounty`,
      `target` (R/B), `speed`, `flying`, `radius`, `slowUntil`, `slowFactor`,
      `stunUntil`, `cell`, `wcell`, `wx/wy` (waypoint), `exiting`, `dead`,
      `wobble`.
- [ ] Créer `Spawner` : `waveDef`, `left`, `t`, `k` (index pour alternance).
- [ ] Créer `Shot` : `kind`, position, `target`, `tx/ty`, `speed`, `dmg`,
      `splash`, `slow`, `color`, `done`.
- [ ] Implémenter `GameSimulation.NewGame(mode)` : initialisation + `RefreshFields`.
- [ ] Implémenter `TryPlace(cx, cy, towerIndex, shiftHeld)` : vérification or,
      `PlacementValidator`, création, décrément or, `RefreshFields`, message
      flottant d'erreur (« Pas assez d'or », « Ne bloquez pas le passage ! »,
      « Un ennemi est là », « Place occupée », « Hors du plateau »).
- [ ] Implémenter `Upgrade(tower)` (max 4, coût `ups[level]`, FX anneau) et
      `Sell(tower)` (valeur `SellValue`, libération des cases, float `+v`,
      `RefreshFields`).
- [ ] Implémenter `SendWave()` : bonus envoi anticipé `⌈timer⌉·(wave+1)` avec
      float « Envoi anticipé +N pts », incrément vague, création spawner,
      reset timer au `gap` du mode.
- [ ] Implémenter `Spawn(def, k)` : choix de la cible (`n ≥ 15` → alternance
      `k%2`, sinon `n%2`), PV = `def.hp · creep.hp · mode.hp`.
- [ ] Implémenter `MakeCreep(type, hp, bounty, target, from)` : voie aléatoire
      parmi 4, position d'entrée hors plateau (`-0,6·CELL`), cas du scindeur
      (position parent ± 3 px, héritage de `wcell`/`exiting`), cas volant
      (ligne droite vers `W+CELL` ou `H+CELL`).
- [ ] Implémenter `Remaining(creep)` (distance restante pour le ciblage) et
      `MoveCreep(creep, dt)` : étourdissement, ralentissement, avancée par
      waypoints, passage en `exiting` quand `field == 0`, appel `Leak`.
- [ ] Implémenter `Leak(creep)` : −1 vie (−2 pour boss), float « -N vie »
      clampé au plateau, `EndGame(false)` à 0.
- [ ] Implémenter `Hurt(creep, dmg)` : mort → or `max(1, round(bounty))`,
      score ×10, kills, float `+g`, FX « pop », scission en 2 `mini`
      (30 % des PV max, 50 % de la prime).
- [ ] Implémenter `Fire(tower)` : filtre `canHit` + portée `range + r`,
      cas **Onde** (anneau FX, dégâts de zone, étourdissement 15 % sauf immunisés),
      tri par `Remaining` croissant, `multi` cibles pour Volée, mise à jour de
      l'angle, création des `Shot` à 8 px du centre.
- [ ] Implémenter `UpdateShots(dt)` : poursuite de la cible vivante, impact à
      `d ≤ step + 2`, dégâts de zone (Éclat, anneau bleu `#4ea1ff`) vs dégâts
      directs + ralentissement 2 s (max des facteurs) sauf immunisés.
- [ ] Implémenter `Update(dt)` dans l'ordre exact : temps, timer de vague
      auto-envoi, spawners, déplacement ennemis, (sortie si `over`), tours
      (cooldown → `Fire` → `rate` ou 0), projectiles, nettoyage morts, FX,
      floats (montée 18 px/s), victoire si `wave == 50` et plus rien en jeu.
- [ ] Implémenter `EndGame(won)` : bonus `vies × 100`, lecture/écriture du
      record (§ 9), flags `best` / `newBest`.
- [ ] Implémenter la boucle à pas fixe : accumulateur `acc += dt × speed`,
      `while (acc ≥ 1/60) Update(1/60)`, `dt` plafonné à 0,1 s. Décider :
      `FixedUpdate` natif Unity avec `Time.timeScale = speed`, **ou** boucle
      manuelle dans `Update` (plus fidèle au JS). **Recommandé : boucle manuelle.**
- [ ] Gérer la pause (`S.paused`), l'overlay ouvert (`ui.overlayOpen`) et
      `S.over` comme conditions d'arrêt de `Update`.
- [ ] Déterminisme : centraliser `Random` dans un `IRandom` injectable (seed
      fixable) pour les tests de parité (§ 11).
- [ ] Écrire les tests EditMode : `WaveMath` (PV/prime vagues 1, 10, 15, 50),
      `SellValue` avant/après 1re vague, `Upgrade` au niveau max refusé,
      `TryPlace` sans or, simulation d'une vague 1 complète sans tour → 10 vies
      perdues, Colosse → 2 vies.

---

## 7. Rendu du plateau et des entités

*(réf. `setupCanvas`, `drawBackground`, `drawTowerShape`, `roundRect`,
`drawCreep`, `draw`)*

### 7.1 Fond statique
- [ ] Générer une texture/mesh de fond une seule fois : dégradé linéaire
      `#16243a → #122033`, grille (lignes alternées alpha 0,06 / 0,13), murs
      `#0b1422` avec hachures diagonales, zones d'entrée vertes
      `rgba(60,203,127,0.16)` et de sortie rouges `rgba(235,55,81,0.16)`,
      4 flèches directionnelles.
- [ ] Choisir l'implémentation : `Texture2D` peinte en C# au démarrage (fidèle
      au canvas) ou `Shader Graph` + `LineRenderer`. **Recommandé : Texture2D.**

### 7.2 Tours
- [ ] Créer `TowerShapeFactory` : génère pour chaque type et niveau un sprite
      (base arrondie 36×36 `#26354d`, intérieur 28×28 `#1a2638`, corps coloré
      selon le type : cercle / hexagone / losange / carré à 4 trous / 3 traits +
      disque / disque à anneaux, pips de niveau jaunes `#fac515`).
- [ ] Séparer le **canon** (rectangle `accent`, 11 ou 14 px pour Aiguille) en
      sprite enfant rotatif pour les tours sauf Onde et Givre.
- [ ] Créer le prefab `TowerView` (SpriteRenderer corps + canon + sélection
      `#fac515` 2 px) piloté par les données `Tower`.
- [ ] Afficher le cercle de portée (remplissage `#ffffff18`, trait `#ffffff88`)
      pour la tour sélectionnée.

### 7.3 Ennemis
- [ ] Créer `CreepShapeFactory` : cercle, triangle (orienté direction), carré,
      losange, aile (battement `sin(t·14 + wob)·3`), boss étoile à 10 branches
      tournante (`a + t`).
- [ ] Créer le prefab `CreepView` : forme, contour (noir 50 % ou cyan `#5fe0e0`
      2 px si ralenti), ombre elliptique pour les volants, oscillation verticale
      `sin(t·6 + wob)·1,5` des volants, étoile « ✶ » `#fff0c2` si étourdi,
      barre de PV (fond noir 60 %, vert > 50 % / jaune > 25 % / rouge).
- [ ] Respecter l'ordre de rendu : terrestres puis volants (sorting layers).
- [ ] Mettre en place un **pool d'objets** pour les ennemis (jusqu'à ~22 par
      vague + scissions).

### 7.4 Projectiles et effets
- [ ] Créer `ShotView` par type : dart (trait 2 px de 9 px de long), missile
      (disque 2,5 + traînée orange 40 %), shell (disque 3,5), défaut (disque 2,2).
- [ ] Créer `FxView` : anneau en expansion (`r → max`, alpha décroissant,
      2 px) et « pop » (6 points en cercle s'éloignant).
- [ ] Créer `FloatingText` : texte gras 11 px avec ombre noire 60 %, montée
      18 px/s, fondu `min(1, life/max·2)`. Utiliser TextMeshPro ou UI Toolkit
      world-space.
- [ ] Pool d'objets pour projectiles, FX et textes flottants.

### 7.5 Couches d'aide au jeu
- [ ] Aperçu des routes : points `rgba(250,197,21,0.22)` rayon 1,8 sur chaque
      cellule des deux chemins (`S.path`).
- [ ] Fantôme de placement : cercle de portée vert `#3ccb7f` / rouge `#eb3751`,
      tour à 65 % d'opacité, surbrillance de la zone 2×2.
- [ ] Bandeau « PAUSE » (fond `rgba(10,16,28,0.55)`, titre 28 px, sous-texte
      « Appuyez sur Espace pour reprendre »).
- [ ] Bandeau de départ (cartouche arrondi 340×44) « Construisez votre
      labyrinthe, puis lancez la vague » / « Bouton « Vague suivante » ou touche N ».
- [ ] Mise à l'échelle : conserver le ratio 560:440 du plateau quel que soit
      la taille de fenêtre (équivalent de `fit()` + `pixelScale`), rendu net
      (pas de flou de filtrage : `FilterMode.Point` ou sprites vectoriels).

---

## 8. Interface utilisateur (UI Toolkit)

*(réf. `index.html`, `style.css`, objet `ui`)*

### 8.1 Thème et layout
- [ ] Créer `Theme.uss` reprenant les variables CSS : `--page #222222`,
      `--frame #1d2939`, `--panel #182230`, `--line #344054`, `--text #ffffff`,
      `--muted #98a2b3`, `--soft #d0d5dd`, `--gold #fac515`, `--accent #eaaa08`,
      `--danger #eb3751`, `--ok #3ccb7f`, `--radius 8px`.
- [ ] Importer une police libre proche de `system-ui` (ex. Inter) et la
      référencer dans un `PanelSettings` (fallback Unicode pour « ✶ », « ← », « ↑ »).
- [ ] Créer `GameScreen.uxml` : conteneur `.game` (flex row, gap 12, padding 12,
      fond `--frame`, bordure `--line`, radius 14) contenant `.left` (560 px :
      titre + hint, plateau, barre des vagues) et `.panel` (stats, tours, info,
      contrôles).
- [ ] Reproduire le mode **« stacked »** (colonne) quand la fenêtre est trop
      étroite (< 880 px) et l'échelle globale 0,2 → 1,6 de `fit()`.

### 8.2 Composants
- [ ] **Titre** « Dédale **Défense** » (span accent) + hint des raccourcis.
- [ ] **Stats** : Vague `x/50`, Vies (classe `low` rouge si ≤ 5), Or (classe
      `gold`), Score — mise à jour uniquement si la valeur change.
- [ ] **Boutons tours** (6) : `kbd` touche, icône 56×56 rendue depuis
      `TowerShapeFactory` (échelle 0,75), nom, coût « N or » ; états `sel`
      (sélectionné) et `poor` (or insuffisant) ; survol → panneau info.
- [ ] **Panneau info** à 3 états :
  - tour sélectionnée : titre + « niveau L/5 », description, tableau Dégâts /
    Portée / Cadence (« x,xx s ») / Ralentit (%) / Vise, avec valeur suivante
    « → nv » en `.up`, boutons **Améliorer N (U)** (désactivé si max ou pauvre,
    libellé « Niveau max ») et **Vendre N (S)** ;
  - type survolé/choisi : titre + coût, description, Dégâts « d0 → d4 »,
    Portée, Cadence, Vise, puis aide contextuelle (« Cliquez sur le plateau… »
    ou « Touche k pour la choisir. ») ;
  - défaut : « Comment jouer » (2 paragraphes).
- [ ] **Contrôles** : « Vague suivante (N) » (désactivé si fini / 50 vagues),
      « Pause » / « Reprendre », « Vitesse ×1/2/3 », « Nouvelle partie » (ghost).
- [ ] **Barre des 7 prochaines vagues** : pastille couleur, « n. Nom »,
      « ← / ↑ / ← ↑  count · PV », classe `boss`, tooltip description,
      barre de progression `.bar` sur la prochaine (`1 − timer/gap`) ;
      message « Dernière vague envoyée ! » après la 50e.
- [ ] **Overlay de démarrage** : carte 380 px, titre, texte « … 50 vagues à
      tenir. », 3 boutons de mode « Nom · N vies · record R » (Normal en
      `primary`).
- [ ] **Overlay de fin** : « Victoire ! » / « Partie terminée », phrase
      récapitulative (vagues tenues, ennemis arrêtés), score en grand,
      « Nouveau record ! » ou « Record : R » · mode, boutons « Rejouer » et
      « Changer de mode ».
- [ ] Reproduire le comportement « les boutons perdent le focus après un clic »
      pour que Espace reste la pause.
- [ ] Curseur : masqué au-dessus du plateau en mode construction, `crosshair`
      sinon (`Cursor.SetCursor` / `Cursor.visible`).

### 8.3 Liaison UI ↔ simulation
- [ ] Créer `GamePresenter` : abonne l'UI aux événements de `GameSimulation`
      (or/vies/score/vague changés, tour sélectionnée, overlay) sans logique
      métier côté UI.
- [ ] Mettre à jour l'UI chaque frame comme `ui.update()` mais avec diff
      (éviter les re-layouts inutiles).

---

## 9. Entrées et persistance

*(réf. section « Input », `endGame`, `ui.showStart`)*

- [ ] Créer `GameInput.inputactions` : `SelectTower1..6` (1-6), `Cancel`
      (0, Échap), `Upgrade` (U), `Sell` (S), `Pause` (Espace), `NextWave` (N),
      `Speed` (F), `MultiPlace` (Maj maintenue), `Place/Select` (clic gauche),
      `CancelBuild` (clic droit), `Pointer` (position).
- [ ] Convertir la position souris écran → coordonnées plateau → cellule
      (`cx = round(x/CELL − 1)` pour les tours 2×2, `floor(x/CELL)` pour la
      sélection), gérer la sortie du plateau (`hover = null`).
- [ ] Ignorer les entrées quand un overlay est ouvert ou la partie terminée ;
      `N` ignoré en pause.
- [ ] Réinitialiser `shiftHeld` à la perte de focus de la fenêtre
      (`OnApplicationFocus`).
- [ ] Support manette/clavier seul (optionnel) : navigation des overlays.
- [ ] Implémenter `HighScoreStore` : `GetBest(mode)` / `SetBest(mode, score)`
      via `PlayerPrefs` clé `dd-best-<mode>` ; robuste si l'écriture échoue.
- [ ] Prévoir un `SettingsStore` (plein écran, VSync, volume futur).
- [ ] Gérer la fermeture propre : `Application.wantsToQuit` → sauvegarde.

---

## 10. Textes, localisation et accessibilité

- [ ] Centraliser toutes les chaînes FR dans une table (`Strings.cs` ou
      Unity Localization) : messages flottants, UI, overlays, descriptions.
- [ ] Vérifier le rendu des caractères spéciaux (é, à, œ, « », ✶, ←, ↑, ×, ·).
- [ ] Préparer les langues du `build.mjs` (en, es, de, id) comme tables vides à
      remplir ultérieurement (non bloquant).
- [ ] Mode « réduire les animations » optionnel (oscillations, rotations).

---

## 11. Tests, parité et validation

- [ ] Exposer un hook `GameAutomation` (équivalent de `window.__game`) :
      accès lecture à l'état, `NewGame`, `SendWave`, `SetBuild`, `TryPlace`,
      `Upgrade`, `Sell`, `PlaceCheck`, `Step(seconds)` — compilé uniquement en
      `DEVELOPMENT_BUILD` / éditeur.
- [ ] Tests **EditMode** (simulation pure, voir § 4-6) exécutés en batch :
      `unity -runTests -testPlatform EditMode`.
- [ ] Tests **PlayMode** : lancement de la scène, placement d'une tour par
      clic simulé, envoi d'une vague, vérification de l'UI (or/vies).
- [ ] **Scénario de parité** : exporter depuis la version JS (via
      `window.__game.step`) un script déterministe (seed figée côté JS en
      remplaçant `Math.random` par un PRNG) : séquence de placements +
      `step(n)` → `{lives, gold, score, kills, wave}` ; rejouer en C# et
      comparer. Tolérance zéro sur les entiers.
- [ ] Vérifier manuellement chaque règle du guide `user.md` (checklist dans
      `todo` ou issue dédiée) : refus de blocage, scission, immunité Blindés,
      volants ignorant le labyrinthe, 2 vies par Colosse, revente 100 %/75 %,
      bonus envoi anticipé, record par mode.
- [ ] Comparaison visuelle : captures Unity 1440/768/390 face aux
      `checks/round-7/screenshot-*.png` existants (couleurs, proportions).
- [ ] Profiling Linux : viser 60 FPS stables en vague 40+ (≈ 40 tours,
      22 ennemis, dizaines de projectiles) ; vérifier l'absence d'allocations
      par frame (pools § 7).
- [ ] Test de longue durée : partie complète 50 vagues en vitesse ×3 sans fuite
      mémoire ni dérive de la simulation.

---

## 12. Build natif Linux et distribution

- [ ] Script d'éditeur `BuildScript.BuildLinux()` : cible
      `StandaloneLinux64`, scènes `Boot` + `Game`, options Release/IL2CPP et
      Development/Mono, sortie `Build/Linux/DedaleDefense.x86_64`.
- [ ] Valider un premier build **Mono** (rapide) puis **IL2CPP** (release).
- [ ] Vérifier le lancement sur une distribution propre (Ubuntu 22.04/24.04,
      Fedora) : dépendances runtime (`libgl1`, `libxcursor1`, `libxrandr2`,
      `libxi6`, `libpulse0`…), permissions d'exécution, fichier `.desktop`.
- [ ] Tester X11 **et** Wayland (XWayland), multi-écrans, DPI élevé.
- [ ] Icône Linux (PNG 256/512 depuis le favicon SVG) et métadonnées
      (`Company`, `Product`, `Version` semver alignée sur `app/package.json`).
- [ ] Packaging : archive `tar.gz` minimale ; étudier **AppImage** et/ou
      **Flatpak** (manifest `com.clem2k.DedaleDefense.yml`) pour la distribution.
- [ ] Stripping IL2CPP : vérifier qu'aucun type de données (ScriptableObjects,
      sérialisation) n'est supprimé (`link.xml` si nécessaire).
- [ ] Taille du build : viser < 100 Mo (supprimer modules inutilisés :
      Physics 3D, Audio si non utilisé, XR…).

---

## 13. Intégration continue

- [ ] Workflow GitHub Actions `.github/workflows/unity-linux.yml` basé sur
      **GameCI** (`game-ci/unity-test-runner`, `game-ci/unity-builder`) :
      tests EditMode/PlayMode puis build `StandaloneLinux64`.
- [ ] Stocker la licence Unity (`UNITY_LICENSE` / `UNITY_EMAIL` /
      `UNITY_PASSWORD`) dans les **secrets** du dépôt — jamais dans le code.
- [ ] Cache du dossier `Library/` pour accélérer les runs.
- [ ] Publication automatique de l'artefact `tar.gz` / AppImage sur les tags
      `v*` via GitHub Releases.
- [ ] Conserver le job existant éventuel pour la version web (`npm run build`)
      en parallèle.

---

## 14. Documentation et finalisation

- [ ] Mettre à jour `README.md` : nouvelle structure (`app/` web, `unity/`
      natif), pré-requis, commandes de build/test Linux, téléchargement des
      releases.
- [ ] Mettre à jour `user.md` : section « Version Linux » (installation,
      lancement, emplacement des sauvegardes `~/.config/unity3d/...`).
- [ ] Ajouter `CHANGELOG.md` avec l'entrée « Portage Unity / build Linux ».
- [ ] Ajouter `unity/README.md` : conventions (1 unité = ?, assemblies,
      ordre d'update, pools), comment ajouter une tour/un ennemi via
      ScriptableObject.
- [ ] Revue de code finale, nettoyage des `TODO`/`Debug.Log`, vérification
      `.gitignore`/LFS, tag `v2.0.0-linux`.
- [ ] Décider du sort de la version web : maintien en parallèle (recommandé)
      ou dépréciation, et l'indiquer dans le README.

---

## Annexe A — Correspondance JS → Unity (aide-mémoire)

| Élément JS (`game.js`)            | Équivalent Unity proposé                          |
|-----------------------------------|---------------------------------------------------|
| `const TOWERS/CREEPS/MODES`       | ScriptableObjects `TowerData/CreepData/GameModeData` |
| `let S` (état global)             | `GameState` (POCO) détenu par `GameSimulation`    |
| `computeField` (Dijkstra)         | `FlowField` dans `DedaleDefense.Core`             |
| `placeCheck`                      | `PlacementValidator.Check`                        |
| `requestAnimationFrame` + `STEP`  | `Update()` + accumulateur, `Update(1/60f)`        |
| `Math.random()`                   | `IRandom` injectable (System.Random seedé)        |
| `localStorage`                    | `PlayerPrefs` (`HighScoreStore`)                  |
| `<canvas>` 2D                     | Sprites générés + SpriteRenderer / Texture2D fond |
| `ctx.fillText` (floats)           | TextMeshPro world-space poolé                     |
| HTML/CSS panneau                  | UI Toolkit UXML/USS                               |
| `addEventListener` clavier/souris | Input System actions                              |
| `window.__game`                   | `GameAutomation` (dev builds uniquement)          |
| `build.mjs` / `npm run build`     | `BuildScript.BuildLinux()` + `build-linux.sh`     |

## Annexe B — Dépendances entre sections

```
0 ─► 1 ─► 2 ─► 3 ─► 4 ─► 5 ─► 6 ─┬─► 7 ─┐
                                  ├─► 8 ─┼─► 11 ─► 12 ─► 13 ─► 14
                                  └─► 9 ─┘
                                  10 (parallèle à 7-9)
```
