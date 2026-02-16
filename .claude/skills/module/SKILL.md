---
name: module
description: Create a new Vantle module with the correct directory, Rust source, and BUILD.bazel following all CLAUDE.md conventions. Use when user says 'new module', 'create module', 'add module', or wants to add a new component/system.
allowed-tools: Read, Glob, Grep, Bash(ls:*), Bash(mkdir:*), Bash(bazel build:*), Write, Edit
disable-model-invocation: true
---

# Module Creation

## Activation

- [ ] EVALUATE: Does this request involve creating a new module, component, or system?
- [ ] DECIDE: "This task requires /module because..."
- [ ] EXECUTE: Follow the workflow below

Create new Vantle modules following all CLAUDE.md conventions.

## Arguments

The user provides a target path relative to the repository root:

```
/module Molten/system/arena/cache
/module component/design/palette
/module system/concurrent/pool
```

The **last** segment is the module name. Everything before it is the parent.

## Conventions Checklist

Before generating any file, verify against CLAUDE.md:

- [ ] **Single-word name** — no underscores, no abbreviations, no CamelCase
- [ ] **Singular directory** — `test/resource` not `tests/resources`
- [ ] **No `mod` directives** — parent uses `pub use child;`
- [ ] **Target name = crate name** — `name = "cache"` in BUILD.bazel
- [ ] **Miette errors** — any error type uses `miette::Diagnostic` + `thiserror::Error`
- [ ] **No comments** — code self-documents

## Process

### Step 1: Validate Name

Reject if the module name contains underscores, abbreviations, or multiple words. Suggest alternatives.

### Step 2: Create Directory

```bash
mkdir -p <path>
```

### Step 3: Generate Source File

Create `<name>.rs` with this structure:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to <action>")]
    #[diagnostic(code(<module>::failed), help("<guidance>"))]
    <Variant>,
}
```

Adapt the error variants and imports based on what the module will do. If the user describes the module's purpose, generate appropriate types and traits. If no purpose is given, generate only the error enum as a starting point.

### Step 4: Generate BUILD.bazel

```starlark
load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_visibility = ["//visibility:public"])

##### Module [ Module ]

rust_library(
    name = "<name>",
    srcs = ["<name>.rs"],
    deps = [
        "@crates//:miette",
        "@crates//:thiserror",
    ],
)

##### Documentation [ Documentation ]
```

Add internal deps (`//path/to:target`) based on what the module needs. Add external deps (`@crates//:name`) as needed.

### Step 5: Update Parent

Read the parent module's `.rs` file and add `pub use <name>;` to its re-exports. Read the parent's `BUILD.bazel` and add `"//<parent_path>:<name>"` to its `deps`.

### Step 6: Verify Structure

Confirm the final layout:

```
parent/
  parent.rs          (now contains: pub use <name>;)
  <name>/
    <name>.rs
    BUILD.bazel
  BUILD.bazel        (now deps on //<parent>/<name>:<name>)
```

## Examples

### Leaf module

```
/module Molten/system/arena/cache
```

Creates:
- `Molten/system/arena/cache/cache.rs` — error enum + types
- `Molten/system/arena/cache/BUILD.bazel` — rust_library target
- Updates `Molten/system/arena/arena.rs` — adds `pub use cache;`
- Updates `Molten/system/arena/BUILD.bazel` — adds dep

### Module with children

If the user specifies children:

```
/module Molten/system/forge/runtime with evaluate, step
```

Creates the parent `runtime` module plus child directories `evaluate/` and `step/`, each with their own `.rs` and `BUILD.bazel`. The parent `runtime.rs` contains `pub use evaluate;` and `pub use step;`.

## Validation

After creation, run:

```bash
bazel build //<path>:target
```

Report any build errors and fix them.
