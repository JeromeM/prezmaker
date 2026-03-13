param(
    [Parameter(Position=0)]
    [ValidateSet("dev", "build", "build-signed", "check", "clean")]
    [string]$Command
)

$ErrorActionPreference = "Stop"

function Install-Frontend {
    Write-Host "[*] Installation des dependances frontend..." -ForegroundColor Cyan
    Push-Location ui
    npm install
    Pop-Location
}

switch ($Command) {
    "dev" {
        Install-Frontend
        Write-Host "[*] Lancement en mode dev..." -ForegroundColor Cyan
        cargo tauri dev
    }
    "build" {
        Install-Frontend
        Write-Host "[*] Build PrezMaker..." -ForegroundColor Cyan
        cargo tauri build
    }
    "build-signed" {
        if (-not $env:TAURI_SIGNING_PRIVATE_KEY) {
            Write-Host "[!] TAURI_SIGNING_PRIVATE_KEY non definie." -ForegroundColor Red
            Write-Host '    $env:TAURI_SIGNING_PRIVATE_KEY = "votre_cle"'
            exit 1
        }
        Install-Frontend
        Write-Host "[*] Build PrezMaker (signe)..." -ForegroundColor Cyan
        cargo tauri build
    }
    "check" {
        Write-Host "[*] Verification Rust..." -ForegroundColor Cyan
        cargo check --manifest-path src-tauri/Cargo.toml
        Write-Host "[*] Verification TypeScript..." -ForegroundColor Cyan
        Push-Location ui
        npx tsc --noEmit
        Pop-Location
    }
    "clean" {
        Write-Host "[*] Nettoyage..." -ForegroundColor Cyan
        cargo clean
        if (Test-Path ui/dist) { Remove-Item -Recurse -Force ui/dist }
        if (Test-Path ui/node_modules/.vite) { Remove-Item -Recurse -Force ui/node_modules/.vite }
    }
    default {
        Write-Host "Usage: .\make.ps1 <commande>" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Commandes:"
        Write-Host "  dev           Lancer en mode developpement"
        Write-Host "  build         Build production"
        Write-Host "  build-signed  Build avec signature updater"
        Write-Host "  check         Cargo check + TypeScript check"
        Write-Host "  clean         Nettoyer les artefacts de build"
    }
}
