# Architecture

## Crate graph

```
                ┌──────────────────┐
                │  notation-tauri  │  Tauri 2 shell (commands, dialogs,
                │                  │  updater side effects, window)
                └────────┬─────────┘
        ┌──────────┬─────┴──────┬─────────────┐
        ▼          ▼            ▼             ▼
┌──────────────┐ ┌──────────┐ ┌───────────┐ ┌─────────────────┐
│ notation-ipc │ │notation- │ │ notation- │ │ notation-updater│
│ session      │ │storage   │ │ ui        │ │ GitHub Releases │
│ (pure logic) │ │.enbx     │ │ tokens    │ │ latest.json     │
└──────┬───────┘ └────┬─────┘ └───────────┘ └─────────────────┘
       │              │            ▲
       └── notation-core ──────────┘
          (model, history, smart shapes)
```

Dependency rules:

- `notation-core` has **no UI and no I/O dependencies** — pure domain logic,
  fully unit-tested. Everything else may depend on it.
- `notation-ipc` is the application session: tool state, gesture commits,
  page management, file operations, settings and updater bookkeeping as
  plain methods on `IpcApp`. It has **no GUI and no dialog dependencies**;
  the whole legacy Iced `update()` semantics live here as testable code.
- `notation-storage` owns every byte that touches the disk (documents,
  settings, recovery files).
- `notation-ui` is front-end agnostic: theme chrome as serializable color
  tokens consumed by the webview (CSS variables) via the `design_tokens`
  command.
- `notation-updater` is GUI-free and network-only.
- `notation-tauri` is a thin adapter: one Tauri command per interaction,
  delegating to `notation-ipc` under a mutex, plus native file dialogs
  (`tauri-plugin-dialog`) and the update-check/download/install side effects.

## Application model (Tauri)

- The webview front end (`ui/`, plain ES modules) renders the whiteboard on
  a `<canvas>` and owns *live gesture state* (in-progress strokes, figure
  drags, select-erase rectangles, element drags) with the same thresholds as
  the legacy prototype (4px tap→move, 3px figure commit).
- Committed interactions are delegated to Rust commands (`board_press`,
  `commit_stroke`, `commit_figure`, `erase_select`, `move_elements_delta`,
  `select_at`, …). The backend is the single source of truth.
- Every mutating command returns a full `Snapshot` (document, undo/redo
  flags, selection, tool and app settings, status, updater state, recovery
  offers); the front end re-renders from it. During drags the front end
  renders a local offset and resynchronizes on release.
- Page navigation mutates the active-page cursor only (no undo entry, no
  dirty flag); content changes go through `History<Document>` checkpoints,
  exactly like the legacy loop.

## Courseware format (`.enbx`)

A ZIP container mirroring the original's conceptual shape:

```
example.enbx (zip)
├── Manifest.json   # format marker + model version
└── Document.xml    # serialized notation-core Document (JSON body)
```

The schema is our own (`notation-storage` owns it); it is versioned via the
document's `version` field for forward evolution.

## Updater protocol (v0.2.0+)

1. Every release attaches a `latest.json` manifest next to the artifacts.
2. `notation-updater::check_manifest` fetches `releases/latest`, reads the
   manifest asset and compares its version with the running build.
3. On update it resolves the platform key (`windows-x86_64`) to an artifact
   URL + expected SHA-256.
4. `download_manifest_asset` streams the installer, verifies the digest and
   stages it.
5. The front end offers "Restart to install": the command spawns the staged
   NSIS installer and exits the app.

## CI / release pipeline

- **CI** (`ci.yml`) on every push/PR: rustfmt, clippy (`-D warnings`),
  tests and release builds on Linux and Windows (Linux installs the
  webkit2gtk 4.1 development stack).
- **Release** (`release.yml`) on `v*` tags: builds the Windows NSIS
  installer with the Tauri CLI, publishes `SHA256SUMS.txt` and
  `latest.json` alongside it, and creates the GitHub Release via `gh`.

## Clean-room boundary

Everything under `docs/re/` records observations of the original product only.
The source tree contains exclusively original code and artwork. Original
vendor behaviors that cannot be reproduced legally or technically (activation
servers, cloud resource bank, Office/Flash automation) are documented in
the RE report with explicit deltas.
