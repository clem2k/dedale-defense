# Dédale Défense

Jeu de tower defense dans le navigateur, inspiré de *Desktop Tower Defense* : les ennemis traversent le plateau, et ce sont vos tours qui forment le labyrinthe. Une tour ne peut jamais fermer le dernier passage.

Voir [user.md](user.md) pour le guide du joueur.

## Structure

```
app/
  build.mjs        # copie le jeu dans app/dist/ et génère les pages (/, en/, es/, de/, fr/, id/)
  package.json
  src/
    index.html     # page du jeu ({{BASE}} est remplacé au build)
    game.js        # logique, rendu canvas, interface
    style.css
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

## Version Unity (en cours)

Une refonte en Unity (build natif Linux) vit dans [`unity/`](unity/) ; la version web ci-dessus reste la référence de comportement.

- `unity/DedaleDefense/Assets/_Project/Scripts/Core` : simulation en C# pur (port de `game.js`), testée avec `dotnet test unity/Core.Tests`.
- `unity/DedaleDefense/Assets/_Project/Scripts/App` : rendu et entrées (IMGUI, sans asset externe).
- Mise en place de l'environnement et build : voir [dev.md](dev.md). Plan complet : [todo.md](todo.md).
