# Architecture

## Crate graph

```
                ┌─────────────┐
                │  notation   │  GUI binary (Iced)
                └──────┬──────┘
        ┌──────────┬───┴────┬─────────────┐
        ▼          ▼        ▼             ▼
┌──────────────┐ ┌────────┐ ┌──────────┐ ┌─────────────────┐
│ notation-ui  │ │storage │ │ core     │ │ notation-updater│
│ theme/styles │ │ .enbx  │ │ model    │ │ GitHub Releases │
└──────┬───────┘ └───┬────┘ └──────────┘ └─────────────────┘
       │             │         ▲
       └── iced ──────┴─ core ─┘
```

Dependency rules:

- `notation-core` has **no UI and no I/O dependencies** — pure domain logic,
  fully unit-tested. Everything else may depend on it.
- `notation-storage` owns every byte that touches the disk (documents,
  settings, recovery files).
- `notation-ui` is the only crate besides the app that depends on `iced`;
  it centralizes palette and widget styling so the GUI crate stays declarative.
- `notation-updater` is GUI-free and network-only; the app drives it through
  messages so it stays testable.

## Application model (Iced)

The app is a classic Elm-architecture loop:

- `Notation` — single state struct holding the active `Document`, a snapshot
  `History<Document>`, the active `Tool`, in-progress strokes/figures, and
  view toggles (page panel, start menu, dialogs).
- `Message` — one enum for every interaction (tool selection, canvas pointer
  events, page management, file operations, updater events).
- `update()` mutates state and pushes pre-mutation snapshots onto the history
  before any destructive change; `undo`/`redo` swap snapshots.
- `view()` renders: top bar, page panel (left), board canvas (center) and the
  bottom tool dock. The canvas is an `iced` `canvas::Program` that draws the
  background, elements and drafts; pointer input is captured by a `mouse_area`.

## Courseware format (`.enbx`)

A ZIP container mirroring the original's conceptual shape:

```
example.enbx (zip)
├── Document.xml    # serialized notation-core Document (pages, elements)
└── Thumb.png       # cover thumbnail rendered by the app (optional)
```

The XML schema is our own (`notation-storage` owns it); it is versioned via a
`version` field for forward evolution.

## Updater protocol

1. `notation-updater` fetches
   `https://api.github.com/repos/RohanDaCoder/Notation10/releases/latest`.
2. Compares the tag against the running version.
3. Selects the asset for the current target triple
   (`Notation10-<tag>-<target>.zip|tar.gz`).
4. Downloads it while reporting progress, then verifies the SHA-256 digest
   against the release's `SHA256SUMS.txt`.
5. Installs by swapping the executable (rename-away + replace + relaunch).

## CI / release pipeline

- **CI** (`ci.yml`) on every push/PR: rustfmt, clippy (`-D warnings`), tests
  and release builds on Linux and Windows.
- **Release** (`release.yml`) on `v*` tags: builds both targets, packages
  archives + `SHA256SUMS.txt`, and publishes the GitHub Release via `gh`.

## Clean-room boundary

Everything under `docs/re/` records observations of the original product only.
The source tree contains exclusively original code and artwork. Original
vendor behaviors that cannot be reproduced legally or technically (activation
servers, cloud resource bank, Office/Flash automation) are documented in the
RE report with explicit deltas.
