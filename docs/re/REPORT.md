# Clean-Room Reverse-Engineering Report — "Prestigio Note" 3.1.4.3735

> Porting contract for Notation10. All findings below are observations of the
> shipped installer only; no vendor code, binaries or assets are copied into
> this repository. The re-implementation is original Rust/Iced code.

## 1. Artifact identity

| Property | Value |
|---|---|
| Installer | `NoteSetup_3.1.4.3735.exe` (295 MiB, SHA-256 `c814c947ae902f41b9ae83623def1b8c84f4c96f574919603c1feff73071ea49`) |
| Installer tech | NSIS 2.46.5 (Unicode), 32-bit, `requireAdministrator` |
| Product | "Note" (rebranded **EasiNote 5** interactive-whiteboard courseware software) |
| Vendor | Guangzhou Shirui Electronics Co., Ltd (CVTE / Seewo), distributed by ASBIS/Prestigio |
| Build | buildkit 5.14.0.657, rev `95c33835`, 2018-11-01 |
| Main binary | `Main/Note.exe` — PE32 **.NET 4.5.2 WPF** assembly (requires .NET Framework 4.5, bundled `dotnetfx45_full`) |
| Updater | EasiUpdate (`swupdater.exe`, `LibEasiUpdate.dll`, `Update.config`: AppName `Note`, registry key `ShiRui\Note`) |
| Support installs | K-Lite codec pack, Flash Player 19, PPTService, SaveAsPDFandXPS, Ghostscript (`gswin32c.exe` for PDF export) |

## 2. Composition

- ~1110 files in `Main/` (460 MiB): `EasiNote.*.dll` (Shell, Business, Controls, Elements,
  Foundation, Theme, Resource, MiniTools, Office…), `Cvte.*.dll` (Paint.Core, Paint.Inks,
  Windows, IO, Algorithm…), WPF support libs (WPFMediaKit, WindowsAPICodePack, Prism-era
  `Microsoft.Windows.Shell`, log4net, Newtonsoft.Json).
- `CultureResource/<lang>/{Main,Auth,Custom}.xaml` for 30+ languages (ar, bs, cs, de, el,
  en, es, et, fa, fi, fr, hr, it, ja, kk, ko, lt, lv, nl, pl, pt, ro, ru, sl, sr, tr, uk,
  zh-CHS, zh-CHT).
- `Extensions/` — pluggable "Enplug" modules (Education, FunctionDraw, Communication,
  InternetExplorer, MiniTools, swenserver).
- `Sounds/` — page-transition sound effects (Applause, Camera, Chimes, Cuckoo, Jump, Toll `.wav`).
- `Dict/` — Chinese/English dictionary data for the dictionary mini-tool.
- `Converter/V0/` — legacy EasiNote (pre-5) tree used to convert old `.enb` files.

## 3. Application model (from strings/resources)

- **Document**: multi-page whiteboard courseware. Pages ("slides") can be grouped
  ("Default group"), reordered, cloned, deleted; per-page speaker notes
  ("Click here to add notes"); page thumbnails.
- **Modes**: Teaching mode, Preparation mode, Desktop mode; subject scenes
  (Standard, Math, Physics, Chemistry, English).
- **Toolbar tool families** (observed labels): Select, Pen/Hard-tipped pen,
  Chinese brush, Bamboo paintbrush, Highlighter, Laser pen, Texture pen, Stamp/Magic pen,
  Eraser (point/select/slide-clear), smart-shape recognition ("Intelligent identification"),
  shapes (line, segment, dotted, wavy, arrow, arc, parallel, circle, ellipse, rectangle,
  parallelogram, trapezoid, triangle/right/isosceles, pentagon, hexagon, star, 3D figures:
  cube, cuboid, cylinder, cone, frustum, sphere, hemisphere, dihedron, pyramids),
  Fill, Text input, Roaming, Add page, Previous/Next page, Undo/Redo, Copy/Shear/Paste,
  Treasure box (mini tools), Resource library, Browser, Dictionary, Subject template.
- **Mini tools** ("Treasure box"): Calculator, Calendar, Clock, Timer, Compass, Ruler,
  Protractor, 30°/45° set squares, Magnifier, Spotlight, Mask (screen cover), Screen shot,
  Camera/Object booth, Writing board, Thinking axis, Encyclopedia, Chinese pinyin,
  English phonetics, Browser, Favorites.
- **Education elements** (Chemistry/Physics): beakers, flasks, burets, condensers, test
  tubes, alcohol lamp, periodic table, atomic structure, ammeter/voltmeter, bulbs, rheostat,
  force composition, etc. `XmlType.config` maps ~100 element type names to
  `Cvte.EasiNote.Elements.*` classes (clean-room port targets a small representative subset
  first; the file documents the full original inventory).
- **Element editor**: property panel (Line: color/thickness/dash style; Fill), animations
  (Enter/Exit/Stress: fade, blinds, box, checkerboard, erase, explode, rotate, zoom, path,
  directional slides; speed slow/medium/fast; trigger before/after/click), layer arrange
  (promote/demote), group/ungroup, lock, mirror/flip, crop, drag-copy, hyperlinks
  (Web/Text/File/Page/Audio/Tool targets), text-to-speech ("Read"), handwriting recognition.
- **Page transitions**: Fade, Blinds, Circle diffusion, Rippling, Waves, Whirling, Blurred,
  directional slides + optional sound effects (from `Sounds/`).
- **File menu (StartMenu)**: New, Open, Save, Save as, Import (File/Clipboard/Text
  edit/Recognize manually), Export (Image/PDF/PPT/Word), Print, Settings, Help,
  User manual, About, Update, Activate product, Exit; mode switching; desktop mode.
- **Settings**: language (30+), default font/size, drawing/background/filling colors,
  speech speed, general materials, page proportion (4:3/16:9 with Adapt/Fill/Stretch),
  auto-update toggle, PPT assistant toggle, recovery.

## 4. Document format (`.enb`)

Observed in `EasiNote.Business.dll` strings and `EnbFileHelper`/`EnbV0Converter` naming:

- `.enb` = container holding `Document.xml` (serialized document: pages, elements,
  properties) + `Thumb.png` (cover thumbnail); `.enbr` is the related recovery/thumbnail
  artifact used by the crash-recovery dialog ("The following document is not closed
  normally at the previous time").
- Import filters: images (bmp/jpg/png/gif/ico/cur), video (wmv/avi/mp4/mkv/mov…),
  audio (wav/wma/mp3/mid), Flash (swf), PDF, TXT, PPT/PPTX, `.enb`.
- Export targets: image, PDF (via Ghostscript `gswin32c.exe` + SaveAsPDFandXPS),
  PPT/Word (via Office automation), with progress strings "Processing page {0}".

Notation10 implements a clean-room `.enbx` container with the same conceptual shape
(ZIP archive, `Document.xml` + `Thumb.png` + media entries) but its own XML schema.

## 5. Visual identity (observed)

- Welcome/splash art (`Welcome.gif`, `welcome_background.png`), UHD-aware resource DLLs
  (`EasiNote.Resource.UHD.dll`), theme DLL (`EasiNote.Theme.dll`), default light UI with
  colored toolbars; blue accent typical of EasiNote 5. Notation10 re-draws its own icons
  and palette (clean-room requirement) while preserving layout and labeling.

## 6. Porting strategy for Notation10

1. Multi-page canvas app with the same chrome: title/menu, side page panel,
   floating toolbars, mode switching. Rust + Iced `canvas` widget for the board.
2. Element model mirroring `XmlType.config` categories (strokes, figures, text, media,
   chemistry/physics subset), serialized to our own `Document.xml` inside `.enbx` zips.
3. Tools prioritized: select/move/resize, hard pen, highlighter, eraser, smart shapes,
   shape palette, text, fill, undo/redo, clipboard, page management, notes.
4. Mini tools as an extension trait system (`Extensions/` equivalent: calculators,
  clock, ruler, protractor…), starting with Calculator, Clock, Timer, Ruler, Protractor.
5. Updater: original used EasiUpdate; Notation10 ships `notation-updater` backed by
   GitHub Releases (tag-based) with SHA-256 verification.

## 7. Known deltas (documented, not silent)

- Original is Windows-only, touch-first, 32-bit .NET. The port is cross-platform Rust.
- Office/Flash/PPT automation cannot be 1:1; import/export ships PDF/image/ENBX first.
- Handwriting recognition, cloud resource bank and TTS depend on vendor services —
  stubs with the same UI affordances, no vendor endpoints are contacted.

## 8. Phase-2 deep RE evidence (2026-11-26)

Toolchain: REA 6.10.1 (`rea setup --client zcode`, `doctor` healthy, managed
static backend smoke-tested on the assemblies) + .NET 10 SDK with ilspycmd
11.2 for BAML→XAML recovery. 155 XAML views recovered across 19 assemblies;
toolbar registrations extracted from the managed tool-bar module; image
resources (811 PNGs inventoried) analyzed programmatically (alpha/luminance
stats, 9-slice edge measurements, dominant colors).

New evidence files (measured facts; port consumes them):

- `UI-SPEC.md` — shell, dock, flyouts, mode ring, color selector, start menu,
  element editor, windows, subject scenes, desktop mode, icon chip style.
- `theme.json` — machine-readable design tokens (sizes, colors, palettes).
- `TOOLBAR.md` — dock groups/sort order, per-tool mode lists, icon inventory
  and glyph notes for the original SVG redraws.
- `MINI-TOOLS.md` — Calculator, Timer, Clock, Calendar, Shaft, SpotLight,
  Magnifier, InnerBoard, Cyclopedia, FlashWindow and shared controls.

Key corrections vs. Phase-1 assumptions: the UI is dock-toolbar-driven (not
menu-rail-driven), all layout is 1080p-relative units, the default quick
palette is 12 colors (not 15), and InkRecognization is hidden in English
display mode.
