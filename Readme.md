<p align="center"><img src="resource/logo.png" alt="Vantle logo" width="61.8%"/></p>
<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Vantle</h1>
<p align="center"><em>Platform for everything</em></p>

<p align="center">
  <a href="Info.md"><strong>Info</strong></a> &nbsp;|&nbsp;
  <a href="Notice.md"><strong>Notice</strong></a> &nbsp;|&nbsp;
  <a href="MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp;
  <a href="Molten/Readme.md"><strong>Molten</strong></a> &nbsp;|&nbsp;
  <a href="system/generation/Readme.md"><strong>Generation</strong></a> &nbsp;|&nbsp;
  <a href="system/observation/Readme.md"><strong>Observation</strong></a> &nbsp;|&nbsp;
  <a href="system/spatialize/Readme.md"><strong>Spatialize</strong></a> &nbsp;|&nbsp;
  <a href="License.md"><strong>License</strong></a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License"/>
  <img src="https://img.shields.io/badge/language-Molten-orange.svg" alt="Molten"/>
  <img src="https://img.shields.io/badge/build-Bazel%209-green.svg" alt="Bazel"/>
</p>

---

**Vantle** is a platform for software research and experimentation.

---

## Features

### Molten

> *Computational expression over hypergraphs*

An AI frontend language designed for continual learning algorithms. Build hypergraphs through polymorphic relations, enabling declarative computation with concepts, orthogonalities, and transformations evaluated with temporal semantics.

[more →](Molten/Readme.md)

### Generation

> *Code generation framework for Rust*

Generate test suites from templates and JSON case definitions. The **autotest** system eliminates boilerplate while enabling data-driven testing with parameter shadowing and tag organization.

[more →](system/generation/Readme.md)

### Observation

> *Trace streaming and recording*

Stream traces peer-to-peer without a central server. The `#[trace]` macro instruments functions with channel-based filtering for selective observation to files or remote peers.

[more →](system/observation/Readme.md)

### Spatialize

> *GPU rendering infrastructure*

Render with wgpu using assembler-pattern context creation and frame-based draw submission. Golden ratio scaling utilities ensure harmonious visual proportions throughout.

[more →](system/spatialize/Readme.md)

---

## Quick Start

### Requirements

- [Bazel](https://bazel.build/) ≥ 9.0.0

### Build

```bash
bazel build //...
```

### Test

```bash
bazel test //...
```

---

© 2025 Vantle · @robert.vanderzee
