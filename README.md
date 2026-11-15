# Dédale Défense

Jeu de tower defense inspiré de *Desktop Tower Defense* : les ennemis traversent le plateau, et ce sont vos tours qui forment le labyrinthe. Une tour ne peut jamais fermer le dernier passage.

Voir [user.md](user.md) pour le guide du joueur.

## Sprites communs

Les deux moteurs utilisent le même [atlas PNG transparent](assets/towers/towers.png),
décrit par [assets/towers/atlas.json](assets/towers/atlas.json). Ses six lignes correspondent
à Caillou, Éclat, Aiguille, Volée, Givre et Onde ; ses huit colonnes sont
E, SE, S, SW, W, NW, N et NE. Les vues ne sont pas tournées par le moteur :
l'angle de visée sélectionne la direction la plus proche, avec un cadrage fixe
et une marge transparente contre les débordements entre images.

Le rendu sur le plateau reste dans une empreinte de 40 × 40 pixels.
Les boutons, les aperçus de placement et les tours posées utilisent cet atlas,
sans changer les collisions, les portées ou les cinq indicateurs d'amélioration.
Le build web copie le PNG dans `app/dist/assets/towers/` et résout son chemin
depuis le module JavaScript, y compris sur les pages localisées.
Le Rust embarque le PNG dans l'exécutable : aucun fichier externe n'est nécessaire.

La [référence JPEG](assets/sprite_towers.jpeg) reste intacte. Pour régénérer
l'atlas après une modification de cette référence, avec Python et Pillow installés :

```sh
python3 assets/towers/build_atlas.py
npm --prefix app run build
cargo build --release --manifest-path rust/Cargo.toml
```

L'extraction retire le fond sombre, les cadres et les annotations. Les coordonnées
sont propres à cette planche ; une nouvelle mise en page nécessite de les adapter.
Les futurs sprites peuvent être regroupés par famille sous `assets/` (par exemple `enemies/`).

## Version native Rust

Le dossier `rust/` contient une version graphique native du jeu, écrite en Rust avec Macroquad. Elle s'exécute comme une application de bureau et ne contient ni HTML ni navigateur.

```sh
cd rust
cargo run --release
```

Cette première compilation télécharge les dépendances Rust. Les tests de logique du plateau et des vagues se lancent avec `cargo test`.

## Structure

```
app/
  build.mjs        # copie le jeu dans app/dist/ et génère les pages (/, en/, es/, de/, fr/, id/)
  content.mjs      # chargement et validation des données communes
  content.test.mjs # tests des données, de la simulation et du build
  package.json
  src/
    index.html     # page du jeu ({{BASE}} est remplacé au build)
    game.js        # logique, rendu canvas, interface
    style.css
rust/
  Cargo.toml       # application native Rust
  build.rs         # valide le JSON et génère les définitions Rust à la compilation
  src/main.rs      # simulation, interface et rendu natif
game-content.json  # source unique des niveaux, vagues, tours, ennemis et règles communes
capture/, checks/  # captures et vérifications de la référence (kongregate.com)
craft.json         # dossier de fabrication : demande, statut, 7 rounds de vérification
feed.json          # journal chronologique de la construction (étapes, outils, vérifications)
usage.json         # temps de travail, jetons et coût de la construction
investigation.cany/# preuves content-addressées (SHA-256), brief, inconnus ouverts
```

## Contenu commun aux deux versions

[game-content.json](game-content.json) est la source unique du contenu et de l'équilibrage :

- `board` : taille des cases, dimensions du plateau et cases ouvertes sur ses bords ;
- `rules` : durées des effets, dispersion des éclats et multiplicateurs de score ;
- `towers` : identifiants, noms, raccourcis, couleurs, prix, améliorations, dégâts, cibles et effets ;
- `enemies` : PV relatifs, vitesse, apparence, nombre par vague, intervalle d'apparition, récompense, immunités, vies perdues et division ;
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

Les tours conservent leurs noms et ont six niveaux, avec cinq coûts d'amélioration (`ups`) et six valeurs de dégâts, portée, cadence, revente et effets (`dmg`, `range`, `rate`, `sell`, `special`, `factor`). Les caractéristiques proviennent de [towers.csv](towers.csv). Pour les réimporter :

```sh
npm --prefix app run import:towers
```

Les coûts après le niveau 1 sont les coûts d'amélioration, pas les coûts cumulés. `speed` dans le CSV est le nombre de tirs par seconde ; `rate` dans le JSON est l'intervalle en secondes. La vitesse du projectile est distincte.

Le plateau jouable contient exactement **13 × 11 tours carrées** (chaque tour occupe 2 × 2 cases de navigation de 20 pixels). L'intérieur mesure 520 × 440 pixels, avec une bordure de 20 pixels : le canvas complet mesure 560 × 480. Les portées du CSV sont des **diamètres**, convertis par `board.rangeScale` en rayons de simulation. Ainsi, 180 couvre toute la largeur jouable, soit un rayon de 260 pixels. Les ennemis gardent leur taille relative aux tours ; leur vitesse verticale est multipliée par 1,1 pour compenser la hauteur accrue et conserver le temps de traversée.

Le placement avance par **demi-tour** (20 pixels), sur les deux axes : deux tours peuvent être décalées d'une demi-case de tour pour créer des couloirs étroits, sans chevauchement. La grille fine, l'aperçu et le clic utilisent ce même pas ; les déplacements diagonaux ne traversent pas les coins des tours. Les ouvertures sont centrées : gauche/droite sur **3 tours de hauteur** (6 demi-cases, 120 pixels), haut/bas sur **4 tours de largeur** (8 demi-cases, 160 pixels). Le Colosse a un rayon de 9 pixels pour passer, comme les autres ennemis, dans les couloirs de 20 pixels.

L'import du CSV ne modifie ni `board` ni `enemies` : leurs proportions restent définies uniquement dans le JSON commun.

Tous les tirs ciblent un ennemi compatible sol/air. `spread` inflige `damage` à la cible et `factor` aux autres ennemis de la zone. `slow` a une probabilité `factor` par ennemi de diviser sa vitesse par deux pendant deux secondes. `stun` a une probabilité `factor` par ennemi de l'immobiliser pendant deux secondes. La zone est centrée sur la cible, de même rayon que la portée de la tour, et exclut les ennemis hors de portée de la tour. Les Blindés restent immunisés au ralentissement et à l'étourdissement.

Avant la première vague, la revente rend les dépenses intégrales ; ensuite, elle utilise la valeur exacte `sell` au niveau courant, y compris pour une tour posée avant le début de partie.

Les raccourcis doivent être uniques, de `1` à `9`. De nouvelles tours peuvent réutiliser les apparences de projectile existantes : `pellet`, `shell`, `dart`, `missile`, `ice`, `pulse`. Un comportement entièrement nouveau nécessite encore une implémentation dans chaque moteur : le JSON partage les données, pas le code de simulation ou de rendu.

Après une modification du JSON, reconstruire les deux versions :

```sh
npm --prefix app run build
cargo build --release --manifest-path rust/Cargo.toml
```

Le build web génère `app/dist/content.js`, importé par toutes les pages, sans chargement réseau supplémentaire de JSON. Le build Rust génère des constantes typées dans le répertoire Cargo et se relance automatiquement si le JSON change. Il ne dépend pas de Node.js. Les deux versions embarquent leur contenu et n'ont pas besoin du JSON à côté du jeu distribué. Les références inconnues et données invalides font échouer le build explicitement.

## Utilisation

Aucune dépendance : seul Node.js est requis.

```sh
cd app
npm run build            # génère app/dist/
npx serve dist           # ou : python3 -m http.server -d dist
```

Puis ouvrir l'adresse affichée dans le navigateur.

## Détails techniques

- JavaScript natif (modules ES) et `<canvas>`, sans framework.
- Le record par mode est stocké dans `localStorage` (clé `dd-best-<mode>`).
- `app/dist/` est ignoré par git.
- La version Rust est indépendante de la version navigateur et se lance avec Cargo.

## Vérifications

`capture/` contient les photos et la lecture de la page de référence ; `checks/round-0` à `round-7` contiennent les photos du jeu reconstruit, comparées à cette référence à 1440, 768 et 390 px (score de similarité dans `craft.json`). Ces 8 rounds ont été produits avant l'ajout des modes Oncle Picsou, Ruée et Marathon : ils ne reflètent pas la build actuelle.

```sh
npm --prefix app test
cargo test --manifest-path rust/Cargo.toml
```

Les tests comparent tous les niveaux et toutes leurs vagues au JSON partagé, contrôlent les tours/ennemis et couvrent les routes, les améliorations, les remboursements, la division et les effets. Les tests web vérifient aussi le contenu embarqué, les pages localisées, l'ajout d'un niveau et le rejet des données invalides.
