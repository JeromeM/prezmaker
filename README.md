<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="PrezMaker" width="80" />
</p>

<h1 align="center">PrezMaker</h1>

<p align="center">
  Generateur de presentations BBCode pour les trackers
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/JeromeM/prezmaker?style=flat-square&color=blue" alt="Latest Release" /></a>
  <a href="../../actions/workflows/release.yml"><img src="https://img.shields.io/github/actions/workflow/status/JeromeM/prezmaker/release.yml?style=flat-square&label=build" alt="Build Status" /></a>
  <a href="../../releases"><img src="https://img.shields.io/github/downloads/JeromeM/prezmaker/total?style=flat-square&color=green" alt="Downloads" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/JeromeM/prezmaker?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="https://paypal.me/grommey"><img src="https://img.shields.io/badge/PayPal-Donation-0070ba?style=flat-square&logo=paypal&logoColor=white" alt="PayPal" /></a>
  <a href="https://www.buymeacoffee.com/grommey"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donation-ffdd00?style=flat-square&logo=buymeacoffee&logoColor=black" alt="Buy Me a Coffee" /></a>
</p>

---

Recherchez un film, une serie, un jeu ou une application, et obtenez une presentation complete formatee en BBCode, prete a etre collee sur un tracker.

## Fonctionnalites

- **Recherche automatique** — interroge TMDB (films/series), IGDB et Steam (jeux) pour recuperer les metadonnees
- **Enrichissement Allocine** — recupere les notes Allocine en complement de TMDB
- **Import torrent** — glissez un `.torrent`, le titre est parse automatiquement, le type de contenu detecte, les infos techniques pre-remplies
- **Editeur de templates** — creez et personnalisez vos templates BBCode avec balises, conditionnels et apercu en temps reel
- **Configuration requise** — tableau min/recommandee pour les jeux, directement integre aux templates
- **Generation NFO** — generez un NFO via LLM a partir du BBCode produit
- **Description IA** — generation automatique de descriptions en francais via LLM (OpenAI, Mistral)
- **Mise a jour automatique** — verification et installation des nouvelles versions au lancement

## Screenshots

| Vue principale | Resultats de recherche |
|:-:|:-:|
| ![Main](screenshots/main.png) | ![Search](screenshots/search.png) |

| Presentation generee | Editeur de templates |
|:-:|:-:|
| ![Preview](screenshots/preview.png) | ![Templates](screenshots/templates.png) |

| Formulaire jeu | Settings |
|:-:|:-:|
| ![Game](screenshots/game.png) | ![Settings](screenshots/settings.png) |

## Installation

Telechargez la derniere version depuis la page [Releases](../../releases/latest) :

| Plateforme | Fichier |
|---|---|
| Windows | `.exe` (NSIS) ou `.msi` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb`, `.AppImage` ou `.rpm` |

L'application se met a jour automatiquement au lancement lorsqu'une nouvelle version est disponible.

## Configuration

Au premier lancement, allez dans les parametres (icone engrenage) :

| Cle | Description | Obligatoire |
|---|---|:-:|
| **TMDB API Key** | Pour rechercher films et series | Oui |
| **IGDB Client ID / Secret** | Pour rechercher des jeux | Oui (jeux) |
| **LLM Provider + API Key** | Pour les descriptions IA et la generation NFO | Non |
| **Couleur du titre** | Code couleur hex pour les titres BBCode | Non |

## Utilisation

1. Selectionnez le type de contenu (Film, Serie, Jeu, Application)
2. Tapez votre recherche ou importez un fichier `.torrent`
3. Selectionnez le bon resultat
4. Completez les informations supplementaires si necessaire (description, config requise, installation...)
5. La presentation BBCode est generee avec apercu HTML en temps reel
6. Copiez le BBCode ou editez-le directement

### Templates

Ouvrez l'editeur de templates pour :
- Visualiser et modifier les templates par type de contenu
- Creer de nouveaux templates avec duplication
- Utiliser des balises (`{{titre}}`, `{{synopsis}}`, `{{game_reqs_table}}`...)
- Ajouter des conditionnels (`{{#if synopsis}}...{{/if}}`)
- Voir l'apercu en temps reel avec des donnees fictives

## Stack technique

| Composant | Technologie |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace : `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Steam, Allocine (scraping), OpenAI/Mistral (LLM) |

## Build depuis les sources

### Prerequis

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/) : `cargo install tauri-cli --version "^2"`
- Dependances systeme selon la plateforme ([voir la doc Tauri](https://v2.tauri.app/start/prerequisites/))

### Build

```bash
git clone https://github.com/JeromeM/prezmaker.git
cd prezmaker

# Installer les dependances frontend
cd ui && npm install && cd ..

# Mode developpement
cargo tauri dev

# Build production
cargo tauri build
```

Un `Makefile` est disponible avec des cibles par plateforme :

```bash
make deps-linux       # Installer les dependances systeme Linux
make build-linux      # Build Linux
make build-windows    # Build Windows
make build-macos-arm  # Build macOS Apple Silicon
make build-macos-intel # Build macOS Intel
make dev              # Lancer en mode dev
```

## Licence

Ce projet est distribue sous licence MIT.
