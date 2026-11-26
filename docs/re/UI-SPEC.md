# UI Layout Specification — "Prestigio Note" (EasiNote 5 rebrand)

> Phase-2 deep reverse-engineering evidence. Every value below was measured from
> the shipped installer (BAML recovered to XAML with REA + ilspycmd, managed C#
> inspected, image resources analyzed pixel-wise). Facts only; the re-implementation
> is original Svelte/CSS that consumes this spec.

## 1. Measurement system

All layout goes through markup extensions (`RelativePixel`, `RelativeFontSize`,
`RelativeCornerRadius`, `RelativeThickness`) that declare **1080p-base design
units** and rescale to the actual display. The port maps this to a single CSS
custom property scale:

```css
:root { --rp: calc(100vh / 1080); }   /* 1 design unit */
/* usage: height: calc(var(--rp) * 80); */
```

Default window: **1280×720**, shell min-width **1440u**. Toolbar font size 12u;
labels truncate with ellipsis inside the button width.

## 2. Shell frame (`ShellFrameView`)

- Root grid: row 0 = board content, row 1 (auto) = toolbar.
- `SystemTimeControl` top-right, margin 30/24, opacity 0.8 (clock + date).
- Board slide template: drop-shadow nine-slice (edge 11/9/18/20) around the
  slide; slide transitions animate 0.8 s with quadratic ease-in-out.
- RecoveryPanel overlays the content grid (crash-recovery snapshot list).
- **Toolbar hide/show**: the dock sits 24u *below* the viewport bottom; it
  slides up/down by 24u on demand (0.2 s linear).
- **Subject scene change**: toolbar fades out/in over 0.6 s while dropping
  118u and returning.

## 3. The dock toolbar (`ShellToolBar`)

Height **80u**, three clusters on one row. Each cluster is skinned by a
nine-slice `MiddleBar.png` (edge 30/30/30/0, transparent, radius 16u on the
top corners) that bleeds 15–30u outside its bounds.

| Cluster | Side | Content (registration order) |
|---|---|---|
| Menu | left | Menu(20), Desktop(25), Tool(30), Resource(35), OpenedWindow(40), Cyclopedia(40), Connect(200), Camera(201) |
| Main | center | Selector(110), Pen(115), Eraser(120), Figure(130), SlideRoam(145), Text(150), InkRecognization(151*) |
| UndoRedo | center | Undo(195), Redo(196) — shown beside the Main group |
| Page | right | NewPage(215), PreviousPage(220), SlideManager(225), NextPage(230) |

\* InkRecognization is removed in English display mode and windowed mode.

- Each outer edge has a collapse chevron button (`ButtonToLeft/Right`), 30u
  wide, that hides the cluster.
- Item = icon cell (56u) over label cell (24u, centered, 12u font, ellipsis).
- Main items are **radio buttons**: selecting animates translateY ±12u (0.2 s);
  active state = rounded pill (radius 20u) behind the icon.
- Disabled items: opacity 0.33. Hidden items collapse entirely.
- Tooltips come from the locale dictionary (`Lang.ToolBar.*`).

### Flyout popup (`ArrowPopup`)

Opens above the dock button. Nine-slice window frame: 32u top band, 22u side
bands, auto bottom; top-center pointer 50×34u. Content margin −20/−40 inset.
`StaysOpen`; closes on command.

## 4. Tool flyouts (contents of each Main button)

### Pen (`InkContent`) — two columns
- Left: thickness setter (**1–48u**, min-width 280u, live stroke preview in
  current ink color) over the pen-mode strip.
- Right: color selector (no opacity column) + conditional texture/magic-pen
  image pickers.
- Pen modes (60u items, horizontal strip): HardPen, MarkPen, BrushPen,
  BambooPen, LaserPen, MagicPen, TexturePen.

### Eraser (`EraserContent`)
- Mode strip: PointEraser (stroke eraser), SelectorEraser (area/object eraser).
- **Clear page** button: 170×52u skin with a 52u broom icon, label
  `SlideClearEraserMode` right-aligned beside it.

### Figure (`FigureContent`)
- Left column 292u: stroke thickness setter (**1–48u**) over the shape grid.
- Right: foreground color + fill color (fill grid appears when a "filled" shape
  category is selected).
- Shape modes (52×52 icons, wrap panel — see TOOLBAR.md §3).

### SlideRoam (`RoamContent`)
- Live slide minimap **320×180u**, 1u black border, pointer rect #1E90FF
  (fill 0F1E90FF) + viewport rect #FF6347 dashed 5,5.
- Zoom slider row + percentage text (20u font) + FitScale button (49u).

### Text (`TextContent`)
- Mode strip only (text templates: plain, lined paper, 田字格, pinyin grid …).

## 5. Mode selection ring (`GenericCategoryModeSelector`)

- Container: nine-slice skin (edge 20), wrap panel, items 52×52u (margin 1u).
- Selection ring: 48×48u border **#1E90FF**, thickness 4u, radius 12u,
  offset +10/+10, moves between items with 0.2 s deceleration.
- License-locked modes show a small badge at the item's bottom-right.

## 6. Color selector (`ExtendedColorSelector`)

- Panel background: vertical gradient **#030303 → #464646**, radius 10u,
  padding 5u.
- Standard colors: grid 5 rows × 3 cols of **40×40u** swatches (radius 5u);
  selected ring **#9D1B01** (2u).
- Eyedropper button 40×40u overlapping the grid's bottom-right corner.
- Optional opacity column (disabled in dock flyouts): "no color" toggle
  40×40u (white tile, red diagonal strike, stroke #9E0B0F 5u), 18×18 icon,
  vertical slider 116u tall, range 1–10, default 10.

Palettes (from `ColorsProvider`):

| List | Colors |
|---|---|
| Quick 12 (dock flyouts) | `#FF1100 #FF8B00 #FFC511 #66D553 #331EB5 #336ED9 #4FA0B7 #FF1FCE #306D00 #000000 #8B7F6E #FFFFFE` |
| Extended 42 | see `theme.json` `palette.extended` |
| Six standard | `#FFFFFF #231F20 #F2343E #FFDE17 #29CC7B #00AEEF` |

Recently used custom colors are appended (up to the 15-cell grid), then
evicted oldest-first.

## 7. Start menu (`StartMenuPopupControl`)

- Left rail of radio buttons, min **175×55u**, radius 8u:
  File, Import, Export, Print, Setting, Help, DisplayMode, Quit.
  - Idle: white vertical gradient (#FFFFFF → #EAEAEA), black bold 17u label,
    40u icon (margin 10/6), drop shadow 2u #BFBFBF.
  - Active: black gradient (#444444 → #000000), white text, right chevron.
  - Items without submenus hide the chevron (Print, Setting, Quit).
- Submenu pane 120u wide, toggled by rail selection (FileMenu, ImportMenu,
  ExportMenu, HelpMenu, DisplayModeMenu).
- Top and bottom separators: 2u radial-gradient bars (#CCCCCC → #FFFFFF).
- Bottom row: ExternalToolsControl — 64×64u tiles with 62×62u #D8EBF7 fill
  (radius 5u) shown on press.

## 8. Element editor (selection toolbar)

- Borderless always-on-top window **372×443u**, transparent margin 30u,
  shadow blur 30 #B0B0B0.
- Tabs: **Material / Font / Animation**, headers 103×52u (selected header image
  expands full cell); close button 38×46u top-right.
- FontPanel: font-family combo + font-size combo (skinned), bold/italic/
  underline toggles, text color grid; skinned scrollbar with image thumb.
- AnimationPanel: effect category tree (top-level/submenu items), speed combo,
  trigger combo.
- MaterialPanel: material/texture swatch grid with custom scrollviewer.

## 9. Windows (page manager, resources, settings, send…)

| Window | Size / chrome | Key colors |
|---|---|---|
| SlideManager | panel 350u wide, paged grid + groups, add/copy/effect/delete ops 42×32u | #242424 dark, #6495ED accent, #FF7F50 highlight |
| Transition settings | 148×110u list inside SlideManager | #A9A9A9 |
| ResourceManagement | 520×80u header + tabs; explorer bar; web grid | #65A0EC accent, #242424 |
| Settings | 540u wide, language list | #F4F4F5 bg, #9E0B0F accent |
| SendMail | 941×600u form | #FAFAFA |
| Recovery | 390u panel, snapshot rows | #F4A460 accent |
| OpenedWindows | 200×90u cards | #1E90FF / #87CEFA |
| Common window | nine-slice skin, 32×32u close, 15u title | — |

## 10. Subject scenes

Left banner 38u; selector cards 122×70u. Five scenes: Standard, Chemistry,
English, Math, Physics (full-screen backgrounds per subject). Writing
templates 90×110u cards: 田字格 (mizige), pinyin mizige, four-line English,
横线/四线 Chinese, math grid, default Ch/En.

## 11. Desktop mode

- Floating **TraceToolBar** 316×53u (44u buttons): pen, eraser, undo/redo…
  plus mini-tools (calendar, clock, inner board, physical show…).
- Pen settings chip 233×44u: 6 quick colors `#00BFF3 #0BE73F #FF0000
  #800080 #FFF200 #FFFFFF` + thickness presets (44×38u buttons).
- Eraser chip 151×53u with a clear-board button (170×52u skin class).

## 12. Icon style (for original redraws)

All dock icons are 52×52u PNGs: a **silver rounded-square chip** (~44×44u,
radius ~8u, vertical gradient #E0E0E0 → #C0C0C0, thin darker border) with a
**dark glyph** (≤ 20% chip area) and a small color accent (brush bristles
brown #806040, magic-pen sparkle gold #E0C080, laser aperture band…).
Glyph-by-glyph descriptions are in TOOLBAR.md §4.
