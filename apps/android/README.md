# Android natif

Application Rust/Macroquad rendue en OpenGL ES, sans HTML, JavaScript embarqué,
WebView ou serveur. Le point d'entrée utilise directement `packages/game-native` :
simulation, modes, tours, sons et sprites sont identiques au bureau.
Le petit lanceur Java Android est généré par `cargo-quad-apk`.

Android 8.0 (API 26) minimum ; compilation/cible API 35. L'APK contient ARM64
(téléphones/tablettes) et x86_64 (émulateurs). Orientation paysage dans les deux sens,
mise à l'échelle proportionnelle et bandes de centrage selon l'écran.
Les données et les médias sont embarqués ; aucune permission réseau n'est demandée.

## Développement local

Prérequis : Rust 1.90+, Node.js 22+, JDK 17, `zip`, `unzip`, CMake,
pkg-config et OpenSSL de développement. Installer les Android command-line tools
puis accepter les licences Android avec `sdkmanager --licenses`.

```sh
export ANDROID_HOME=/chemin/vers/Android/Sdk
export NDK_HOME="$ANDROID_HOME/ndk/28.2.13676358"
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
sdkmanager "platform-tools" "platforms;android-35" "build-tools;35.0.0" "ndk;28.2.13676358"
rustup target add aarch64-linux-android x86_64-linux-android
cargo install --git https://github.com/not-fl3/cargo-quad-apk \
  --rev d411c8fe1c08339e46d7dc6b40ac04ea5e3a01b6 --locked
bash scripts/build-android.sh debug
```

Depuis la racine du monorepo. Utiliser **cargo quad-apk**, pas `cargo apk` :
Macroquad nécessite son lanceur Android spécifique.
La révision est figée : la version 0.1.4 publiée sur crates.io utilise un ancien
Cargo incompatible avec le workspace. Le petit point d'entrée Android reste en
édition Rust 2021 pour le code JNI injecté par Miniquad ; le moteur reste en 2024.
Le script vérifie la signature de l'APK produit :
`target/android-artifacts/debug/apk/dedale-android.apk`.

Avec un appareil en débogage USB ou un émulateur déjà lancé :

```sh
"$ANDROID_HOME/platform-tools/adb" install -r target/android-artifacts/debug/apk/dedale-android.apk
"$ANDROID_HOME/platform-tools/adb" shell am start -n fr.dedale.defense/.MainActivity
```

## Build Docker local / Jenkins

Depuis la racine, sur un hôte Docker Linux x86_64 :

```sh
docker build -f infra/docker/Dockerfile -t dedale-build .
docker run --rm --user "$(id -u):$(id -g)" \
  -e HOME=/workspace/.cache/home -e CARGO_HOME=/workspace/.cache/cargo \
  -v "$PWD:/workspace" dedale-build bash scripts/build-android.sh debug
```

La construction de l'image télécharge les outils officiels et accepte leurs licences.
L'APK reste dans `target/` sur l'hôte. Aucun service cloud de compilation n'est utilisé.
Le `Jenkinsfile` emploie la même image, teste web et moteur partagé, puis archive
le site web, l'exécutable Linux et l'APK release. Prérequis Jenkins : plugins
Pipeline/Docker Pipeline, agent Linux x86_64 étiqueté `docker` avec accès au démon Docker.
Configurer le job Pipeline depuis SCM sur ce dépôt ; aucun webhook n'est nécessaire.

`release` active les optimisations Rust mais utilise encore la **clé de débogage**
générée localement par le packager. C'est un APK de développement, pas une livraison
Play Store : signature de production, AAB et publication ne sont pas configurés.
Ne jamais versionner de keystore ni de mot de passe ; conserver une éventuelle clé
de distribution dans les credentials Jenkins et la monter uniquement pour signer.

## Commandes tactiles

Choisir un mode, toucher une tour dans le panneau, puis une position sur le plateau.
Toucher une tour posée pour l'améliorer ou la vendre. Toucher une seconde fois
le même type de tour, ou **Annuler**, quitte le placement.
**Vague suivante**, **Pause/Reprendre**, vitesse, **Annuler**, **Menu** et **Rejouer**
sont accessibles sans clavier. **Options** suspend la simulation et règle le son.

Pour modifier le jeu, travailler dans le moteur partagé ; pour modifier le paquet
(identifiant, versions, SDK, architectures), éditer `apps/android/Cargo.toml`.
Le bureau reste un point d'entrée distinct et conserve ses commandes clavier/souris.

## Vérification sur appareil

Après installation, vérifier le choix du mode, le placement aux bords du plateau,
les améliorations/reventes, les commandes tactiles, les options audio et la fin de
partie. Tester aussi les deux orientations paysage et un aller-retour en arrière-plan.
Les tests du workspace couvrent le moteur partagé et la conversion des coordonnées
tactiles ; ils ne remplacent pas ce contrôle sur téléphone ou émulateur.
