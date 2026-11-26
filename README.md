# Notation10

A clean-room re-implementation of the **Prestigio "Note"** interactive whiteboard
application (NoteSetup 3.1.4.3735 — a rebrand of Seewo/CVTE **EasiNote 5**) in
**Rust** with the **Tauri 2** GUI framework.

> **Clean-room notice.** This project was written purely from black-box
> observations of the shipped installer (file inventory, UI strings, config and
> data-format inspection — see `docs/re/REPORT.md`). No vendor code, binaries,
> or assets are contained in this repository; all artwork here is original.

## Status

Prototype — multi-page whiteboard canvas with the recovered 1:1 dock
layout (menu / tool / page clusters), pen modes (hard pen, highlighter,
brush, bamboo, laser, magic, texture), stroke/area erasers, the full figure
palette (plain shapes, line family, pseudo-3D solids, fill), slide roam with
zoom, start menu, element editor with font panel, page manager, settings,
crash recovery, desktop-mode trace toolbar, and the mini-tool suite
(calculator, clock, timer, calendar, ruler, spotlight, magnifier,
cyclopedia, side board). `.enbx` courseware persistence, light/dark theming
and a GitHub-Releases based in-app updater are wired through
GUI-agnostic crates with unit tests.

## Crate layout

| Crate | Role |
|---|---|
| `notation-tauri` | Tauri 2 shell: window, commands, dialogs, updater side effects |
| `notation-ipc` | Pure application session (tools, gestures, pages, files) — fully unit-tested |
| `notation-core` | Domain model: documents, pages, elements, history, smart shapes |
| `notation-storage` | `.enbx` container, settings, crash-recovery snapshots |
| `notation-ui` | Design tokens (theme palette as serializable data) |
| `notation-updater` | GitHub Releases update client with `latest.json` manifest + SHA-256 |

The front end (`ui/`) is a **Svelte 5 + TypeScript + Vite** app (compiled,
no runtime framework code on the board's hot path). It renders the
whiteboard on a canvas, mirrors the recovered design system
(`docs/re/theme.json` — 1080p-relative design units through the `--rp`
CSS scale) and talks to the Rust layer through Tauri commands. It also runs
standalone in a plain browser against a mock backend (`ui/src/lib/mock.ts`)
for development and verification.

## Building

```bash
# Prerequisites: Rust stable, Node.js 20+ (for the front end and Tauri CLI).
npm install -g @tauri-apps/cli@^2

# Bundle the front end first (the shell embeds ui/dist at compile time).
cd ui && npm ci && npm run build && cd ..

# Development (debug build with devtools-friendly webview).
npx tauri dev --config crates/notation-tauri/tauri.conf.json

# Release build (Windows produces the NSIS installer).
npx tauri build --config crates/notation-tauri/tauri.conf.json --bundles nsis

# Cross-check the non-GUI crates without any webview dependencies.
cargo test -p notation-core -p notation-ipc -p notation-storage \
  -p notation-ui -p notation-updater
```

On Linux you need the usual Tauri prerequisites (webkit2gtk 4.1, GTK3, RSVG).
On Windows the NSIS bundler bootstraps itself on first `tauri build`.

## Front-end harness (no native build required)

```bash
cd ui && npm ci
npm run dev        # Vite dev server with hot reload
npm run build      # bundle to ui/dist
npm run check      # svelte-check type pass
python3 -m http.server -d dist 8841
# open http://localhost:8841/ — the mock backend takes over.
```

## Releases and updates

Pushing a `v*` tag triggers `.github/workflows/release.yml`: it builds the
Windows NSIS installer, publishes it together with `SHA256SUMS.txt` and a
`latest.json` update manifest as a GitHub Release (via the `gh` CLI). The
in-app updater (Start menu → *Check for updates*) reads that manifest,
compares versions, downloads and SHA-256-verifies the installer and offers to
run it.

## License

MIT — see [LICENSE](LICENSE).
