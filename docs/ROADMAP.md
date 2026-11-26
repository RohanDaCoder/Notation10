# Notation10 — Clean-Room Port Roadmap

**Goal:** a 1:1 clean-room re-implementation of the *Prestigio Notes* ("Note") Windows
application (v3.1.4.3735) in **Rust** with the **Iced** GUI framework, shipped with
CI, tests, tag-based GitHub releases and an in-app updater.

**Clean-room policy:** we only observe the original product (installer payload,
resources, UI strings, public behavior) and write original Rust code from those
observations. No proprietary code, assets or binaries are copied into this repo.
Original artwork (icons/logos) is re-drawn from scratch; only functional UI labels
and layout metrics are mirrored.

---

## Phase 0 — Bootstrap *(done)*

- [x] Toolchain in place: Rust stable, `gh` CLI, 7-Zip, REA (rea.tools) connected to the coding agent.
- [x] Repository cloned; git identity configured (`RohanDaCoder <102975537+RohanDaCoder@users.noreply.github.com>`).

## Phase 1 — Clean-room reverse engineering

- [ ] Download `NoteSetup_3.1.4.3735.exe` (NSIS 2.46.5-Unicode, ~294 MB) from the vendor CDN.
- [ ] Extract the NSIS payload (7-Zip) into a quarantined work directory (never committed).
- [ ] Identify the app technology stack (Electron / Qt / .NET / native), frameworks and runtime.
- [ ] Inventory: executables, libraries, resources, licenses, version info, file associations.
- [ ] Document the UI structure: windows, panes, toolbars, menus, dialogs, shortcuts, strings.
- [ ] Document the data layer: storage location, notebook/note formats, sync/settings files.
- [ ] Produce `docs/re/REPORT.md` (the porting contract) + `docs/re/ui-inventory.md`.
- [ ] REA-assisted analysis where applicable (binary inspection, bundle analysis).

## Phase 2 — Scaffold

- [ ] Scalable, modular multi-crate Cargo workspace:
      `notation` (GUI binary), `notation-core` (model + storage), `notation-ui` (theme/widgets),
      `notation-updater` (self-update client).
- [ ] CI workflow: fmt + clippy + test + debug build on push/PR (Linux + Windows matrix).
- [ ] Release workflow (tag-based): on `v*` tags, build optimized binaries, attach
      checksums, publish a GitHub Release.
- [ ] README, architecture doc, contribution/PR conventions (squash-merge).

## Phase 3 — Prototype

- [ ] Main window shell reproducing the original layout per the RE report
      (three-pane: notebook sidebar / note list / editor).
- [ ] Domain model in `notation-core` (Notebook, Note, metadata) with unit tests.
- [ ] Window sizing, fonts, iconography approximating the original look.
- [ ] Runs headless-compiled locally; binary builds in CI.

## Phase 4 — Features via pull requests (review + squash merge)

| PR | Feature | Contents |
|----|---------|----------|
| #1 | Persistence | note/notebook CRUD + on-disk store in the original's data shape, autosave |
| #2 | Editor | rich text toolbar, formatting, note title/date rendering per original |
| #3 | Search & organize | full-text search, notebook management, pin/favorite, sort |
| #4 | Theming | light/dark palette mirroring the original, accent colors |
| #5 | In-app updater | check GitHub Releases API, compare semver, download + verify + install |
| #6 | Settings & import/export | settings dialog, note export formats |
| #7 | Shortcuts & polish | keyboard shortcuts, about dialog, tray/exit behavior |

## Phase 5 — Release system

- [ ] Tag `v0.1.0`; CI produces Windows installer/portable exe + Linux binary.
- [ ] GitHub Release with artifacts and `SHA256SUMS.txt`.
- [ ] In-app updater validated against the published release.

## Phase 6 — Fidelity hardening

- [ ] Walk the RE report checklist pane-by-pane; file gap issues; fix in follow-up PRs.
- [ ] Performance pass (startup, list scrolling, editor latency).
- [ ] Final 1:1 review and `v1.0.0` if parity is reached.

---

## Working agreements

- All GitHub operations via the **gh CLI**.
- Feature work lands **only via PRs** after the initial prototype; each PR is
  reviewed (inline comment review) and **squash-merged**.
- Commits signed-off with the project committer e-mail above.
- English only in code, issues, PRs and docs.
