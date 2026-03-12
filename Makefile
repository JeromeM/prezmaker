# PrezMaker - Build targets
# Usage:
#   make build-linux          Build for Linux (current machine)
#   make build-windows        Build for Windows (current machine)
#   make build-macos-arm      Build for macOS Apple Silicon
#   make build-macos-intel    Build for macOS Intel
#   make build-signed-*       Same but with updater signature
#   make deps-linux           Install Linux system dependencies
#   make deps-frontend        Install frontend npm packages
#   make clean                Clean all build artifacts
#   make check                Run cargo check + tsc

# --- Configuration ---

TAURI_CLI = cargo tauri
UI_DIR = ui
TAURI_DIR = src-tauri

# Signing env (set these or export before calling make build-signed-*)
TAURI_SIGNING_PRIVATE_KEY ?=
TAURI_SIGNING_PRIVATE_KEY_PASSWORD ?=

# --- Frontend dependencies ---

.PHONY: deps-frontend
deps-frontend:
	cd $(UI_DIR) && npm install

# --- System dependencies ---

.PHONY: deps-linux
deps-linux:
	sudo apt-get update
	sudo apt-get install -y \
		libwebkit2gtk-4.1-dev \
		libappindicator3-dev \
		librsvg2-dev \
		patchelf \
		libgtk-3-dev \
		libssl-dev

# --- Check / Lint ---

.PHONY: check
check:
	cargo check --manifest-path $(TAURI_DIR)/Cargo.toml
	cd $(UI_DIR) && npx tsc --noEmit

# --- Unsigned builds ---

.PHONY: build-linux
build-linux: deps-frontend
	$(TAURI_CLI) build

.PHONY: build-windows
build-windows: deps-frontend
	$(TAURI_CLI) build

.PHONY: build-macos-arm
build-macos-arm: deps-frontend
	rustup target add aarch64-apple-darwin
	$(TAURI_CLI) build --target aarch64-apple-darwin

.PHONY: build-macos-intel
build-macos-intel: deps-frontend
	rustup target add x86_64-apple-darwin
	$(TAURI_CLI) build --target x86_64-apple-darwin

# --- Signed builds (for updater) ---

.PHONY: build-signed-linux
build-signed-linux: deps-frontend
	TAURI_SIGNING_PRIVATE_KEY="$(TAURI_SIGNING_PRIVATE_KEY)" \
	TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$(TAURI_SIGNING_PRIVATE_KEY_PASSWORD)" \
	$(TAURI_CLI) build

.PHONY: build-signed-windows
build-signed-windows: deps-frontend
	TAURI_SIGNING_PRIVATE_KEY="$(TAURI_SIGNING_PRIVATE_KEY)" \
	TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$(TAURI_SIGNING_PRIVATE_KEY_PASSWORD)" \
	$(TAURI_CLI) build

.PHONY: build-signed-macos-arm
build-signed-macos-arm: deps-frontend
	rustup target add aarch64-apple-darwin
	TAURI_SIGNING_PRIVATE_KEY="$(TAURI_SIGNING_PRIVATE_KEY)" \
	TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$(TAURI_SIGNING_PRIVATE_KEY_PASSWORD)" \
	$(TAURI_CLI) build --target aarch64-apple-darwin

.PHONY: build-signed-macos-intel
build-signed-macos-intel: deps-frontend
	rustup target add x86_64-apple-darwin
	TAURI_SIGNING_PRIVATE_KEY="$(TAURI_SIGNING_PRIVATE_KEY)" \
	TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$(TAURI_SIGNING_PRIVATE_KEY_PASSWORD)" \
	$(TAURI_CLI) build --target x86_64-apple-darwin

# --- Dev ---

.PHONY: dev
dev: deps-frontend
	$(TAURI_CLI) dev

# --- Clean ---

.PHONY: clean
clean:
	cargo clean
	rm -rf $(UI_DIR)/dist $(UI_DIR)/node_modules/.vite
