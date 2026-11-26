# Mini-Tools Specification — "Prestigio Note" (EasiNote 5)

> Phase-2 evidence from the `EasiNote.MiniTools` assembly (BAML→XAML, C#
> inspection). Sizes in 1080p design units (UI-SPEC §1). All behavior notes
> are functional observations; the port implements them from scratch.

Mini tools are spawned from the desktop-mode TraceToolBar strip and the Tool
(MagicBox) panel. Common window chrome: borderless, always-on-top, no taskbar
entry, drag handle + close button (46×46u zones), optional maximize.

## Calculator (`CalculatorWindow`)

- Window **360×530u**, no resize; calculator control fills it.
- Display strip at top (dark), result font 28u; expression + "M" memory
  indicator; keys are custom buttons in a grid; body background **#363636**,
  accent key **#65A0EC**, text **#FFFFFF**, secondary text #808080.
- One result + one operations row layout; number pad 4 columns.
- Standard calculator behavior (%, CE/C, ±, memory M+/M-/MR/MC, sqrt …).

## Timer (`TimerWindow` / `TimerControl`)

- Window **456×350u**; body 456×280u.
- Digits: up/down stepper cells (**40×40u** arrows), digit font **52u**,
  digit color **#FFF799** (pale yellow) on the dark timer container skin.
- Controls (image buttons): set (edit), start, pause, stop + a small
  hyperlink row.
- Countdown → 0 plays a sound and flashes; timer runs while the tool floats.

## Clock (`Clock`)

- Window **256×256u** (face 240u): analog clock with separate images for the
  dial, hour hand, minute hand, second hand; drag handle & close (46u zones);
  no digital text. Hands rotate by real time.

## Calendar (`Calendar`)

- Window ~**320×460u**, WPF calendar re-templated: month header with prev/next
  arrows, day-of-week row, 6×7 day grid; today marker **#FAC61B** (amber),
  weekend/other-month text **#808080**, selected day tile accent **#6AB9D4**
  (cyan), first-day tile **#95C455** (green); toolbar: back-to-table,
  full-screen, close.
- Header font 20u; day cells 40u class.

## Shaft (`Shaft`) — ruler / protractor workbench

- Full-screen stage (min 1920×440, full 1080) with draggable axis:
  horizontal/vertical rulers, tick strips (Point_Big/Point_Small), stick pins
  (Stick/Stick_Focus), node titles (30u font), add-resource, export, close.
- Semi-transparent dark chrome (#525252 at 33–66% alpha), white lines, move
  storyboard animations; nodes can link to media (NodeMediaBar: play/pause,
  sound slider 8u thick) and details (NodeDetailControl 1344×756, edit-link,
  max/default size, close).

## SpotLight

- Config window ~**200×160u** + a zoomable spotlight overlay: square lamp
  toggle (SquareLamp), zoom in/out flags, close/config 28u buttons, slider
  (track thumb 28u); chrome #2E2E2E dark with #7C7979 borders, warm accent
  #A8834E. Launch storyboard fades the overlay in.

## Magnifier

- Round magnifier: ring image **777×777u** window (min 300), zoom in/out flags
  (200u bar, 28u buttons), touch lock, config, close; border #2E2E2E, text
  #F5F5F5, accent #9ACD32.

## InnerBoard (side board)

- Side blackboard overlay **1690×870u** (min 300×300) sliding in from the
  right edge (up/down/collapse storyboards, exponential ease); board skin +
  right rail with 3 tools: pen, erase board, save board (47–48u icons);
  transparent hit layer.

## Cyclopedia

- Browser-like panel **1120×666u**: menu bar (left/middle/right images),
  search box (245u), result list (666u scroll area), paging (prev/next 34u,
  page number), content 14–15u text; chrome grays #F2F2F2→#585858, link blue
  **#1E90FF**, accent **#73A7ED**. In the port this is a local/offline
  knowledge stub (no vendor endpoints).

## Others

- **FlashWindow**: 46×46u floating chrome (move/screenshot/max/min/close)
  for embedded Flash content. The port ships no Flash — the slot stays
  reserved.
- **WrapScreen**: full-width tip banner (min 360u) with dark text on light.
- **NumberControl**: reusable digit stepper (shared with Timer).
- **NodeDetail / NodeMediaBar**: courseware node detail (1344×756) + media
  control bar (play/pause, sound, 140–160u wide).

## Sound effects

`Sounds/` ships WAV/MP3 effects for: applause, camera, chimes, cuckoo, jump,
page turn, timer alert, tool select — used on page transitions and timer
events. The port substitutes original synthesized/CC0 sounds.
