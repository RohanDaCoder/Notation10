# Notation10

A clean-room re-implementation of the **Prestigio "Note"** interactive whiteboard
application (NoteSetup 3.1.4.3735 — a rebrand of Seewo/CVTE **EasiNote 5**) in
**Rust** with the **Iced** GUI framework.

> **Clean-room notice.** This project was written purely from black-box
> observations of the shipped installer (file inventory, UI strings, config and
> data-format inspection — see `docs/re/REPORT.md`). No vendor code, binaries,
> or assets are contained in this repository; all artwork here is original.

## Status

Prototype — multi-page whiteboard canvas with pen/shape/eraser/text tools,
page management, `.enbx` courseware persistence, theming, and a GitHub-Releases
based in-app updater.

## Building

```bash
cargo run -p notation     # run the app
cargo test --workspace    # unit + integration tests
cargo clippy --workspace --all-targets -- -D warnings
```

Linux builds need the usual GUI development packages (`libxkbcommon-dev`,
`libwayland-dev`, `libx11-dev`, …). Windows builds work out of the box with the
MSVC toolchain.

## Repository layout

| Crate | Purpose |
|---|---|
| `crates/notation` | The GUI application binary (Iced front-end) |
| `crates/notation-core` | Domain model: documents, pages, elements, history, tools |
| `crates/notation-storage` | `.enbx` courseware format, settings, crash recovery |
| `crates/notation-ui` | Shared theme, palette and widget styling |
| `crates/notation-updater` | Self-update client backed by GitHub Releases |

## Releases

Releases are **tag-based**: pushing a `v*` tag triggers `.github/workflows/release.yml`,
which builds optimized Windows and Linux binaries, packages them with
`SHA256SUMS.txt`, and publishes a GitHub Release. The in-app updater downloads
from those releases and verifies the SHA-256 checksum before installing.

```bash
git tag v0.1.0 && git push origin v0.1.0
```

## License

MIT — see [LICENSE](LICENSE).
