# Dédale Défense

Jeu de tower defense inspiré de *Desktop Tower Defense* : les ennemis traversent le plateau, et ce sont vos tours qui forment le labyrinthe. Une tour ne peut jamais fermer le dernier passage.

Voir [user.md](user.md) pour le guide du joueur.

## Monorepo et compilation

- [Web](apps/web/) : JavaScript et canvas.
- [Bureau](apps/desktop/) : application native Rust/Macroquad.
- [Android](apps/android/README.md) : APK natif Rust/OpenGL ES, sans WebView,
  avec commandes tactiles en paysage. Il partage le moteur du bureau.
- [Moteur natif](packages/game-native/) et [lecteur de contenu](packages/content/) :
  code commun ; CSV, JSON et médias restent à la racine, sans duplication.

**GitHub sert uniquement au stockage des fichiers. Aucun GitHub Actions ni build
hébergé sur GitHub n'est utilisé.** Compiler en local ou sur votre Jenkins dans
Docker, avec le [Dockerfile](infra/docker/Dockerfile) et le [Jenkinsfile](Jenkinsfile).
Les téléchargements de dépendances/outils se font sur la machine de build.

```sh
npm test
npm run build
cargo test --workspace --locked
cargo run -p dedale-desktop --locked
# Avec le SDK Android et cargo-quad-apk installés :
bash scripts/build-android.sh debug
```

Le guide Android détaille les prérequis, le build Docker, l'installation par ADB
et la signature de développement. Les anciens chemins `app/` et `rust/` sont
remplacés par les dossiers ci-dessus ; les sorties deviennent `apps/web/dist/`
et `target/` à la racine.

## Effets sonores

Les trois applications utilisent les MP3 de [assets/fx/](assets/fx/) :
`01` à `06` correspondent respectivement à Caillou, Éclat, Aiguille, Volée,
Givre et Onde ; `HIT` accompagne chaque ennemi qui fait perdre des vies,
y compris la dernière vie. Une salve multicible ne joue qu'un son de tir.
Les tours personnalisées reprennent le son de leur type de projectile (`shot`).

**Options** suspend temporairement la simulation et règle le volume FX de 0 à 100 %.
Le bouton **Mute OFF / Mute ON**, toujours visible dans l'interface principale,
coupe ou réactive immédiatement les sons, sans modifier ce volume.
Les réglages restent actifs entre les parties pendant la session ; le volume initial
est de 50 %. Dans le navigateur, les sons s'activent après la première interaction.

Le build web copie les MP3 avec des chemins relatifs au module pour toutes les pages
localisées. Le build Rust décode les MP3 en WAV avec `minimp3` et `hound`, puis les
embarque : ni convertisseur externe ni fichier audio externe n'est requis à l'exécution.
La fonctionnalité audio de Macroquad est activée ; sous Linux, ALSA doit être disponible.

## Sprites communs

Les deux moteurs utilisent le même [atlas PNG transparent](assets/towers/towers.png),
référencé directement dans [towers.csv](towers.csv). Ses six lignes correspondent
à Caillou, Éclat, Aiguille, Volée, Givre et Onde ; ses huit colonnes sont
E, SE, S, SW, W, NW, N et NE. Les vues ne sont pas tournées par le moteur :
l'angle de visée sélectionne la direction la plus proche, avec un cadrage fixe
et une marge transparente contre les débordements entre images.

Le rendu sur le plateau reste dans une empreinte de 40 × 40 pixels.
Les boutons, les aperçus de placement et les tours posées utilisent cet atlas,
sans changer les collisions, les portées ou les cinq indicateurs d'amélioration.
Le build web copie le PNG dans `apps/web/dist/assets/towers/` et résout son chemin
depuis le module JavaScript, y compris sur les pages localisées.
Le Rust embarque le PNG dans l'exécutable : aucun fichier externe n'est nécessaire.

La [référence JPEG](assets/sprite_towers.jpeg) reste intacte. Pour régénérer
l'atlas après une modification de cette référence, avec Python et Pillow installés :

```sh
python3 assets/towers/build_atlas.py
npm run build
cargo build -p dedale-desktop --release --locked
```

L'extraction retire le fond sombre, les cadres et les annotations. Les coordonnées
sont propres à cette planche ; une nouvelle mise en page nécessite de les adapter.

### Ennemis

Les deux moteurs utilisent également [l'atlas des ennemis](assets/enemies/enemies.png),
référencé directement dans [ennemies.csv](ennemies.csv). Il contient une seule
ligne de dix sprites, dans l'ordre de [ennemies.csv](ennemies.csv) : Marcheurs,
Coureurs, Essaim, Volants, Blindés, Scindeurs, Éclats, Brutes, Intercepteurs et Colosse.
La correspondance utilise les identifiants, pas l'ordre des définitions du moteur.

La [référence JPEG](assets/sprite%20ennemis.jpeg) reste intacte. L'extraction supprime
le fond sombre, y compris au centre des formes creuses, et ajoute une marge transparente :

```sh
python3 assets/enemies/build_atlas.py
npm run build
cargo build -p dedale-desktop --release --locked
```

Le script nécessite Pillow et les dimensions actuelles de la référence (4128 × 1024).
La taille d'affichage reste proportionnelle au rayon de chaque ennemi ; les collisions,
les PV, les indicateurs d'effets et les barres de vie restent indépendants des sprites.
Le web copie le PNG dans `apps/web/dist/assets/enemies/` avec un chemin relatif au module,
et le Rust l'embarque dans l'exécutable, comme pour les tours.

Les Brutes et les Intercepteurs sont définis dans le CSV. Les vagues existantes ne
changent pas : une vague personnalisée peut utiliser `brute` ou `interceptor`.

### Modifier les caractéristiques et les sprites

[towers.csv](towers.csv) et [ennemies.csv](ennemies.csv) sont les seules sources des
caractéristiques et des associations aux sprites. Les deux builds les lisent
automatiquement : aucun import manuel ni copie des données dans un JSON n'est nécessaire.

| Colonne | Utilisation |
|---|---|
| `sprite file` | Chemin du PNG depuis la racine, sous `assets/` |
| `sprite row`, `sprite column` | Ligne et colonne **à partir de 1** |
| `sprite size` | Taille en pixels d'une case carrée du PNG (208 actuellement) |
| `draw size` (tours) | Taille d'affichage en pixels (40 actuellement) |
| `draw scale` (ennemis) | Taille d'affichage = rayon `r` × ce facteur (2.25 actuellement) |

Pour les tours, `sprite column` désigne la première de huit cases consécutives :
E, SE, S, SW, W, NW, N et NE. Pour les ennemis, elle désigne une seule case.
Par exemple, Caillou utilise la ligne 1 de `assets/towers/towers.png` ;
le Colosse utilise la ligne 1, colonne 10 de `assets/enemies/enemies.png`.
Changer la ligne ou la colonne réassocie le sprite sans régénérer le PNG.
Plusieurs types peuvent partager la même case ou utiliser des PNG différents.

Dans [towers.csv](towers.csv), chaque tour garde six lignes (niveaux 1 à 6).
Les colonnes communes `id`, `name`, `key`, `color`, `accent`, `desc`, `shot`,
`projectile speed`, `multi` et les colonnes de sprite se remplissent **uniquement
au niveau 1** ; elles restent vides aux autres niveaux. Les statistiques de combat
se remplissent à chaque niveau, et `earth`/`fly` doivent rester identiques sur les six lignes.
Les numéros `tower` sont consécutifs ; `id` est l'identifiant stable et `key` le raccourci.

Dans [ennemies.csv](ennemies.csv), chaque ligne définit un type entier.
`flying` et `immune` utilisent `0`/`1`. Les quatre colonnes `split ...` sont
toutes vides, ou toutes remplies ; `split type` référence l'`id` d'un autre ennemi.
Les nombres utilisent un point décimal ; les champs contenant une virgule
ou un saut de ligne doivent être entre guillemets (les tableurs le font automatiquement).

Après une modification, reconstruire et recharger la page web ou relancer le natif :

```sh
npm run build
cargo run -p dedale-desktop --release --locked
```

Il n'y a pas de rechargement à chaud. Une ligne invalide, une référence inconnue,
un PNG absent ou une case hors de l'image font échouer le build explicitement.

## Version native Rust

Le dossier [apps/desktop/](apps/desktop/) lance le moteur graphique natif
[packages/game-native/](packages/game-native/), écrit en Rust avec Macroquad.
Le même moteur est compilé pour Android par [apps/android/](apps/android/).
Ces applications ne contiennent ni HTML ni navigateur.

```sh
cargo run -p dedale-desktop --release --locked
```

Node.js est également requis à la compilation native : le build Rust appelle le même
lecteur/validateur de CSV que le web, afin d'éviter deux interprétations différentes.
Le jeu compilé ne dépend ni de Node.js ni des CSV/PNG externes.
La première compilation télécharge les dépendances Rust. Les tests de logique du
plateau et des vagues se lancent à la racine avec `cargo test --workspace --locked`.

## Structure

```
apps/
  web/                 # jeu navigateur, build.mjs, tests et src/
  desktop/             # point d'entrée natif pour ordinateur
  android/             # point d'entrée et métadonnées APK, guide Android
packages/
  content/
    content.mjs        # chargement et validation des données communes
    csv.mjs            # lecture des CSV
  game-native/
    build.rs           # génère les définitions Rust et embarque les médias
    src/lib.rs         # simulation/rendu partagés bureau et Android
infra/docker/Dockerfile # outils de compilation pour local/Jenkins
scripts/build-android.sh # compilation et vérification de signature APK
Jenkinsfile            # tests et builds Docker, archivage local à Jenkins
Cargo.toml             # workspace Rust
Cargo.lock             # dépendances Rust communes verrouillées
package.json           # workspace npm et commandes web à la racine
assets/                # sprites, références et FX partagés
game-content.json  # niveaux, vagues, plateau et règles communes uniquement
towers.csv         # caractéristiques, niveaux et sprites des tours
ennemies.csv       # caractéristiques et sprites des ennemis
```

## Contenu commun aux trois applications

[game-content.json](game-content.json) conserve uniquement la configuration générale :

- `board` : taille des cases, dimensions du plateau et cases ouvertes sur ses bords ;
- `rules` : durées des effets, dispersion des éclats et multiplicateurs de score ;
- `waveSets` : listes de vagues explicites, avec type d'ennemi, PV de base, multiplicateur de récompense et entrées ;
- `levels` : modes de jeu, avec vies, or, multiplicateur de PV, délai, nombre de vagues et référence à une liste de vagues.

Une vague utilise les statistiques de son ennemi. Les PV finaux sont `vague.hp × ennemi.hp × niveau.hp`, et la récompense est `ennemi.bounty × vague.bountyMultiplier`. Les listes actuelles conservent la progression précédente, sans formule dupliquée dans les moteurs.

Pour ajouter un niveau, ajouter une entrée à `levels`, par exemple :

```json
{
  "id": "entrainement",
  "name": "Entraînement",
  "hp": 0.5,
  "lives": 50,
  "gold": 150,
  "gap": 30,
  "waves": 10,
  "waveSet": "standard"
}
```

Pour créer des vagues personnalisées, ajouter une liste à `waveSets`, puis la référencer dans le niveau :

```json
[
  { "type": "walker", "hp": 16, "bountyMultiplier": 2, "targets": ["R"] },
  { "type": "runner", "hp": 25, "bountyMultiplier": 3, "targets": ["R", "B"] }
]
```

`R` désigne le trajet gauche → droite et `B` le trajet haut → bas. Les entrées d'une liste alternent **par ennemi**, pas par vague. Dès la vague 15, les listes actuelles utilisent les deux entrées. Le Rust est désormais aligné sur ce comportement du navigateur, ainsi que sur ses noms, couleurs, rayons et règles d'effets.

Les tours ont six niveaux, avec cinq coûts d'amélioration (`ups`) et six valeurs de
dégâts, portée, cadence, revente et effets (`dmg`, `range`, `rate`, `sell`, `special`,
`factor`). Ces tableaux sont générés automatiquement depuis [towers.csv](towers.csv).

Les coûts après le niveau 1 sont les coûts d'amélioration, pas les coûts cumulés.
`speed` dans le CSV des tours est le nombre de tirs par seconde ; `rate` dans les données
générées est l'intervalle en secondes. `projectile speed` est la vitesse du projectile.
Dans le CSV des ennemis, `speed` est la vitesse de déplacement.

Le plateau jouable contient exactement **13 × 11 tours carrées** (chaque tour occupe 2 × 2 cases de navigation de 20 pixels). L'intérieur mesure 520 × 440 pixels, avec une bordure de 20 pixels : le canvas complet mesure 560 × 480. Les portées du CSV sont des **diamètres**, convertis par `board.rangeScale` en rayons de simulation. Ainsi, 180 couvre toute la largeur jouable, soit un rayon de 260 pixels. Les ennemis gardent leur taille relative aux tours ; leur vitesse verticale est multipliée par 1,1 pour compenser la hauteur accrue et conserver le temps de traversée.

Le placement avance par **demi-tour** (20 pixels), sur les deux axes : deux tours peuvent être décalées d'une demi-case de tour pour créer des couloirs étroits, sans chevauchement. La grille fine, l'aperçu et le clic utilisent ce même pas ; les déplacements diagonaux ne traversent pas les coins des tours. Les ouvertures sont centrées : gauche/droite sur **3 tours de hauteur** (6 demi-cases, 120 pixels), haut/bas sur **4 tours de largeur** (8 demi-cases, 160 pixels). Le Colosse a un rayon de 9 pixels pour passer, comme les autres ennemis, dans les couloirs de 20 pixels.

La lecture des CSV ne réécrit jamais le JSON commun ni les images.

Tous les tirs ciblent un ennemi compatible sol/air. `spread` inflige `damage` à la cible et `factor` aux autres ennemis de la zone. `slow` a une probabilité `factor` par ennemi de diviser sa vitesse par deux pendant deux secondes. `stun` a une probabilité `factor` par ennemi de l'immobiliser pendant deux secondes. La zone est centrée sur la cible, de même rayon que la portée de la tour, et exclut les ennemis hors de portée de la tour. Les Blindés restent immunisés au ralentissement et à l'étourdissement.

Avant la première vague, la revente rend les dépenses intégrales ; ensuite, elle utilise la valeur exacte `sell` au niveau courant, y compris pour une tour posée avant le début de partie.

Les raccourcis doivent être uniques, de `1` à `9`. De nouvelles tours peuvent réutiliser les apparences de projectile existantes : `pellet`, `shell`, `dart`, `missile`, `ice`, `pulse`. Un comportement entièrement nouveau nécessite encore une implémentation dans chaque moteur : les CSV partagent les données, pas le code de simulation ou de rendu.

Après une modification du JSON ou des CSV, reconstruire les applications voulues :

```sh
npm run build
cargo build -p dedale-desktop --release --locked
bash scripts/build-android.sh debug
```

Le build web génère `apps/web/dist/content.js`, importé par toutes les pages, sans chargement
réseau supplémentaire de JSON ou de CSV. Le build Rust génère des constantes typées
dans le répertoire Cargo et se relance automatiquement si le JSON, les CSV, leur lecteur
ou un PNG référencé change. Les deux versions embarquent les données ; le Rust embarque
aussi les images. Les références inconnues et données invalides font échouer le build explicitement.

## Utilisation web

Aucune dépendance npm : seul Node.js est requis.

```sh
npm run build            # génère apps/web/dist/
python3 -m http.server -d apps/web/dist
```

Puis ouvrir l'adresse affichée dans le navigateur.

## Détails techniques

- JavaScript natif (modules ES) et `<canvas>`, sans framework.
- Le record par mode est stocké dans `localStorage` (clé `dd-best-<mode>`).
- `apps/web/dist/` et `target/` sont ignorés par git.
- La version Rust est indépendante de la version navigateur et se lance avec Cargo.

## Vérifications

Les tests couvrent les versions navigateur et native :

```sh
npm test
cargo test --workspace --locked
cargo fmt --all -- --check
```

Les tests comparent les niveaux et les vagues au JSON partagé, les tours/ennemis aux
données issues des CSV, et couvrent les routes, améliorations, remboursements, divisions
et effets. Les tests web modifient aussi les CSV dans un projet temporaire pour vérifier
les statistiques et associations de sprites du build, sans modifier la configuration générale.
