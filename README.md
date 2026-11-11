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
```

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
