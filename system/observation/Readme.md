<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Observation</h1>
<p align="center"><em>Trace streaming and recording for Vantle</em></p>

<p align="center">
  <a href="../../Readme.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../../License.md"><strong>License</strong></a>
</p>

---

This document describes the **Observation** system, a trace streaming and recording framework for Vantle built on gRPC.

---

## Portal

The observation server (portal) receives and stores traces from instrumented applications.

### Invoke

Run the portal server:

```bash
bazel run //system/observation:command -- --address 127.0.0.1:50051 --store file:///tmp/traces
```

### Arguments

- `--address`: Server address (default: 127.0.0.1:50051)
- `--capacity`: Broadcast buffer size (default: 4096)
- `--store`: URI for recording traces (currently supports `file://`)

---

## Trace

The `#[trace]` macro instruments functions for observation.

### Usage

```rust
#[trace(channels = [core])]
fn process() {
    // Function body is automatically instrumented
}
```

### Channels

Channels filter which spans to observe. Common channels include:

- `core`: Core runtime operations
- `analysis`: Analysis and evaluation
- `debug`: Debugging and diagnostics

---

## Structure

```
component/observation/     Streaming layer and types
system/observation/        Portal server and trace initialization
```
