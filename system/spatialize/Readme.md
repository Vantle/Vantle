<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Spatialize</h1>
<p align="center"><em>GPU rendering infrastructure for Vantle</em></p>

<p align="center">
  <a href="../../Readme.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../../Molten/system/spatialize/Readme.md"><strong>Molten Spatialize</strong></a> &nbsp;|&nbsp; <a href="../../License.md"><strong>License</strong></a>
</p>

---

This document describes the **Spatialize** system, a GPU rendering infrastructure built on wgpu.

---

## Context

The render context manages GPU resources and pipeline state.

### Assembler

Build a rendering context with the assembler pattern:

```rust
use render::{Assembler, Context};

let context = Assembler::new()
    .surface(surface)
    .adapter(adapter)
    .size(width, height)
    .assemble()
    .await?;
```

| Field | Description |
|-------|-------------|
| `surface` | wgpu surface for presentation |
| `adapter` | wgpu adapter for device creation |
| `size` | Initial viewport dimensions |

### Pipelines

Build GPU pipelines with the raster and compute assemblers:

```rust
use raster::Raster;

let pipeline = Raster::assembler()
    .shader("path/to/pipeline.wgsl")
    .vertex(Vertex::layout())
    .bind(0, Binding::uniform(wgpu::ShaderStages::VERTEX))
    .target(format, Some(wgpu::BlendState::ALPHA_BLENDING))
    .assemble(device)?;
```

---

## Structure

```text
system/spatialize/
  render/           GPU pipeline and frame management
  interact/         Input and collision systems
  proportion.rs     Golden ratio utilities
```

---

© 2025 Vantle
