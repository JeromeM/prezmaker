# PrezMaker

Générateur de présentations BBCode pour les trackers. Recherchez un film, une série, un jeu ou une application, et obtenez une présentation complète formatée en BBCode, prête à être collée sur un tracker.

## Fonctionnalités

- **Recherche automatique** : interroge TMDB (films/séries), IGDB (jeux) pour récupérer les métadonnées
- **Enrichissement Allocine** : récupère les notes Allocine en plus de TMDB
- **Import torrent** : glissez un `.torrent`, le titre est parsé automatiquement, le type de contenu détecté (film, série, jeu), et les infos techniques pré-remplies
- **Éditeur de templates** : créez et personnalisez vos templates BBCode avec des balises `{{titre}}`, `{{synopsis}}`, etc. Aperçu en temps réel
- **Multi-templates** : plusieurs templates par type de contenu (film, série, jeu, app)
- **Génération NFO** : générez un NFO via LLM à partir du BBCode produit
- **Applications** : créez des présentations pour des logiciels avec infos techniques
- **Description IA** : génération automatique de descriptions en français via LLM (OpenAI, Mistral)

## Screenshots

<!-- Ajoutez vos captures d'écran ici -->

| Vue principale | Résultats de recherche |
|:-:|:-:|
| ![Main](screenshots/main.png) | ![Search](screenshots/search.png) |

| Présentation générée | Éditeur de templates |
|:-:|:-:|
| ![Preview](screenshots/preview.png) | ![Templates](screenshots/templates.png) |

| Formulaire jeu | Settings |
|:-:|:-:|
| ![Game](screenshots/game.png) | ![Settings](screenshots/settings.png) |

## Installation

Téléchargez la dernière version depuis la page [Releases](../../releases) :

| Plateforme | Fichier |
|---|---|
| Windows | `.msi` ou `.exe` |
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Linux | `.deb` ou `.AppImage` |

## Configuration

Au premier lancement, allez dans les paramètres (icône engrenage) pour configurer :

| Clé | Description | Obligatoire |
|---|---|:-:|
| **TMDB API Key** | Pour rechercher films et séries | Oui |
| **IGDB Client ID / Secret** | Pour rechercher des jeux | Oui (jeux) |
| **LLM Provider + API Key** | Pour les descriptions IA et la génération NFO | Non |
| **Couleur du titre** | Code couleur hex pour les titres BBCode | Non |

## Utilisation

1. Sélectionnez le type de contenu (Film, Série, Jeu, Application)
2. Tapez votre recherche ou importez un fichier `.torrent`
3. Sélectionnez le bon résultat
4. La présentation BBCode est générée avec aperçu HTML en temps réel
5. Copiez le BBCode (`Ctrl+C`) ou éditez-le directement

### Templates

Ouvrez l'éditeur de templates (icône document dans la barre) pour :
- Visualiser et modifier les templates par type de contenu
- Créer de nouveaux templates à partir du template par défaut
- Utiliser des balises comme `{{titre}}`, `{{synopsis}}`, `{{ratings_table}}`
- Voir l'aperçu en temps réel avec des données fictives

## Stack technique

| Composant | Technologie |
|---|---|
| GUI | [Tauri v2](https://v2.tauri.app/) |
| Frontend | React 19 + TypeScript + Tailwind CSS v4 + Vite |
| Backend | Rust (workspace : `prezmaker-lib`, `prezmaker-cli`, `src-tauri`) |
| APIs | TMDB, IGDB, Allocine (scraping), OpenAI/Mistral (LLM) |

## Build depuis les sources

### Prérequis

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- Dépendances système Tauri ([voir la doc](https://v2.tauri.app/start/prerequisites/))

### Étapes

```bash
# Cloner le repo
git clone https://github.com/JeromeM/prezmaker.git
cd prezmaker

# Installer les dépendances frontend
cd ui && npm install && cd ..

# Lancer en mode dev
cargo tauri dev

# Ou builder pour la production
cargo tauri build
```

## Licence

Ce projet est distribué sous licence MIT.

---

*Prez by Grommey*