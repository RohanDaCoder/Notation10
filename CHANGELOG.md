# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
tag-based semantic version releases.

## [Unreleased]

### Added

- Clean-room reverse-engineering report (`docs/re/REPORT.md`) and roadmap.
- Multi-crate Cargo workspace scaffold (`notation`, `notation-core`,
  `notation-storage`, `notation-ui`, `notation-updater`).
- CI workflow: rustfmt + clippy + tests + release builds (Linux/Windows).
- Tag-based release workflow with checksummed artifacts.
- Iced prototype: multi-page whiteboard canvas, pen/highlighter/eraser,
  shape tools (rectangle, ellipse, line, arrow), text tool, page panel,
  undo/redo, and `.enbx` persistence.
