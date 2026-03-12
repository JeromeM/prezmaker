<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="PrezMaker" width="80" />
</p>

<h1 align="center">PrezMaker</h1>

<p align="center">
  Générateur de présentations BBCode pour les trackers
</p>

<p align="center">
  <a href="https://paypal.me/grommey"><img src="https://img.shields.io/badge/PayPal-Donation-0070ba?style=flat-square&logo=paypal&logoColor=white" alt="PayPal" /></a>
  <a href="https://www.buymeacoffee.com/grommey"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donation-ffdd00?style=flat-square&logo=buymeacoffee&logoColor=black" alt="Buy Me a Coffee" /></a>
</p>

---

Recherchez un film, une série, un jeu ou une application, et obtenez une présentation complète formatée en BBCode, prête à être collée sur un tracker.

## Fonctionnalités

- **Recherche automatique** — interroge TMDB (films/séries), IGDB et Steam (jeux) pour récupérer les métadonnées
- **Enrichissement Allociné** — récupère les notes Allociné en complément de TMDB
- **Import torrent** — glissez un `.torrent`, le titre est parsé automatiquement, le type de contenu détecté, les infos techniques pré-remplies
- **Éditeur de templates** — créez et personnalisez vos templates BBCode avec balises, conditionnels et aperçu en temps réel
- **Configuration requise** — tableau min/recommandée pour les jeux, récupéré automatiquement depuis Steam
- **Génération NFO** — générez un NFO via LLM à partir du BBCode produit
- **Description IA** — génération automatique de descriptions en français via LLM (Groq, Mistral, Gemini)
- **Mise à jour automatique** — vérification et installation des nouvelles versions au lancement
- **Couleur par template** — chaque template peut avoir sa propre couleur de titre
- **Paramètres organisés** — interface à onglets (Général, Clés API, IA/LLM)

## Screenshots

| Vue principale | Résultats de recherche |
|:-:|:-:|
| ![Main](screenshots/main.png) | ![Search](screenshots/search.png) |

| Présentation générée | Éditeur de templates |
|:-:|:-:|
| ![Preview](screenshots/preview.png) | ![Templates](screenshots/templates.png) |

| Formulaire jeu | Paramètres |
|:-:|:-:|
| ![Game](screenshots/game.png) | ![Settings](screenshots/settings.png) |

## Installation

Téléchargez la dernière version depuis la page [Releases](../../releases/latest) :

| Plateforme | Fichier |
|---|---|
| Windows | `.exe` (NSIS) ou `.msi` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb`, `.AppImage` ou `.rpm` |

L'application se met à jour automatiquement au lancement lorsqu'une nouvelle version est disponible.

## Configuration

Au premier lancement, allez dans les paramètres (icône engrenage) :

| Clé | Description | Obligatoire |
|---|---|:-:|
| **TMDB API Key** | Pour rechercher films et séries | Oui |
| **IGDB Client ID / Secret** | Pour rechercher des jeux | Oui (jeux) |
| **LLM Provider + API Key** | Pour les descriptions IA et la génération NFO | Non |
| **Couleur du titre** | Code couleur hex par défaut pour les titres BBCode | Non |

## Utilisation

1. Sélectionnez le type de contenu (Film, Série, Jeu, Application)
2. Tapez votre recherche ou importez un fichier `.torrent`
3. Sélectionnez le bon résultat
4. Complétez les informations supplémentaires si nécessaire (description, config requise, installation...)
5. La présentation BBCode est générée avec aperçu HTML en temps réel
6. Copiez le BBCode ou éditez-le directement

### Templates

Ouvrez l'éditeur de templates pour :
- Visualiser et modifier les templates par type de contenu
- Créer de nouveaux templates avec duplication
- Définir une couleur de titre personnalisée par template
- Utiliser des balises (`{{titre}}`, `{{synopsis}}`, `{{game_reqs_table}}`...)
- Ajouter des conditionnels (`{{#if synopsis}}...{{/if}}`)
- Voir l'aperçu en temps réel avec des données fictives

## Stack technique

| Composant | Technologie |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace : `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Steam, Allociné (scraping), Groq/Mistral/Gemini (LLM) |

## Build depuis les sources

### Prérequis

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/) : `cargo install tauri-cli --version "^2"`
- Dépendances système selon la plateforme ([voir la doc Tauri](https://v2.tauri.app/start/prerequisites/))

### Build

```bash
git clone https://github.com/JeromeM/prezmaker.git
cd prezmaker

# Installer les dépendances frontend
cd ui && npm install && cd ..

# Mode développement
cargo tauri dev

# Build production
cargo tauri build
```

Un `Makefile` est disponible avec des cibles par plateforme :

```bash
make deps-linux       # Installer les dépendances système Linux
make build-linux      # Build Linux
make build-windows    # Build Windows
make build-macos-arm  # Build macOS Apple Silicon
make build-macos-intel # Build macOS Intel
make dev              # Lancer en mode dev
```

## Licence

Ce projet est distribué sous licence MIT.
