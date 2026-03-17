# Version Bumper - Mémoire

## Version actuelle
- **1.28.0** (depuis 2026-03-17 : feature release)

## Fichiers de version à maintenir synchronisés
1. `/apps/perso/PrezMaker/prezmaker-lib/Cargo.toml` (source de vérité)
2. `/apps/perso/PrezMaker/src-tauri/Cargo.toml`
3. `/apps/perso/PrezMaker/ui/package.json`
4. `/apps/perso/PrezMaker/src-tauri/tauri.conf.json`

Note: prezmaker-cli n'existe plus

## Historique des versions
- 1.1.0 → 1.1.1 (2026-03-10) : bugfix Steam search retry en anglais pour certaines locales
- 1.3.0 → 1.4.0 (2026-03-11) : feature release majeure (11 fonctionnalités + 10 tests)
- 1.20.0 → 1.21.0 (2026-03-14) : feature release (module d'upload C411)
- 1.21.2 → 1.22.0 (2026-03-15) : feature release MINOR bump
- 1.23.0 → 1.23.1 (2026-03-15) : bugfix NFO persistence, reload, game NFO
- 1.23.1 → 1.24.0 (2026-03-15) : feature release (60 tests, scan progress, NFO regeneration)
- 1.24.0 → 1.25.0 (2026-03-16) : feature release (HTML output)
- 1.25.0 → 1.25.1 (2026-03-16) : bugfix HTML tags
- 1.25.1 → 1.26.0 (2026-03-16) : feature release
- 1.26.0 → 1.26.1 (2026-03-16) : bugfix HTML preview parsing
- 1.26.1 → 1.26.2 (2026-03-16) : patch bump
- 1.27.1 → 1.28.0 (2026-03-17) : feature release (manual torrent search + link torrent)

## Règles appliquées
- **Feature terminée** : MINOR++ (PATCH→0)
- **Bug fix** : PATCH++
- **WIP** : pas de changement
- **Feature + Fix ensemble** : MINOR++, PATCH++ (car mix)
- **Plusieurs features** : MINOR++ seul
- **Plusieurs fixes** : PATCH++ seul

Notes:
- Tous les 5 fichiers de version doivent rester synchronisés
- Ne jamais modifier MAJOR sans instruction explicite
