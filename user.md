# Guide du joueur — Dédale Défense

## But du jeu

Tenez toutes les vagues du mode choisi sans perdre toutes vos vies. Chaque ennemi qui atteint la sortie coûte une vie (deux pour un Colosse). Posez des tours pour les arrêter et allongez leur chemin avec un labyrinthe.

## Modes

| Mode         | Vagues | Entre vagues | Vies | Or de départ | Ennemis |
|--------------|--------|--------------|------|--------------|---------|
| Facile       | 30     | 28 s         | 30   | 70           | PV ×0,75 |
| Normal       | 50     | 24 s         | 20   | 60           | PV ×1 |
| Difficile    | 60     | 20 s         | 10   | 50           | PV ×1,35 |
| Oncle Picsou | 40     | 24 s         | 20   | 10 000       | PV ×1,3 |
| Ruée         | 20     | 10 s         | 15   | 60           | PV ×1,1 |
| Marathon     | 70     | 30 s         | 30   | 80           | PV ×0,85, croissance plus lente |

Sur le web, votre record est enregistré par mode dans ce navigateur.

## Android

L'application Android est native, fonctionne hors ligne et se joue en paysage.
Choisir un mode, toucher une tour dans le panneau, puis le plateau pour la poser.
Toucher une tour posée ouvre ses commandes d'amélioration et de vente.
Les boutons sous le plateau permettent d'envoyer une vague, de mettre en pause,
de changer la vitesse, d'annuler le placement et de revenir au menu.
Après la fin d'une partie, **Rejouer** relance le même mode.
Les mêmes modes, règles, sprites et sons que sur ordinateur sont utilisés.

## Son et options

Dans les versions web, bureau et Android, **Options** ouvre le réglage du volume des effets
sonores (0 à 100 %, 50 % au départ). La partie est suspendue pendant les réglages ;
**Fermer** ou **Échap** revient au jeu, sans annuler une pause déjà activée.
Dans la version native, le curseur se déplace aussi avec les flèches gauche/droite.

Le bouton **Mute OFF / Mute ON** dans l'interface principale coupe ou réactive les
sons immédiatement, sans perdre le volume choisi. Les réglages sont conservés entre
les parties tant que le jeu reste ouvert. Chaque tour possède son son de tir ;
un son distinct signale la perte de vies. Sur le web, les sons commencent après
la première interaction avec la page.

## Raccourcis

| Touche | Action |
|--------|--------|
| 1 – 6 | Choisir une tour |
| 0 / Échap | Annuler |
| U | Améliorer la tour sélectionnée |
| S | Vendre la tour sélectionnée |
| Maj (maintenue) | Poser plusieurs tours d'affilée |
| Espace | Pause |
| N | Vague suivante |
| F | Changer la vitesse |

## Les tours

| # | Tour | Coût | Portée | Cible | Particularité |
|---|------|------|--------|-------|---------------|
| 1 | Caillou | 5 | 60 → 180 | sol + air | 10 → 400 dégâts |
| 2 | Éclat | 15 | 70 → 90 | sol + air | 4 tirs par seconde |
| 3 | Aiguille | 20 | 90 → 140 | sol | Dégâts de zone autour de la cible |
| 4 | Volée | 50 | 60 → 75 | air | 3 puis 4 tirs par seconde |
| 5 | Givre | 50 | 50 → 75 | sol + air | 75 %, puis 100 % de chance de ralentir |
| 6 | Onde | 30 | 40 | sol | 10 % → 90 % de chance d'étourdir |

Les tours s'améliorent 5 fois, jusqu'au niveau 6 (touche U ou panneau d'info).

La grille accueille 13 tours en largeur et 11 en hauteur. La portée indiquée est le diamètre du cercle visible quand une tour est sélectionnée : 180 couvre toute la largeur de la grille.

Une tour peut être déplacée d'une **demi-tour** à chaque pas, horizontalement ou verticalement, pour former des petits couloirs et des décalages diagonaux. Elle ne peut pas chevaucher une autre tour. Les ouvertures centrées à gauche/droite mesurent **3 tours de haut** ; celles du haut/bas mesurent **4 tours de large**.

Les effets de zone touchent les ennemis compatibles autour de la cible, uniquement s'ils sont également à portée de la tour. Givre divise leur vitesse par deux pendant deux secondes ; Onde les arrête pendant deux secondes. La probabilité est tirée séparément pour chaque ennemi. Aiguille inflige ses dégâts complets à sa cible et des dégâts de zone distincts aux voisins.

## Les ennemis

Marcheurs, Coureurs (rapides), Essaim (nombreux), Volants (survolent les tours), Blindés (insensibles au givre et au choc), Scindeurs (se divisent en deux) et le Colosse (boss). Le panneau « Prochaines vagues » montre ce qui arrive.

## Conseils

- Le message de construction disparaît après cinq secondes ou dès le premier clic dans la partie, sans bloquer le placement.
- Une tour qui fermerait le dernier passage est refusée : allongez le chemin au lieu de le fermer.
- « Vague suivante » (N) envoie la vague immédiatement ; les secondes gagnées s'ajoutent au score.
- Vendre une tour rend toutes les dépenses avant la première vague, puis le prix de revente indiqué pour son niveau.
- Prévoyez des tours anti-air (Volée, Caillou, Éclat, Givre) : les Volants ignorent le labyrinthe.
