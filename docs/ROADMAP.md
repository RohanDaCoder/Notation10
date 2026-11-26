# Notation10 — Clean-Room Port Roadmap

**Goal:** a 1:1 clean-room re-implementation of the *Prestigio Notes* ("Note") Windows
application (v3.1.4.3735) in **Rust**, shipped with CI, tests, tag-based GitHub
releases and an in-app updater. GUI framework: **Tauri 2** (migrated from the
Iced prototype in v0.2.0).

**Clean-room policy:** we only observe the original product (installer payload,
resources, UI strings, public behavior) and write original Rust code from those
observations. No proprietary code, assets or binaries are copied into this repo.
Original artwork (icons/logos) is re-drawn from scratch; only functional UI labels
and layout metrics are mirrored.

---

## Phase 0 — Bootstrap *(done)*

- [x] Toolchain in place: Rust stable, `gh` CLI, 7-Zip, REA (rea.tools) connected to the coding agent.
- [x] Repository cloned; git identity configured (`RohanDaCoder <102975537+RohanDaCoder@users.noreply.github.com>`).

## Phase 1 — Clean-room reverse engineering *(done)*

- [x] Download `NoteSetup_3.1.4.3735.exe` (NSIS 2.46.5-Unicode, ~294 MB) from the vendor CDN.
- [x] Extract the NSIS payload (7-Zip) into a quarantined work directory (never committed).
- [x] Identify the app technology stack: .NET 4.5.2 WPF rebrand of Seewo EasiNote 5.
- [x] Inventory: executables, libraries, resources, licenses, version info, file associations.
- [x] Document the UI structure: windows, panes, toolbars, menus, dialogs, shortcuts, strings.
- [x] Document the data layer: `.enb` container, `.enbr` recovery, settings registry keys.
- [x] Produce `docs/re/REPORT.md` (the porting contract).
- [x] REA-assisted analysis where applicable (binary inspection, bundle analysis).

## Phase 2 — Scaffold *(done)*

- [x] Scalable, modular multi-crate Cargo workspace.
- [x] CI workflow: fmt + clippy + test + debug build on push/PR (Linux + Windows matrix).
- [x] Release workflow (tag-based): builds the installer, attaches checksums
      and `latest.json`, publishes the GitHub Release.
- [x] README, architecture doc, contribution/PR conventions (squash-merge).

## Phase 3 — Prototype *(done — Iced v0.1.0, Tauri since v0.2.0)*

- [x] Main window shell reproducing the original layout per the RE report.
- [x] Domain model in `notation-core` with unit tests.
- [x] Window sizing, fonts, iconography approximating the original look.
- [x] Runs headless-compiled locally; binary builds in CI.

## Phase 4 — Features via pull requests (review + squash merge)

| PR | Feature | Status |
|----|---------|--------|
| #1 | Autosave recovery snapshots | merged (v0.1.0) |
| #2 | GUI migration Iced → Tauri 2 | merged (v0.2.0) |
| #3 | Crash-recovery prompt ("not closed normally") | merged (v0.2.0) |
| #4 | Export page as PNG (start-menu "Export image") | merged (v0.2.0) |
| #5+ | Unsaved-changes guard, drag-and-drop `.enbx`, mini tools ("Treasure box") | planned |

## Phase 5 — Release system *(done)*

- [x] Tag `v0.1.0` (Iced era) and `v0.2.0` (Tauri era): CI produces a Windows
      NSIS installer + checksums + update manifest.
- [x] GitHub Release with artifacts, `SHA256SUMS.txt`, `latest.json`.
- [x] In-app updater validated against the published release protocol.

## Phase 6 — Tauri migration *(done, v0.2.0)*

- [x] `notation-ipc`: the whole session state machine as pure, unit-tested Rust
      (checkpoints, gesture commits, status strings mirrored from the Iced loop).
- [x] `notation-ui`: theme as serializable design tokens (dropped the iced dep).
- [x] `notation-updater`: `latest.json` manifest protocol + platform assets.
- [x] `notation-tauri`: Tauri 2 shell, command layer, native dialogs, updater
      side effects, NSIS bundling.
- [x] `ui/`: dependency-free webview front end (canvas renderer, gesture
      machine, dock, page panel, start menu, settings, about, update bar,
      notes strip) + browser mock backend for harness testing.
- [x] Documented deltas: figure/select-erase now undoable; page navigation no
      longer pollutes undo history; highlighter renders as translucent stroke.

## Phase 7 — Fidelity hardening

- [ ] Walk the RE report checklist pane-by-pane; file gap issues; fix in follow-up PRs.
- [ ] Performance pass (startup, canvas latency, snapshot size).
- [ ] Final 1:1 review and `v1.0.0` if parity is reached.

---

## Working agreements

- All GitHub operations via the **gh CLI**.
- Feature work lands **only via PRs**; each PR is reviewed (inline comment
  review) and **squash-merged**.
- Commits signed-off with the project committer e-mail above.
- English only in code, issues, PRs and docs.
