<p align="center">
  English | <a href="README.md">Fran&ccedil;ais</a>
</p>

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="PrezMaker" width="80" />
</p>

<h1 align="center">PrezMaker</h1>

<p align="center">
  <strong>BBCode presentation generator for trackers</strong>
</p>

<p align="center">
  <a href="../../releases/latest"><img src="https://img.shields.io/github/v/release/JeromeM/prezmaker?style=flat-square&label=Version&color=blue" alt="Release" /></a>
  <a href="../../releases/latest"><img src="https://img.shields.io/github/downloads/JeromeM/prezmaker/total?style=flat-square&label=Downloads&color=green" alt="Downloads" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/JeromeM/prezmaker?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="https://paypal.me/grommey"><img src="https://img.shields.io/badge/PayPal-Donate-0070ba?style=flat-square&logo=paypal&logoColor=white" alt="PayPal" /></a>
  <a href="https://www.buymeacoffee.com/grommey"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-ffdd00?style=flat-square&logo=buymeacoffee&logoColor=black" alt="Buy Me a Coffee" /></a>
</p>

---

Search for a movie, TV show, game or application, and get a complete BBCode-formatted presentation ready to paste on a tracker.

<p align="center">
  <img src="screenshots/main.png" alt="PrezMaker" width="800" />
</p>

## Features

### Search and import

- **Automatic search** — queries TMDB (movies/TV shows), IGDB and Steam (games) to retrieve metadata
- **Allocine enrichment** — fetches Allocine ratings alongside TMDB
- **Torrent import** — drag and drop a `.torrent` directly onto the app or click the home screen. Title is parsed automatically, content type detected, and technical info pre-filled
- **System requirements** — min/recommended specs table for games, fetched automatically from Steam

### Templates and editor

- **Template editor** — create and customize your BBCode templates with syntax highlighting, real-time preview and error tooltips
- **Conditional tags** — `{{#if synopsis}}...{{/if}}`, with comparison support (`>`, `<`, `==`, `!=`)
- **Formatting tags** — `{{heading:...}}`, `{{section:...}}`, `{{url:URL:label}}`, `{{br}}`, `{{center}}`, `{{bold}}`, etc.
- **Composite blocks** — `{{ratings_table}}`, `{{game_reqs_table}}`, `{{game_tech_table}}`, `{{screenshots_grid}}`
- **Favorite template** — set a default template per content type, automatically pre-selected
- **Per-template color** — each template can have its own title color
- **Export / Import** — share or back up your templates as JSON files
- **Tag search** — filter available tags in the sidebar
- **Tab / Shift+Tab** — indent and unindent in the editor

### AI and generation

- **AI descriptions** — automatic description generation in French via LLM (Groq, Mistral, Gemini)
- **NFO generation** — generate an NFO from the produced BBCode via LLM

### Interface

- **Light / Dark theme** — toggle between the two via the sun/moon button in the top bar
- **Automatic updates** — checks and installs new versions on launch, or manually from the About window
- **Window persistence** — window size and position are remembered between launches
- **Clickable links** — links in previews and in the app open in the system browser

## Screenshots

<table>
  <tr>
    <td align="center"><strong>Search results</strong></td>
    <td align="center"><strong>Generated presentation</strong></td>
  </tr>
  <tr>
    <td><img src="screenshots/search.png" alt="Search" /></td>
    <td><img src="screenshots/preview.png" alt="Preview" /></td>
  </tr>
  <tr>
    <td align="center"><strong>Template editor</strong></td>
    <td align="center"><strong>Game form</strong></td>
  </tr>
  <tr>
    <td><img src="screenshots/templates.png" alt="Templates" /></td>
    <td><img src="screenshots/game.png" alt="Game" /></td>
  </tr>
  <tr>
    <td align="center"><strong>Settings</strong></td>
    <td></td>
  </tr>
  <tr>
    <td><img src="screenshots/settings.png" alt="Settings" /></td>
    <td></td>
  </tr>
</table>

## Installation

Download the latest version from the [Releases](../../releases/latest) page:

| Platform | File |
|---|---|
| Windows | `.exe` (NSIS) or `.msi` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb`, `.AppImage` or `.rpm` |

The application updates automatically on launch when a new version is available.

## Configuration

On first launch, a setup wizard guides you through configuring the required API keys.

You can also access settings at any time via the gear icon:

| Key | Description | Required |
|---|---|:-:|
| **TMDB API Key** | To search for movies and TV shows | Yes |
| **IGDB Client ID / Secret** | To search for games | Yes (games) |
| **LLM Provider + API Key** | For AI descriptions and NFO generation | No |
| **Pseudo** | Signature in the presentation footer | No |
| **Title color** | Default hex color code for BBCode titles | No |

## Usage

1. Select the content type (Movie, TV Show, Game, Application)
2. Type your search or import a `.torrent` file (drag & drop or click)
3. Select the correct result
4. Fill in additional information if needed
5. The BBCode presentation is generated with a real-time HTML preview
6. Copy the BBCode or edit it directly

## Tech stack

| Component | Technology |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace: `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Steam, Allocine (scraping), Groq/Mistral/Gemini (LLM) |

## Build from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/): `cargo install tauri-cli --version "^2"`
- System dependencies per platform ([see Tauri docs](https://v2.tauri.app/start/prerequisites/))

### Build

```bash
git clone https://github.com/JeromeM/prezmaker.git
cd prezmaker

# Install frontend dependencies
cd ui && npm install && cd ..

# Development mode
cargo tauri dev

# Production build
cargo tauri build
```

A `Makefile` is available with per-platform targets:

```bash
make deps-linux       # Install Linux system dependencies
make build-linux      # Build Linux
make build-windows    # Build Windows
make build-macos-arm  # Build macOS Apple Silicon
make build-macos-intel # Build macOS Intel
make dev              # Run in dev mode
```

## License

This project is distributed under the MIT License.
