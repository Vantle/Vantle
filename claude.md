# CLAUDE.md

**Vantle** implements **Molten**, a computational expression language over hypergraphs.

Rust 2021 | Bazel 9+ | Autotest testing
Docs: @Molten/index.html | Deps: @MODULE.bazel

---

## Principles

**DRY** — Never duplicate logic. Factor shared behavior into composable abstractions.
**SRP** — Each module, function, and type has exactly one responsibility.

When you see an abstraction opportunity, take it. Prefer composition over repetition.

---

## Naming

*Module namespacing replaces compound names; single words enable polymorphic composition.*

| Rule | Example |
| ------ | --------- |
| Single-word identifiers | `channel::Specification` not `ChannelSpec` |
| No underscores | `coalesce(particles)` not `coalesce_particles` |
| No abbreviations | `configuration` not `cfg`, `specification` not `spec` |
| Singular directories | `test/resource` not `tests/resources` |

Exceptions: Underscore re-exports are acceptable only when wrapping or exposing external APIs (e.g., `pub use tracing_subscriber;`). Naming conventions may be violated when compliance is impossible (e.g., `returns/` when `return` is a reserved keyword).

---

## Modules

*Bazel manages module resolution; re-exports define public API surface.*

| Rule | Good | Bad |
| ------ | ------ | ----- |
| No `mod` directive | `pub use collision;` | `mod collision;` |
| Re-export owned children only | `pub use child;` (child/ exists) | `pub use sibling;` |
| Depth limit: one level | `pub use child;` | `pub use a::b::c;` |
| No globs | `pub use child;` | `pub use child::*;` |
| Keep imports local | `use dep::Thing;` | `pub use dep::Thing;` |

---

## Code Quality

*Miette enables rich diagnostics; self-documenting code reduces maintenance.*

**Errors** — Always use `miette` with context, codes, and suggestions:

```rust
#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("failed to match particles")]
    #[diagnostic(code(matching::failed), help("ensure particles are compatible"))]
    Match,
}
```

**Style**:

- No comments — code self-documents
- Turbofish at call sites: `iter.collect::<Vec<_>>()`
- Grouped imports: `use glyphon::{Attrs, Buffer, Cache};`
- Early returns over nesting; combinators over if-else
- Small, focused functions with one responsibility
- `#[expect(...)]` at system boundaries only, never `#[allow(clippy::...)]`
- Run `rustfmt --edition 2024` and fix all clippy warnings before building

**API Changes**:

- Always clean break — no backward compatibility shims, aliases, or deprecation paths
- Remove old APIs entirely when replacing them

---

## Visual Design

*Golden ratio creates consistent proportions across all UI elements.*

```rust
const PHI: f32 = 1.618_033_988_749_895;
fn scale(k: f32) -> f32 { PHI.powf(k) }
```

Apply to: zoom, spacing, sizes, aspect ratios, animation timing.

---

## Commands

```bash
bazel build //...                              # build all
bazel test //...                               # test all
bazel run //Molten/system/forge:command lava   # interactive runtime
rustfmt --edition 2024 $(find . -name "*.rs")  # format
```

---

## Testing

*Never write manual Rust test functions.* Use Autotest (JSON-driven generation):

1. Template functions in `.template.rs`
2. Test cases in `cases.json`
3. Build rule: `rust_autotest()` in BUILD.bazel

See @Molten/test/resource for patterns.

---

## Bazel

Target name = crate name. Use `crate_name` only for:

- Root modules: `name = "module"` with `crate_name = "actual"`
- Conditional compilation: multiple targets, same crate
- Reserved keywords as target names

```python
rust_library(name = "wave", srcs = ["wave.rs"])
rust_library(name = "module", srcs = ["vantle.rs"], crate_name = "vantle")
```

---

## Structure

```txt
Vantle/
├── Molten/
│   ├── system/      # Core: forge, graph, hypergraph, query
│   ├── component/   # Reusable: arena, graph, hypergraph
│   └── test/        # Test suites
├── component/       # Platform components
├── system/          # Platform systems
├── test/            # Platform tests
└── platform/        # Platform definitions
```
