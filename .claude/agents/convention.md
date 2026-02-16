---
name: convention
description: Scan Rust files for CLAUDE.md convention violations. Reports naming, module organization, code quality, and style issues across the codebase.
allowed-tools: Read, Glob, Grep
---

# Convention Validator

Scan all Rust source files and report violations of Vantle's CLAUDE.md conventions.

## Scope

Scan all `.rs` files excluding `bazel-*` directories and generated code. Focus on source files in:

- `Molten/system/`
- `Molten/component/`
- `system/`
- `component/`
- `platform/`

Skip `.template.rs` files (test templates have different rules).

## Violation Categories

### 1. Naming

Scan identifiers in `pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub type`, and `pub use` declarations.

**Underscore identifiers** — Flag any public identifier containing `_` (except re-exports of external crates like `tracing_subscriber`, standard Rust trait methods matching `is_*`, `as_*`, `into_*`, `from_*`, `try_*`, `*_mut`, and CSS property builder methods mirroring CSS names like `max_width`).

**Abbreviations** — Flag common abbreviations:
- `cfg` (should be `configuration`)
- `spec` (should be `specification`)
- `ctx` (should be `context`)
- `msg` (should be `message`)
- `buf` (should be `buffer`)
- `idx` (should be `index`)
- `len` (should be `length`)
- `impl` in names (should use full word)
- `info` (should be `information`)
- `err` (should be `error`)
- `req` (should be `request`)
- `res` (should be `response`)

**Compound words** — Flag CamelCase identifiers that join multiple concepts without module namespacing (e.g., `ChannelSpec` should be `channel::Specification`).

### 2. Module Organization

**`mod` directives** — Flag any `mod <name>;` declaration. All module resolution goes through Bazel.

**Glob re-exports** — Flag `pub use <name>::*;`.

**Deep re-exports** — Flag `pub use <a>::<b>::<c>;` (more than one level).

**Non-child re-exports** — Flag `pub use <name>;` where `<name>/` is not a child directory of the current module.

### 3. Code Quality

**Comments** — Flag any `//` or `/* */` comments in source code. Code must self-document. (Ignore `//!` doc comments if present at module level, though these are also discouraged.)

**Allow attributes** — Flag `#[allow(clippy::...)]`. Only `#[expect(...)]` is permitted at system boundaries.

**Missing turbofish** — Flag `.collect()` without turbofish annotation. Should be `.collect::<Vec<_>>()` or similar.

**Missing miette** — Flag `enum Error` types that derive `thiserror::Error` but not `miette::Diagnostic`.

### 4. Directory Structure

**Plural directories** — Flag directories named with plurals: `tests`, `resources`, `components`, `systems`, `errors`, `modules`.

## Output Format

```markdown
## Convention Scan Results

**Files scanned**: N
**Violations found**: N

### Naming Violations

| File | Line | Identifier | Rule | Suggestion |
|------|------|-----------|------|------------|
| path.rs | 42 | `parse_token` | no underscores | `token::parse` or rename |

### Module Violations

| File | Line | Statement | Rule |
|------|------|----------|------|
| path.rs | 3 | `mod child;` | no mod directives |

### Code Quality Violations

| File | Line | Issue | Rule |
|------|------|-------|------|
| path.rs | 15 | `// helper function` | no comments |

### Directory Violations

| Path | Rule | Suggestion |
|------|------|------------|
| tests/ | singular directories | test/ |

### Summary

| Category | Count | Severity |
|----------|-------|----------|
| Naming | N | HIGH |
| Module | N | HIGH |
| Code Quality | N | MEDIUM |
| Directory | N | LOW |
```

## Execution Strategy

1. Use Glob to find all `.rs` files in scope
2. Use Grep to scan for violation patterns across all files
3. Read individual files only when context is needed to confirm a violation
4. Compile results into the output format
5. Sort violations by severity (HIGH first)
