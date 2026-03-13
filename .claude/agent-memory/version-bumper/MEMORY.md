# Version Bumper - Mémoire

## Version actuelle
- **1.4.0** (depuis feature release majeure : conditionals imbriquées, opérateurs de comparaison, indentation, syntax highlighting, color pickers, etc.)

## Fichiers de version à maintenir synchronisés
1. `/apps/perso/PrezMaker/prezmaker-lib/Cargo.toml` (source de vérité)
2. `/apps/perso/PrezMaker/prezmaker-cli/Cargo.toml`
3. `/apps/perso/PrezMaker/src-tauri/Cargo.toml`
4. `/apps/perso/PrezMaker/ui/package.json`
5. `/apps/perso/PrezMaker/src-tauri/tauri.conf.json`

## Historique des versions
- 1.1.0 → 1.1.1 (2026-03-10) : bugfix Steam search retry en anglais pour certaines locales
- 1.3.0 → 1.4.0 (2026-03-11) : feature release majeure (11 fonctionnalités + 10 tests)

## Règles appliquées
- **Feature terminée** : MINOR++ (PATCH→0)
- **Bug fix** : PATCH++
- **WIP** : pas de changement
- **Feature + Fix ensemble** : MINOR++, PATCH++ (car mix)
- **Plusieurs features** : MINOR++ seul
- **Plusieurs fixes** : PATCH++ seul
