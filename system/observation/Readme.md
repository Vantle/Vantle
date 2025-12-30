<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Observation</h1>
<p align="center"><em>Trace streaming and recording for Vantle</em></p>

<p align="center">
  <a href="../../Readme.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../../License.md"><strong>License</strong></a>
</p>

---

This document describes the **Observation** system, a peer-to-peer trace streaming framework for Vantle.

---

## Architecture

Observation uses a peer-to-peer model with no central server. Applications stream traces directly to:

- **Files**: Local recording via `file://` URIs
- **Peers**: Remote streaming via `grpc://` URIs

Each application decides where to send its traces. See [Forge](../../Molten/Readme.md#observation) for an example of configuring trace destinations.

---

## Trace

The `#[trace]` macro instruments functions for observation.

### Usage

```rust
#[trace(channels = [core])]
fn process() {
    evaluate();
}
```

### Channels

Channels filter which spans to observe. Common channels include:

- `core`: Core runtime operations
- `analysis`: Analysis and evaluation
- `debug`: Debugging and diagnostics

---

## Structure

```text
component/observation/     Streaming layer and span types
system/observation/        Trace initialization and encoding
```

---

(c) 2025 Vantle
