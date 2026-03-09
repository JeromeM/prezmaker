# PrezMaker

Generateur de presentations BBCode pour les trackers. Recherchez un film, une serie, un jeu ou une application, et obtenez une presentation complete formatee en BBCode, prete a etre collee sur un tracker.

## Fonctionnalites

- **Recherche automatique** : interroge TMDB (films/series), IGDB (jeux) pour recuperer les metadonnees
- **Enrichissement Allocine** : recupere les notes Allocine en plus de TMDB
- **Import torrent** : glissez un `.torrent`, le titre est parse automatiquement, le type de contenu detecte (film, serie, jeu), et les infos techniques pre-remplies
- **Editeur de templates** : creez et personnalisez vos templates BBCode avec des balises `{{titre}}`, `{{synopsis}}`, etc. Apercu en temps reel
- **Multi-templates** : plusieurs templates par type de contenu (film, serie, jeu, app)
- **Generation NFO** : generez un NFO via LLM a partir du BBCode produit
- **Applications** : creez des presentations pour des logiciels avec infos techniques
- **Description IA** : generation automatique de descriptions en francais via LLM (OpenAI, Mistral)

## Screenshots

<!-- Ajoutez vos captures d'ecran ici -->

| Vue principale | Resultats de recherche |
|:-:|:-:|
| ![Main](screenshots/main.png) | ![Search](screenshots/search.png) |

| Presentation generee | Editeur de templates |
|:-:|:-:|
| ![Preview](screenshots/preview.png) | ![Templates](screenshots/templates.png) |

| Import torrent | Formulaire jeu |
|:-:|:-:|
| ![Torrent](screenshots/torrent.png) | ![Game](screenshots/game.png) |

> Pour ajouter vos images : creez un dossier `screenshots/` a la racine et placez-y vos captures PNG.

## Installation

Telechargez la derniere version depuis la page [Releases](../../releases) :

| Plateforme | Fichier |
|---|---|
| Windows | `.msi` ou `.exe` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb` ou `.AppImage` |

## Configuration

Au premier lancement, allez dans les parametres (icone engrenage) pour configurer :

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
4. La presentation BBCode est generee avec apercu HTML en temps reel
5. Copiez le BBCode (`Ctrl+C`) ou editez-le directement

### Templates

Ouvrez l'editeur de templates (icone document dans la barre) pour :
- Visualiser et modifier les templates par type de contenu
- Creer de nouveaux templates a partir du template par defaut
- Utiliser des balises comme `{{titre}}`, `{{synopsis}}`, `{{ratings_table}}`
- Voir l'apercu en temps reel avec des donnees fictives

## Stack technique

| Composant | Technologie |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace : `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Allocine (scraping), OpenAI/Mistral (LLM) |

## Build depuis les sources

### Prerequis

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- Dependances systeme Tauri ([voir la doc](https://v2.tauri.app/start/prerequisites/))

### Etapes

```bash
# Cloner le repo
git clone https://github.com/JeromeM/prezmaker.git
cd prezmaker

# Installer les dependances frontend
cd ui && npm install && cd ..

# Lancer en mode dev
cargo tauri dev

# Ou builder pour la production
cargo tauri build
```

## Licence

Ce projet est distribue sous licence MIT.

---

*Prez by Grommey*
