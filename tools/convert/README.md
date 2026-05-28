# Contour-Walker Geometry Compiler

Raster-to-vector geometry compiler for retro / sector-based game engines.

Converts ASCII occupancy maps into simplified polygonal sector geometry.

---

# Overview

This project converts:

```text
ASCII occupancy grids
```

into:

```text
sector-based vector geometry
```

using:

```text
flood fill
→ boundary extraction
→ ordered contour tracing
→ polygon simplification
→ wall generation
```

The goal is to create a practical geometry compiler for:

* retro FPS engines,
* software-rendered engines,
* sector/portal engines,
* BSP experimentation,
* and future procedural geometry systems.

The compiler intentionally moves away from:

* naïve tile walls,
* per-tile geometry,
* and grid-locked maps,

toward:

* inferred polygonal geometry,
* large simplified walls,
* and vector-like map representations.

---

# Pipeline Architecture

Current pipeline:

```text
ASCII grid
    →
sector flood fill
    →
boundary edge extraction
    →
ordered contour tracing
    →
polygon simplification
    →
collinear edge merge
    →
map01.txt
```

---

# Why This Architecture?

## The Naïve Approach

The original implementation attempted:

```text
tile
    →
emit walls directly
```

This works for:

* rectangles,
* orthogonal rooms,
* simple test maps.

But it quickly breaks down because:

* map files become enormous,
* every wall becomes fragmented,
* diagonal structures become staircases,
* rendering costs explode,
* and polygonal geometry becomes impossible.

---

## Intermediate Approach: Tile Edge Merging

The next approach attempted:

```text
extract tile edges
    →
merge collinear neighbors
```

This improved:

* file size,
* large flat walls,
* and orthogonal maps.

But it remained fundamentally brittle:

* diagonals still staircase,
* contours fragment,
* simplification becomes unstable,
* topology becomes ambiguous.

---

## Final Chosen Architecture: Ordered Contour Tracing

The current system instead treats the ASCII grid as:

```text
an occupancy field
```

rather than:

```text
a collection of tiles
```

This is a major conceptual shift.

Instead of:

```text
"emit walls from tiles"
```

the engine now does:

```text
"extract polygons from occupancy"
```

This unlocks:

* diagonal inference,
* triangular rooms,
* simplified polygonal geometry,
* future BSP generation,
* SVG/vector import possibilities,
* procedural geometry systems,
* and efficient rendering.

---

# Core Systems

## 1. Flood Fill Sector Detection

Sector tiles are grouped using BFS flood fill.

This allows:

* disconnected sector types,
* automatic sector discovery,
* and sector-based geometry extraction.

---

## 2. Boundary Edge Extraction

The system extracts only:

```text
sector boundary edges
```

rather than all tile edges.

Interior edges are discarded.

This dramatically reduces geometry complexity.

---

## 3. Ordered Contour Walker

This is the core of the system.

The contour walker:

* reconstructs ordered polygon boundaries,
* traverses connected edges,
* preserves winding order,
* and produces contour chains suitable for simplification.

This replaced earlier:

* unordered edge soup,
* and local edge merging systems.

---

## 4. Douglas–Peucker Simplification

Contours are simplified using:

```text
Douglas–Peucker polygon simplification
```

This allows:

* staircase reduction,
* diagonal inference,
* and large wall reduction.

Example:

```text
########
#......#
#.....##
#....###
```

becomes:

```text
single diagonal wall
```

instead of dozens of tiny stair segments.

---

## 5. Global Collinear Merge

After simplification:

* adjacent collinear edges are merged globally.

This minimizes:

* map size,
* render cost,
* and wall count.

---

# CLI Usage

## Convert ASCII Map

```bash
cargo run -- ascii2map source.txt map01.txt
```

Example:

```bash
cargo run -- ascii2map ../../maps/ascii/idea1.txt ../../maps/episode01/map01.txt
```

---

# ASCII Map Format

Example:

```text
KEY

# = wall textureN
1 = wall brickwall

. = sector metalfloor metalroof
, = sector textureD textureU

MAP

####################
#..................#
#........P.........#
#..................#
####################
```

P is a reserved letter that indicates the player's spawning point.
"sector" keywords must be followed by two textures. First the floor texture then the roof.
"wall" keywords must be followed by a single texture.

---

# Supported Geometry

## Fully Supported

### Orthogonal Rooms

```text
##########
#........#
#........#
##########
```

---

### Irregular Manhattan Shapes

```text
##########
#........#
#....#####
#....#
######
```

---

### Simplified Diagonal Inference

```text
####################
#...............111#
#.............1111#
#...........11111#
##################
```

These staircase forms can simplify into:

```text
large inferred diagonal walls
```

---

### Multi-Sector Maps

```text
#######1111
#.....#1111
#.....#1111
#######1111
```

Different sector materials and wall textures are supported.

---

# Unsupported / Experimental Geometry

## Acute Diagonal Exterior Corners

Example:

```text
      ##
     ####
    ##..##
```

These introduce:

* contour ambiguity,
* non-manifold vertices,
* branching traversal cases.

The current contour walker is:

```text
tile-edge topology based
```

rather than:

```text
marching-squares topology based
```

These cases are currently experimental.

---

# Manhattan-Closed Geometry

The current system works best with:

```text
Manhattan-closed occupancy
```

Meaning:

* regions are connected through cardinal directions,
* boundaries do not touch only diagonally,
* and geometry forms valid tile-connected regions.

Good:

```text
##
##
```

Bad:

```text
#.
.#
```

Diagonal-only connectivity creates ambiguous topology.

---

# Why Marching Squares Was Considered

At several stages we considered replacing the architecture with:

```text
occupancy field
    →
marching squares
    →
polygon extraction
```

This would solve:

* diagonal ambiguity,
* corner ambiguity,
* acute contour cases,
* and non-manifold topology.

However:

* marching squares is significantly more complex,
* harder to debug,
* and less transparent during engine bring-up.

The current contour-walker approach was chosen because:

* it is understandable,
* incremental,
* debuggable,
* and already extremely capable for beta-level geometry compilation.

---

# Current Limitations

## 1. Acute Corner Ambiguity

Some diagonal corner arrangements can:

* reverse contour winding,
* fragment contours,
* or generate invalid polygons.

---

## 2. No Hole Detection Yet

Interior voids are not yet treated as nested polygons.

---

## 3. No Convex Decomposition

Large concave polygons are emitted directly.

Future BSP systems may require:

* convex decomposition,
* monotone partitioning,
* or triangulation.

---

## 4. Simplification Can Overreach

Aggressive simplification values may:

* cut corners incorrectly,
* self-intersect geometry,
* or merge semantic walls unintentionally.

---

# Future Work

## Marching Squares Extraction

Potential future replacement for tile-edge extraction.

Would enable:

* true continuous contour extraction,
* robust diagonal topology,
* and cleaner polygon generation.

---

## Portal Generation

Automatic:

```text
sector-to-sector portal inference
```

for visibility systems and portal rendering.

---

## BSP Compilation

Future support for:

* BSP generation,
* node splitting,
* and software-rendered visibility trees.

---

## SVG / Vector Import

Future possibility:

```text
SVG
    →
polygon sectors
    →
compiled game maps
```

---

## Polygon Triangulation

Future support for:

* floor rasterization,
* GPU rendering,
* and convex decomposition.

---

# Philosophy

This project intentionally explores:

```text
geometry as compilation
```

rather than:

```text
geometry as manual level editing
```

The long-term goal is:

* adaptive geometry systems,
* procedural sector generation,
* computational map design,
* and geometry pipelines that operate more like compilers than editors.

---

# Status

Current status:

```text
Robust beta
```

Successfully supports:

* large maps,
* irregular polygons,
* inferred diagonals,
* sector extraction,
* polygon simplification,
* and compact geometry generation.

Remaining challenges are primarily:

* advanced topology,
* diagonal ambiguity,
* and non-manifold contour handling.

```
```
