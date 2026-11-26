# Toolbar Map & Icon Inventory — "Prestigio Note" (EasiNote 5)

> Phase-2 evidence. Registration data recovered from the managed tool-bar
> service module; icon appearance measured from the resource PNGs
> (alpha/luminance analysis). Glyph descriptions below are functional notes
> used to draw original SVG icons in the port — no vendor pixels ship.

## 1. Dock groups (order = SortHint)

| Key | Group | Sort | Flyout | Action |
|---|---|---|---|---|
| Menu | Menu | 20 | StartMenuPopup | open start menu |
| Desktop | Menu | 25 | — | switch to desktop (trace) mode |
| Tool | Menu | 30 | — | open MagicBox (utilities) panel |
| Resource | Menu | 35 | — | open resource manager |
| OpenedWindow | Menu | 40 | OpenedWindowControl | windowed courseware list |
| Cyclopedia | Menu | 40 | — | open Cyclopedia tool |
| Connect | Menu | 200 | — | launch external EasiConnect (if present) |
| Camera | Menu | 201 | — | launch external camera app (if present) |
| Selector | Main | 110 | — | select/move mode |
| Pen | Main | 115 | InkContent | ink tools |
| Eraser | Main | 120 | EraserContent | eraser tools |
| Figure | Main | 130 | FigureContent | shapes |
| SlideRoam | Main | 145 | RoamContent | pan/zoom the slide |
| MiniBarText | Main | 150 | TextContent | text tool |
| InkRecognization | Main | 151 | — | handwriting recognition (hidden in EN) |
| Undo | UndoRedo | 195 | — | undo (Ctrl+Z registered) |
| Redo | UndoRedo | 196 | — | redo (Ctrl+Y registered) |
| NewPage | Page | 215 | — | insert blank slide after current |
| PreviousPage | Page | 220 | — | go to previous slide |
| SlideManager | Page | 225 | — | open page manager |
| NextPage | Page | 230 | — | go to next slide |

## 2. Mode items per tool

**Pen** (order, category `Pen`): HardPen 10 · MarkPen · BrushPen · BambooPen ·
LaserPen · MagicPen · TexturePen.
**Eraser**: PointEraser (stroke) · SelectorEraser (area/objects) + Clear-page button.
**Figure** (category groups): SmartShape · Circle · Ellipse · Parallelogram ·
RightTriangle · Trapezoidal · IsoscelesTriangle · Rectangle · DottedLine ·
WavyLines · Arrow · StraightLine · Pseudo3DCube · Pseudo3DTriangularPyramid ·
Pseudo3DRectangularPyramid · Pseudo3DCylinder · Pseudo3DCone ·
Pseudo3DTruncatedCone · Pseudo3DHemisphere · Pseudo3DSphere · Pseudo3DDihedron.
**Fill**: Fill · (plus extended: Star, Pentagon, Hexagon, ArcLines, Segment,
ParallelLines registered in the shape palette).
**Text**: Text · MiniBarText variants (writing templates).
**Roam**: SlideRoam (pan) with zoom slider + fit.

## 3. Icon inventory (resource names → 52×52 PNG)

menu_normal, desktop_normal, tool_normal, opendatabase, office, cyclopedia,
addnewpage, prepage, nextpage, pagemanagericon, pen_general, markpen_general,
brush_general, bamboobrush, laserpen, magicpen, texturepen, point, eraser,
freeeraser, roam, undo, redo, handinput, filler, textinput, arrow, rectangle,
circle, ellipse, parallelogram, trapezoidal, righttriangle, isoscelestriangle,
smartshape, star, pentagon, hexagon, straightline, dottedline, parallellines,
wavylines, arc, segment, pseudo3d* (cube, cone, cylinder, sphere, hemisphere,
dihedron, triangular/rectangular pyramid, truncated cone), copy, cut, paste,
dictionary, phonetic, chemicalformulatool, subjecttemplate, atomicstruct,
office, clean, cleancontainer, buttontoleft, buttontoright, middlebar (9-slice
skin), containerbg (flyout skin), penthumb…texturepenthumb (pen flyout
thumbnails = live stroke samples).

## 4. Glyph notes (functional descriptions for original redraws)

All dock icons share the silver-chip style (UI-SPEC §12). Glyph notes:

- **pen_general** — diagonal pen body, wide nib pointing bottom-left, body
  band across the middle.
- **markpen_general** — flat chisel marker drawn horizontally, square tip.
- **brush_general** — angled handle with bristle tuft (brown accent) at
  lower left.
- **bamboobrush** — brush with two parallel band joints on the handle.
- **laserpen** — pen with a bright horizontal aperture band near the tip.
- **magicpen** — wand with 4-point sparkle at upper left + dot trail.
- **texturepen** — pen with a hatch/cross-hatch texture patch at the top.
- **point** — pointing-hand cursor with extended index finger (dark fill).
- **eraser** — angled parallelogram block, two-tone split.
- **freeeraser** — eraser block with a checkered/motion pattern + small squares.
- **clean** — upright broom (52u tall, in the clear-page chip).
- **handinput** — hand with pen writing motion.
- **filler** — paint bucket with droplet (dark lid line on top).
- **textinput** — "A|" type glyph (cursor bar).
- **arrow** — block arrow pointing right-up.
- **roam** — four-direction move arrows in a diamond arrangement.
- **undo / redo** — curved arrows (undo curls left, redo curls right).
- **addnewpage** — page sheet with bold plus at its center.
- **prepage / nextpage** — chevron arrows pointing left / right.
- **pagemanagericon** — film-frame: bracket rails + row of cells.
- **menu_normal** — filled rounded tile with 3 horizontal lines (list).
- **desktop_normal** — monitor silhouette on a stand.
- **tool_normal** — wrench crossing a gear.
- **opendatabase** — open folder / database stack.
- **office** — window layout with title bar dots.
- **cyclopedia** — closed book with "B" letterform.
- **smartshape** — irregular pentagon blob (intelligent shape).

Pen flyout thumbnails show **live stroke samples** for each pen type (a
diagonal stroke rendered by that pen's renderer) — the port regenerates them
with its own ink engine.

## 5. Desktop-mode strip (TraceToolBar 316×53)

Modes (from the desktop mode item list): Arrow (select) · Eraser · Square
(shape) · Roam · TextInput · Point — plus mini-tool tiles (calendar, clock,
inner board, physical show…) and undo/redo, driven by the same mode service.
