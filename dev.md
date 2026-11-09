# dev.md — Environnement de développement Unity (Ubuntu)

Procédure complète, de zéro, pour développer et compiler **Dédale Défense (version Unity)** sur **Ubuntu 22.04 / 24.04 LTS (x86_64)**.
Le plan de migration détaillé est dans [todo.md](todo.md). Le projet Unity vivra dans `unity/DedaleDefense/`, la version web (`app/`) reste intacte tant que la parité n'est pas validée.

> Toutes les commandes sont à lancer dans un terminal. `sudo` est requis pour les installations système.
> Versions : Unity **6 LTS (6000.0.x)**. Le numéro exact du patch est fixé dans `unity/DedaleDefense/ProjectSettings/ProjectVersion.txt` une fois le projet créé ; installez toujours la version qui y figure.

---

## 1. Prérequis matériel et système

- Ubuntu 22.04 ou 24.04 (64 bits), session graphique (X11 ou Wayland) pour l'éditeur.
- 8 Go de RAM minimum (16 Go conseillés), ~25 Go de disque libre (éditeur + modules + `Library/`).
- GPU avec pilotes Vulkan ou OpenGL 4.5 (Mesa suffit).

```sh
lsb_release -a                 # vérifier la version d'Ubuntu
uname -m                       # doit afficher x86_64
sudo apt update && sudo apt upgrade -y
```

## 2. Outils de base

```sh
sudo apt install -y build-essential git git-lfs curl wget gpg ca-certificates \
  unzip zip xz-utils software-properties-common
git lfs install
git config --global user.name  "Votre Nom"
git config --global user.email "vous@example.com"
```

## 3. Dépendances de compilation Linux (IL2CPP) et d'exécution

IL2CPP convertit le C# en C++ puis le compile avec `clang`. Les bibliothèques ci-dessous sont aussi nécessaires pour exécuter l'éditeur et le player.

```sh
sudo apt install -y clang lld libc6-dev libstdc++-12-dev
# Bibliothèques d'exécution de l'éditeur / du player
sudo apt install -y libgtk-3-0 libnss3 libnspr4 libasound2t64 libxss1 libxtst6 \
  libxrandr2 libxcursor1 libxi6 libgbm1 libvulkan1 mesa-vulkan-drivers \
  libglu1-mesa libssl3 libcurl4
```

> Ubuntu 22.04 : remplacez `libasound2t64` par `libasound2`.

## 4. SDK .NET (outillage C#, tests hors Unity, formatage)

Le code de simulation (`DedaleDefense.Core`) est pur C# : il peut être testé avec `dotnet test` sans lancer Unity.

```sh
sudo apt install -y dotnet-sdk-8.0
dotnet --version
```

## 5. Unity Hub

```sh
wget -qO - https://hub.unity3d.com/linux/keys/public \
  | gpg --dearmor | sudo tee /usr/share/keyrings/Unity_Technologies_ApS.gpg > /dev/null
echo "deb [signed-by=/usr/share/keyrings/Unity_Technologies_ApS.gpg] https://hub.unity3d.com/linux/repos/deb stable main" \
  | sudo tee /etc/apt/sources.list.d/unityhub.list
sudo apt update
sudo apt install -y unityhub
```

Lancez `unityhub`, connectez-vous avec votre compte Unity et activez une licence **Unity Personal** (gratuite) : *Hub → Account → Manage licenses → Add → Get a free personal license*.

## 6. Éditeur Unity 6 + modules

Dans le Hub : *Installs → Install Editor → Unity 6 LTS (6000.0.x)* puis cochez :

- **Linux Build Support (IL2CPP)** — build natif Linux de release
- **Documentation** (optionnel)

(Le backend Mono pour Linux est inclus avec l'éditeur Linux.)

Équivalent en ligne de commande (la liste des versions et des changesets est sur https://unity.com/releases/editor/archive) :

```sh
unityhub -- --headless editors -r                 # versions disponibles
unityhub -- --headless install --version 6000.0.XX --changeset XXXXXXXXXXXX \
  --module linux-il2cpp
unityhub -- --headless editors -i                 # versions installées
```

Emplacement par défaut de l'éditeur : `~/Unity/Hub/Editor/<version>/Editor/Unity`.
Ajoutez un alias pour les scripts :

```sh
echo 'export UNITY_PATH="$HOME/Unity/Hub/Editor/6000.0.XX/Editor/Unity"' >> ~/.bashrc
source ~/.bashrc
```

## 7. IDE C#

**Option A — Visual Studio Code**

```sh
wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor \
  | sudo tee /usr/share/keyrings/microsoft.gpg > /dev/null
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/code stable main" \
  | sudo tee /etc/apt/sources.list.d/vscode.list
sudo apt update && sudo apt install -y code
code --install-extension ms-dotnettools.csdevkit
code --install-extension visualstudiotoolsforunity.vstuc
```

**Option B — JetBrains Rider** : `sudo snap install rider --classic`.

Dans Unity : *Edit → Preferences → External Tools → External Script Editor* = VS Code ou Rider, puis *Regenerate project files*.

## 8. Récupérer le dépôt

```sh
git clone https://github.com/clem2k/dedale-defense.git
cd dedale-defense
git lfs pull
git checkout -b ma-branche
```

## 9. Ouvrir / créer le projet Unity

Projet existant : Hub → *Add → Add project from disk* → `unity/DedaleDefense`.
Ouverture en ligne de commande :

```sh
"$UNITY_PATH" -projectPath unity/DedaleDefense
```

Réglages à vérifier (déjà versionnés dans `ProjectSettings/`) : *Asset Serialization = Force Text*, *Version Control = Visible Meta Files*, `Time.fixedDeltaTime = 1/60`.

## 10. Tests

La simulation (`unity/DedaleDefense/Assets/_Project/Scripts/Core`) est du C# pur, sans UnityEngine. Les tests xUnit compilent ces mêmes sources, sans lancer Unity :

```sh
dotnet test unity/Core.Tests
```

Ils couvrent les données (modes, formule de vagues), le placement (jamais de blocage du dernier passage), la revente/amélioration, le déterminisme à graine fixe et une partie complète de 50 vagues.

## 11. Build natif Linux

```sh
export UNITY_PATH="$HOME/Unity/Hub/Editor/6000.0.XX/Editor/Unity"   # voir §6
./unity/build-linux.sh              # backend Mono, build rapide
./unity/build-linux.sh --il2cpp     # backend IL2CPP (release)
./unity/Build/Linux/DedaleDefense.x86_64
```

Le script lance Unity en `-batchmode -nographics -buildTarget Linux64 -executeMethod BuildScript.BuildLinux`. Le journal est dans `unity/Build/build.log`. La scène de lancement est générée automatiquement (le jeu s'instancie via `RuntimeInitializeOnLoadMethod`).
Sans écran pour **lancer** le jeu (serveur) : `sudo apt install -y xvfb` puis `xvfb-run -a ./unity/Build/Linux/DedaleDefense.x86_64`.

Premier import : Unity génère les fichiers `.meta` ; committez-les (ils sont versionnés).

## 12. Dépannage

| Symptôme | Solution |
|----------|----------|
| Le Hub ne démarre pas (`libfuse`/sandbox) | `sudo apt install -y libfuse2t64` ; en dernier recours `unityhub --no-sandbox` |
| `No usable version of libssl` | `sudo apt install -y libssl3` ; sur 24.04 un lien vers `libssl1.1` n'est pas nécessaire avec Unity 6 |
| Build IL2CPP : `clang: not found` | `sudo apt install -y clang lld` |
| Licence refusée en batchmode | activer d'abord la licence via le Hub (GUI), ou fournir `-serial/-username/-password` (licence Pro) |
| Éditeur noir / crash Vulkan | lancer avec `-force-glcore` |
| Fichiers `.meta` en conflit | ne jamais les supprimer ; résoudre le conflit en gardant le GUID existant |

## 13. Intégration continue

`.github/workflows/unity.yml` :
- **core-tests** : `dotnet test` à chaque push/PR touchant `unity/` (aucun secret requis).
- **build-linux** : build Unity via [GameCI](https://game.ci), désactivé par défaut. Pour l'activer : créer la variable de dépôt `UNITY_BUILD=true` et les secrets `UNITY_LICENSE` (fichier `.ulf` de licence personnelle, voir la doc GameCI « activation »), `UNITY_EMAIL`, `UNITY_PASSWORD`. L'exécutable est publié en artefact `DedaleDefense-Linux`.
