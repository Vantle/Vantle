# Documentation Structure

This guide defines the structural patterns for Vantle documentation.

## Readme.md Template

Every component Readme.md follows this structure:

```markdown
<p align="center"><img src="resource/logo.png" alt="Name logo" width="61.8%"/></p>
<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Name</h1>
<p align="center"><em>Brief description of what this does</em></p>

<p align="center">
  <a href="../Readme.md"><strong>Parent</strong></a> &nbsp;|&nbsp;
  <a href="../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp;
  <a href="License.md"><strong>License</strong></a>
</p>

---

This document describes **Component Name**, a brief explanation of purpose.

---

## Primary Feature

Description of the main feature or concept.

### Invoke

How to run or use this component:

```bash
bazel run //path/to:target -- --flag value
```

### Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `--flag` | What it does | value |

---

## Secondary Feature

Additional features or concepts.

### Usage

```rust
use component::path::Type;

fn example() {
    // Usage example
}
```

---

## Structure

```
component/
  subdir/     Description of subdir
  file.rs     Description of file
```

---

(c) 2025 Vantle
```

## Required Sections

Every Readme.md must include:

1. **Header Block**: Logo (if exists), title, tagline, navigation
2. **Introduction**: One paragraph explaining what this component does
3. **Primary Feature**: Main functionality with usage example
4. **Structure**: Directory layout showing component organization

## Optional Sections

Include when applicable:

- **Arguments/Options**: For CLI tools or configurable components
- **Examples**: Additional usage examples beyond basic usage
- **Theory**: Conceptual background for complex systems
- **Features**: Feature list for multi-feature components

## Section Order

Maintain consistent ordering:

1. Header (logo, title, tagline, nav)
2. Introduction paragraph
3. Theory/Concepts (if needed)
4. Primary feature (Invoke/Usage)
5. Secondary features
6. Examples
7. Structure
8. Footer

## Code Examples

### Bazel Commands

```bash
# Build
bazel build //path/to:target

# Test
bazel test //path/to:test

# Run
bazel run //path/to:command -- --flag value
```

### Rust Code

```rust
use component::module::Type;

fn example(input: Type) -> Result<Output, Error> {
    input.method()
}
```

### JSON Configuration

```json
{
  "key": "value",
  "nested": {
    "field": 123
  }
}
```

## Link Conventions

### Relative Paths

Always use relative paths for internal documentation:

```markdown
[Parent](../Readme.md)
[Sibling](../sibling/Readme.md)
[Child](child/Readme.md)
```

### Standard Links

Every Readme.md should link to:

| Link | Path | Purpose |
|------|------|---------|
| Vantle | `../../Readme.md` (adjust depth) | Root documentation |
| Module | `../../MODULE.bazel` | Dependencies |
| License | `../../License.md` | Licensing |

### Related Documentation

Link to related components:

```markdown
See [Observation](../observation/Readme.md) for tracing.
```

## Structure Diagrams

Use code blocks for directory structure:

```
component/
  module/       Core module implementation
  resource/     Static resources and templates
  test/         Test suites
```

Rules:
- Two-space indent per level
- Short descriptions aligned
- Only show relevant directories/files

## Completeness Checklist

- [ ] Header with title and tagline
- [ ] Navigation links to parent and related docs
- [ ] Introduction paragraph explaining purpose
- [ ] At least one usage example with code
- [ ] Structure diagram showing layout
- [ ] All code examples compile and work
- [ ] All internal links resolve
- [ ] No orphaned sections without content
