---
name: dependency
description: Audit and update all project dependencies. Checks BCR modules, Rust crates, toolchains, and HTTP archives for newer versions. Use when user says 'update deps', 'check versions', 'dependency', 'upgrade', or 'outdated'.
allowed-tools: Read, Grep, Glob, WebFetch(domain:crates.io), WebFetch(domain:registry.bazel.build), WebFetch(domain:docs.rs), WebSearch, Edit, Bash(bazel sync:*), Bash(bazel build:*), Bash(bazel test:*), Bash(cargo search:*), Bash(bazel run:*), Bash(CARGO_BAZEL_REPIN=true bazel build:*)
disable-model-invocation: true
---

# Dependency Audit & Update

## Activation

- [ ] EVALUATE: Does this request involve checking, auditing, or updating dependencies?
- [ ] DECIDE: "This task requires /dependency because..."
- [ ] EXECUTE: Follow the workflow below

Audit all project dependencies in `MODULE.bazel` and update them to their latest compatible versions.

## Dependency Types

All dependencies live in `MODULE.bazel`. There are five types:

| Type | Pattern | Count | Source |
|------|---------|-------|--------|
| BCR modules | `bazel_dep(name, version)` | ~14 | registry.bazel.build |
| Rust crates | `crate.spec(package, version)` | ~54 | crates.io |
| Rust toolchain | `rust.toolchain(versions)` | 1 | rust-lang.org |
| Python toolchain | `python.toolchain(python_version)` | 1 | python.org |
| HTTP archives | `vnu(urls)` | 1 | npmjs.org |

## Process

### Step 1: Parse Current Versions

Read `MODULE.bazel` and extract all dependencies with their current versions.

### Step 2: Check Latest Versions

For each dependency type, query the appropriate source:

**BCR Modules:**
```
WebSearch: "bazel_dep <name> latest version site:registry.bazel.build"
```

**Rust Crates:**
```
WebFetch: https://crates.io/api/v1/crates/<package>
```
Parse the JSON response for `crate.max_stable_version` or `crate.newest_version`.

**Rust Toolchain:**
```
WebSearch: "latest stable rust version 2026"
```

**Python Toolchain:**
```
WebSearch: "latest python 3.x stable release 2026"
```

**vnu HTTP Archive:**
```
WebFetch: https://registry.npmjs.org/vnu-jar/latest
```
Parse for `version` and `dist.tarball` (for URL) and `dist.shasum` (for SHA256).

### Step 3: Report

Present findings in a table:

```markdown
## Dependency Audit Report

### Outdated

| Name | Type | Current | Latest | Notes |
|------|------|---------|--------|-------|
| serde | crate | 1.0.228 | 1.0.235 | |
| rules_rust | BCR | 0.68.1 | 0.70.0 | aligned with prost/wasm |

### Up to Date

| Name | Type | Version |
|------|------|---------|
| miette | crate | 7.6.0 |

### Update N dependencies?
```

### Step 4: Update (on user approval)

Edit `MODULE.bazel` to update version strings. Follow alignment rules below.

### Step 5: Repin & Verify

```bash
bazel sync --config=sync
bazel build //...
bazel test //...
```

Report any build or test failures. If failures occur, identify the breaking dependency and suggest rollback or fix.

## Version Alignment Rules

These dependency groups must stay version-aligned. Update all members together:

### WASM Ecosystem (exact pins)
- `wasm-bindgen` (=X.Y.Z)
- `wasm-bindgen-futures` (=X.Y.Z)
- `web-sys` (=X.Y.Z)
- `js-sys` (=X.Y.Z)

### Protobuf/gRPC
- `prost` + `prost-types` (same version)
- `tonic` + `tonic-prost` (same version)
- `protoc-gen-prost` + `protoc-gen-tonic` (same version)

### Rules Rust
- `rules_rust` + `rules_rust_prost` + `rules_rust_wasm_bindgen` (same version)

### Tracing
- `tracing` + `tracing-core` + `tracing-subscriber` + `tracing-chrome` (compatible versions)

## Preservation Rules

When updating `MODULE.bazel`, preserve:
- All `features` lists exactly as they are
- All `default_features = False` flags
- All `crate.annotation()` blocks (gen_binaries, gen_build_script)
- All section comments (`##### Crates`, etc.)
- The exact pin syntax (`=X.Y.Z`) for WASM crates
- The `crate.from_specs()` and `use_repo(crate, "crates")` calls at the end

## Arguments

The user may specify scope:

```
/dependency              # audit all
/dependency crates       # audit only Rust crates
/dependency bcr          # audit only BCR modules
/dependency toolchain    # audit only toolchains
/dependency <name>       # audit a specific dependency
```
