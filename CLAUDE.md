# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands
- `pnpm tauri dev` — Start development server
- `pnpm tauri build` — Build production binary
- `pnpm build` — Frontend-only build (vue-tsc + vite)
- `pnpm dev` — Frontend-only dev server (port 1420)

## Architecture

### Overview
- **Frontend**: Vue 3 + TypeScript + Vite, all in a **single SFC** (`src/App.vue`, ~8500 lines)
- **Backend**: Rust/Tauri 2, single file (`src-tauri/src/lib.rs`, ~2270 lines)
- **Package manager**: pnpm (uses pnpm-workspace.yaml)
- **State persistence**: `localStorage` for all app state (profiles, settings, install history)

### App.vue Structure (~8500 lines)
- **Template** (~1600 lines): 6 views inside `<main class="app-shell">` — `overview`, `mods`, `logs`, `tools`, `profiles`, `settings`
- **Script setup** (~3100 lines): All reactive state (vue `ref`/`computed`), functions, event handlers
- **Style scoped** (~3800 lines): CSS variables + component styles, warm-toned retro farmhouse theme

### Key Tauri Commands (rust -> frontend bridge)
- `launch_game`, `open_folder`, `move_folder`, `write_text_file`
- `preview_zip_mods`, `install_zip_mods` (ZIP mod installation with conflict resolution)
- `download_zip_from_url` (URL ZIP with parallel chunked download)
- `install_latest_smapi` (SMAPI download and install)
- `download_nxm_file`, `test_nexus_api_key` (Nexus Mods integration)
- `register_nxm_protocol`, `read_startup_nxm_link`, `read_pending_nxm_link` (NXM protocol handler)
- `get_smapi_log_folder`, `read_latest_smapi_log` (SMAPI diagnostics)
- `open_url_in_browser`

### localStorage Keys
- `junimo-box-game-path`, `junimo-box-game-path-valid`, `junimo-box-stardew-exists`, etc.
- `junimo-box-profiles`, `junimo-box-current-profile-id`
- `junimo-box-install-history`, `junimo-box-nexus-api-key`
- `junimo-box-notice-dismissed-*`
- `junimo-box-zip-preview-conflict-mode`

### Color System (CSS Variables in style.css)
Background: `--bg-page: #f5efe3`, `--bg-surface: #fffaf0`, `--bg-card: #f6ead8`
Text: `--text-primary: #2d241b`, `--text-secondary: #7a6652`
Semantic: `--green-bg: #6fa85f`, `--danger-bg: #b9574f`, `--warning-bg: #f8e7c8`
Borders: `--border-subtle: rgba(92,70,48,0.12)`, `--border-strong: rgba(92,70,48,0.22)`

### Storage Paths (Windows)
- App state: `%LOCALAPPDATA%\JunimoBox\`
- Lock file: `%LOCALAPPDATA%\JunimoBox\junimo-box.lock`
- Pending NXM link: `%LOCALAPPDATA%\JunimoBox\pending-nxm-link.txt`
- Game temp: `<game>\Junimo Box Temp\`
- Game downloads: `<game>\Junimo Box Downloads\`
- Deleted mods: `<game>\Junimo Box Deleted Mods\`

### Version Roadmap (from project goals doc)
- v0.7 (current): Visual unification + Profiles hardening + NXM fixes
- v0.8: Download & install flow (NXM injection, download queue, install pre-check)
- v0.9: Backup/restore/config hardening (restore points, rollback, recycle bin)
- v1.0: Stable local mod manager ready for daily use
- v2.0+: Mod repository, one-click install, conflict detection, cloud sync

### Important Design Principles
- All UI in a single SFC (App.vue) — do not split into components without explicit request
- All Rust commands in a single file (lib.rs) — do not split into modules
- Prefer `localStorage` for persistence, avoid file-based storage on frontend
- Use Chinese (Simplified) for all user-facing strings and comments
- Warm retro farmhouse aesthetic (browns, creams, sage green)
- All color values should use CSS variables with fallbacks: `var(--name, #original)`
