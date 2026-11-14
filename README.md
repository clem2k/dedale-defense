# Dédale Défense

Jeu de tower defense inspiré de *Desktop Tower Defense* : les ennemis traversent le plateau, et ce sont vos tours qui forment le labyrinthe. Une tour ne peut jamais fermer le dernier passage.

Voir [user.md](user.md) pour le guide du joueur.

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
  package.json
  src/
    index.html     # page du jeu ({{BASE}} est remplacé au build)
    game.js        # logique, rendu canvas, interface
    style.css
rust/
  Cargo.toml       # application native Rust
  src/main.rs      # simulation, interface et rendu natif
capture/, checks/  # captures et vérifications de la référence (kongregate.com)
craft.json         # dossier de fabrication : demande, statut, 7 rounds de vérification
feed.json          # journal chronologique de la construction (étapes, outils, vérifications)
usage.json         # temps de travail, jetons et coût de la construction
investigation.cany/# preuves content-addressées (SHA-256), brief, inconnus ouverts
```

Les deux versions partagent la même table de modes (6 modes : Facile, Normal, Difficile, Oncle Picsou, Ruée, Marathon — voir `MODES` dans `app/src/game.js` et `Mode` dans `rust/src/main.rs`). Le test `every_mode_matches_the_browser_build` vérifie que le Rust reste aligné sur le JavaScript.

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

`capture/` contient les photos et la lecture de la page de référence ; `checks/round-0` à `round-7` contiennent les photos du jeu reconstruit, comparées à cette référence à 1440, 768 et 390 px (score de similarité dans `craft.json`). Ces 8 rounds ont été produits avant l'ajout des modes Oncle Picsou, Ruée et Marathon : ils ne reflètent pas la build actuelle. La logique du plateau et des vagues est couverte par `cargo test` dans `rust/`.
