<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Spatialize</h1>
<p align="center"><em>Interactive hypergraph visualization for Molten</em></p>

<p align="center">
  <a href="../../Readme.md"><strong>Molten</strong></a> &nbsp;|&nbsp; <a href="../../../Readme.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../../../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../../../system/spatialize/Readme.md"><strong>Platform Spatialize</strong></a> &nbsp;|&nbsp; <a href="../../../License.md"><strong>License</strong></a>
</p>

---

This document describes **Molten Spatialize**, an interactive visualization system for hypergraph exploration.

---

## Invoke

Run the spatialize visualization:

```bash
bazel run //Molten/system/spatialize:command
```

This opens an interactive window displaying hypergraph state with real-time layout simulation.

---

## Panes

Toggle between visualization modes:

| Pane | Description | Key |
|------|-------------|-----|
| Relation | Edge and node relationships | Tab |
| Inference | Derivation and inference paths | Tab |

---

## Controls

### Navigation

| Action | Control |
|--------|---------|
| Pan | Left click + drag |
| Rotate | Middle click + drag / Control + drag |
| Zoom | Scroll wheel / pinch |
| Select | Right click |

### View

| Action | Control |
|--------|---------|
| Toggle pane | Tab |
| Relation pane | R key |
| Inference pane | I key |
| Deselect | Escape |

---

## Layout

Force-directed layout simulation positions nodes and edges automatically. The simulation uses:

- Repulsion between nodes
- Attraction along edges
- Boundary constraints

---

## Structure

```
Molten/system/spatialize/
  command.rs        Application entry point
  pane.rs           Visualization pane modes
  view.rs           View state and transformations
  layout.rs         Force-directed simulation
  scene.rs          Scene graph management
  render.rs         Render submission
  mouse.rs          Input state tracking
  palette.rs        Color definitions
```

---

(c) 2025 Vantle
