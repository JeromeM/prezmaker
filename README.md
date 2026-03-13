<p align="center">
  <a href="README.en.md">English</a> | Fran&ccedil;ais
</p>

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="PrezMaker" width="80" />
</p>

<h1 align="center">PrezMaker</h1>

<p align="center">
  <strong>Generateur de presentations BBCode pour les trackers</strong>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/JeromeM/prezmaker?style=flat-square&label=Version&color=blue" alt="Release" /></a>
  <a href="../../releases/latest"><img src="https://img.shields.io/github/downloads/JeromeM/prezmaker/total?style=flat-square&label=Telechargements&color=green" alt="Downloads" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/JeromeM/prezmaker?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="https://paypal.me/grommey"><img src="https://img.shields.io/badge/PayPal-Donation-0070ba?style=flat-square&logo=paypal&logoColor=white" alt="PayPal" /></a>
  <a href="https://www.buymeacoffee.com/grommey"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donation-ffdd00?style=flat-square&logo=buymeacoffee&logoColor=black" alt="Buy Me a Coffee" /></a>
</p>

---

Recherchez un film, une serie, un jeu ou une application, et obtenez une presentation complete formatee en BBCode, prete a etre collee sur un tracker.

<p align="center">
  <img src="screenshots/main.png" alt="PrezMaker" width="800" />
</p>

## Fonctionnalites

### Recherche et import

- **Recherche automatique** — interroge TMDB (films/series), IGDB et Steam (jeux) pour recuperer les metadonnees
- **Enrichissement Allocine** — recupere les notes Allocine en complement de TMDB
- **Import torrent** — glissez un `.torrent` directement sur l'application ou cliquez sur l'ecran d'accueil. Le titre est parse automatiquement, le type de contenu detecte, et les infos techniques pre-remplies
- **Configuration requise** — tableau min/recommandee pour les jeux, recupere automatiquement depuis Steam

### Templates et editeur

- **Editeur de templates** — creez et personnalisez vos templates BBCode avec coloration syntaxique, apercu en temps reel et infobulles d'erreurs
- **Balises conditionnelles** — `{{#if synopsis}}...{{/if}}`, avec support des comparaisons (`>`, `<`, `==`, `!=`)
- **Balises de mise en forme** — `{{heading:...}}`, `{{section:...}}`, `{{url:URL:label}}`, `{{br}}`, `{{center}}`, `{{bold}}`, etc.
- **Blocs composites** — `{{ratings_table}}`, `{{game_reqs_table}}`, `{{game_tech_table}}`, `{{screenshots_grid}}`
- **Template favori** — definissez un template par defaut par type de contenu, pre-selectionne automatiquement
- **Couleur par template** — chaque template peut avoir sa propre couleur de titre
- **Export / Import** — partagez ou sauvegardez vos templates au format JSON
- **Recherche de balises** — filtrez les balises disponibles dans la sidebar
- **Tab / Shift+Tab** — indentation et desindentation dans l'editeur

### IA et generation

- **Description IA** — generation automatique de descriptions en francais via LLM (Groq, Mistral, Gemini)
- **Generation NFO** — generez un NFO a partir du BBCode produit via LLM

### Interface

- **Theme clair / sombre** — basculez entre les deux via le bouton soleil/lune dans la barre du haut
- **Mise a jour automatique** — verification et installation des nouvelles versions au lancement, ou manuellement depuis la fenetre A propos
- **Persistance de la fenetre** — la taille et la position de la fenetre sont memorisees entre les lancements
- **Liens cliquables** — les liens dans les previews et dans l'app s'ouvrent dans le navigateur systeme

## Screenshots

<table>
  <tr>
    <td align="center"><strong>Resultats de recherche</strong></td>
    <td align="center"><strong>Presentation generee</strong></td>
  </tr>
  <tr>
    <td><img src="screenshots/search.png" alt="Recherche" /></td>
    <td><img src="screenshots/preview.png" alt="Preview" /></td>
  </tr>
  <tr>
    <td align="center"><strong>Editeur de templates</strong></td>
    <td align="center"><strong>Formulaire jeu</strong></td>
  </tr>
  <tr>
    <td><img src="screenshots/templates.png" alt="Templates" /></td>
    <td><img src="screenshots/game.png" alt="Jeu" /></td>
  </tr>
  <tr>
    <td align="center"><strong>Parametres</strong></td>
    <td></td>
  </tr>
  <tr>
    <td><img src="screenshots/settings.png" alt="Parametres" /></td>
    <td></td>
  </tr>
</table>

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

Au premier lancement, un assistant vous guide pour configurer les cles API necessaires.

Vous pouvez aussi acceder aux parametres a tout moment via l'icone engrenage :

| Cle | Description | Obligatoire |
|---|---|:-:|
| **TMDB API Key** | Pour rechercher films et series | Oui |
| **IGDB Client ID / Secret** | Pour rechercher des jeux | Oui (jeux) |
| **LLM Provider + API Key** | Pour les descriptions IA et la generation NFO | Non |
| **Pseudo** | Signature dans le footer des presentations | Non |
| **Couleur du titre** | Code couleur hex par defaut pour les titres BBCode | Non |

## Utilisation

1. Selectionnez le type de contenu (Film, Serie, Jeu, Application)
2. Tapez votre recherche ou importez un fichier `.torrent` (drag & drop ou clic)
3. Selectionnez le bon resultat
4. Completez les informations supplementaires si necessaire
5. La presentation BBCode est generee avec apercu HTML en temps reel
6. Copiez le BBCode ou editez-le directement

## Stack technique

| Composant | Technologie |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace : `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Steam, Allocine (scraping), Groq/Mistral/Gemini (LLM) |

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
