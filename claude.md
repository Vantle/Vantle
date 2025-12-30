# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

**Vantle** is a platform implementing **Molten**, a computational expression language over hypergraphs.

- **Language**: Rust (edition 2021)
- **Build System**: Bazel 8.0+
- **Runtime**: Forge interactive runtime
- **Testing**: Autotest framework (JSON-driven test generation)

See @Molten/Readme.md for Molten language documentation.
See @MODULE.bazel for dependencies and versions.

---

## CRITICAL: Naming Conventions

**IMPORTANT: YOU MUST follow these naming rules exactly.**

### Single-Word Names Only

- Use concise, context-precise single words for all identifiers
- **NEVER use underscores** in variable names, function names, or type names
- If multiple words are needed, use module namespacing to provide context
- Use full descriptive names, never abbreviate

```rust
// GOOD - use namespacing for multi-word concepts
mod channel {
    pub struct Specification { ... }
}
mod trace {
    pub struct Arguments { ... }
}
pub fn coalesce(particles: &[Particle]) -> Wave { ... }

// BAD - compound names or abbreviations
pub struct ChannelSpec { ... }
pub struct TraceArgs { ... }
pub fn coalesce_particles(...) { ... }
```

### Singular Directory and File Names

- Use **singular form** for all directory and file names
- Prefer `resource` over `resources`, `test` over `tests`

```text
GOOD: Molten/test/resource/system/hypergraph/
BAD:  Molten/tests/resources/system/hypergraph/
```

### Test Tags

- Same single-word rule applies to test tags
- No underscores in tags

```json
// GOOD
"tags": ["superset", "residue", "multiple"]

// BAD
"tags": ["superset_residue", "multiple_matches"]
```

---

## CRITICAL: Module Organization

**IMPORTANT: YOU MUST follow these module rules exactly.**

- **Never use `mod` directive** - Bazel works with `pub use` instead
- **Maximum re-export depth is one level** - Use `pub use a;` only
- **Never glob re-export** - Never use `pub use a::b::*`
- **Deep imports are local only** - Import `a::b::c::Thing` locally, never re-export

```rust
// GOOD - in module root
pub use component;
pub use translate;

// GOOD - local usage in function
use component::graph::state::particle::Particle;

// BAD - too deep for re-export
pub use component::graph::state::particle;

// BAD - glob import
pub use component::*;
```

---

## CRITICAL: Code Quality

**IMPORTANT: YOU MUST follow these rules.**

### No Comments

- **Never add comments** - Code must be self-documenting through clear naming
- No inline comments, no function comments
- Exception: Doc comments (`///`) on public APIs for generated docs only

### Error Handling

- **Always use `miette`** for error reporting with rich context and suggestions

```rust
#[derive(Error, Diagnostic, Debug)]
pub enum MyError {
    #[error("Failed to match particles")]
    #[diagnostic(code(matching::failed), help("Ensure particles are compatible"))]
    MatchFailed,
}
```

### Type Annotations

- Prefer turbofish `::<Type>` at call sites over type annotations on bindings

```rust
// GOOD
let items = iter.collect::<Vec<_>>();

// BAD
let items: Vec<_> = iter.collect();
```

### Formatting and Linting

- Run `rustfmt --edition 2021` before building
- Fix all clippy warnings - never use `#[allow(clippy::...)]`

### Function Design

- Keep functions small and focused
- Prefer early returns over deep nesting
- Prefer functional combinators (`.and_then()`, `.map()`, `.ok_or()`) over explicit if-else

---

## Build and Test Commands

```bash
# Build
bazel build //...                              # Build all
bazel build //Molten/system/forge:command      # Build Forge

# Test
bazel test //...                               # Test all
bazel test //Molten/test/component/...         # Component tests
bazel test //Molten/test/system/...            # System tests

# Run Forge
bazel run //Molten/system/forge:command lava   # Interactive runtime

# Format (required before building)
rustfmt --edition 2021 $(find . -name "*.rs")
```

---

## Project Structure

```text
Vantle/
├── Molten/
│   ├── system/        # Core implementations (forge, graph, hypergraph, query)
│   ├── component/     # Reusable components (arena, graph, hypergraph)
│   └── test/          # Test suites and resources
├── component/         # Platform components (generation framework)
├── system/            # Platform system modules (generation, logging)
├── test/              # Platform tests
└── platform/          # Platform definitions
```

---

## Testing: Autotest Framework

Tests use a JSON-driven generation framework. **Never write manual Rust test functions.**

### Pattern

1. **Template**: Write functions in `.template.rs`
2. **Cases**: Define test data in `cases.json`
3. **Build Rule**: Use `rust_autotest()` in BUILD.bazel

See @Molten/test/resource for existing test patterns.

### JSON Structure

```json
{
  "functions": [{
    "function": "disjoint",
    "tags": ["particle", "set"],
    "parameters": { "candidate": [["a", 1]], "basis": [["b", 1]] },
    "returns": { "()": [["a", 1]] },
    "cases": [{
      "tags": ["overlap"],
      "parameters": { "basis": [["a", 3]] },
      "returns": { "()": null }
    }]
  }]
}
```

- Function-level parameters are defaults; case-level parameters override them
- Use multiple function entries with different default parameters to reduce repetition

### BUILD.bazel

```python
rust_autotest(
    name = "particle",
    template = "//path/to:template",
    cases = "//path/to:cases.json",
    deps = ["//Molten/component/graph/state/particle:module"],
)
```

---

## Bazel Configuration

### Target Naming

- Only specify `crate_name` if it differs from target name

```python
# GOOD - target matches file name
rust_library(
    name = "wave",
    srcs = ["wave.rs"],
)

# GOOD - module needs explicit crate_name
rust_library(
    name = "module",
    srcs = ["implementation.rs"],
    crate_name = "module",
)

# BAD - unnecessary crate_name
rust_library(
    name = "particle",
    srcs = ["particle.rs"],
    crate_name = "particle",  # Redundant
)
```

---

## Development Workflow

### Before Committing

1. Format: `rustfmt --edition 2021 $(find . -name "*.rs")`
2. Test: `bazel test //...`
3. Fix clippy warnings (no suppressions)
4. Build: `bazel build //...`

### Code Review Checklist

- [ ] Single-word names only (no underscores)
- [ ] No comments
- [ ] Functions are small and focused
- [ ] Early returns, functional combinators
- [ ] No `mod` directives (`pub use` only)
- [ ] No deep re-exports
- [ ] No clippy suppressions
- [ ] Errors use miette
