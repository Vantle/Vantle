<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Generation</h1>
<p align="center"><em>Code generation framework for Rust</em></p>

<p align="center">
  <a href="../../Readme.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../../License.md"><strong>License</strong></a>
</p>

---

This document describes the **Generation** framework, a code generation system for Rust projects built on Bazel.

---

## Autotest

Autotest is an implementation of the Generation framework that provides JSON-driven test generation, eliminating boilerplate and enabling data-driven testing.

### Template

Write functions in a `.template.rs` file:

```rust
use component::graph::state::particle::Particle;

fn disjoint(candidate: Particle<String>, basis: Particle<String>) -> Option<Particle<String>> {
    candidate.disjoint(&basis).map(|_| candidate.clone())
}
```

### Cases

Define test data in `cases.json`:

```json
{
  "functions": [
    {
      "function": "disjoint",
      "tags": ["particle", "disjoint"],
      "parameters": {
        "candidate": [["a", 1]],
        "basis": [["b", 1]]
      },
      "returns": { "()": [["a", 1]] },
      "cases": [
        {
          "tags": ["empty"],
          "parameters": { "basis": [] },
          "returns": { "()": [["a", 1]] }
        }
      ]
    }
  ]
}
```

### Build

Use Bazel rules to generate and run tests:

```python
rust_autotest_template(
    name = "template",
    src = "function.template.rs",
    deps = ["//Molten/component/graph/state/particle:module"],
)

rust_autotest(
    name = "function",
    template = ":template",
    cases = ":cases.json",
    deps = ["//Molten/component/graph/state/particle:module"],
)
```

### Features

- **Parameter shadowing**: Function-level defaults with case-level overrides
- **Tag organization**: Filter tests by tags
- **Schema validation**: Parameters match function signatures
- **Rich diagnostics**: Error reporting via miette

---

## Structure

```
component/generation/     Schema and types
system/generation/        Generator binary
```

---

(c) 2025 Vantle
