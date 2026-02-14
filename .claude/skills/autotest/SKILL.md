---
name: autotest
description: Generate Autotest template and cases.json for a module. Creates the JSON-driven test scaffold with .template.rs, cases.json, and BUILD.bazel using rust_autotest. Use when user says 'add tests', 'autotest', 'generate tests', 'test cases', or wants to create test coverage for a module.
allowed-tools: Read, Glob, Grep, Bash(ls:*), Bash(mkdir:*), Bash(bazel build:*), Bash(bazel test:*), Write, Edit
disable-model-invocation: true
---

# Autotest Generation

## Activation

- [ ] EVALUATE: Does this request involve creating tests, adding test coverage, or generating test cases?
- [ ] DECIDE: "This task requires /autotest because..."
- [ ] EXECUTE: Follow the workflow below

Generate JSON-driven Autotest scaffolds following Vantle's testing conventions.

## Arguments

The user provides a module path:

```
/autotest Molten/system/arena/cache
/autotest system/observation/trace
```

## Process

### Step 1: Analyze the Module

Read the target module's `.rs` file to understand:

- Public functions and their signatures
- Input types and return types
- Error variants
- Key behaviors to test

### Step 2: Determine Test Location

Map the module path to the test resource directory:

| Module Path | Test Resource Path |
|---|---|
| `Molten/system/<mod>` | `Molten/test/resource/system/<mod>` |
| `Molten/component/<mod>` | `Molten/test/resource/component/<mod>` |
| `system/<mod>` | `test/resource/system/<mod>` |
| `component/<mod>` | `test/resource/component/<mod>` |

Create the test resource directory:

```bash
mkdir -p <test_resource_path>
```

### Step 3: Generate Template

Create `<name>.template.rs` with functions that exercise the module's public API:

```rust
use <crate>::path::to::Type;

fn <function_name>(<params>) -> <ReturnType> {
    // Call the function under test
    // Return the result for comparison with expected output
}
```

Rules:
- Import the module under test
- One template function per public function or behavior
- Parameters match the types in `cases.json`
- Return types must be serializable (for JSON comparison)
- No `#[test]` attributes — Autotest generates those
- No assertions — Autotest compares returns against `cases.json`

### Step 4: Generate Cases

Create `cases.json` with test data:

```json
{
  "functions": [
    {
      "function": "<function_name>",
      "cases": [
        {
          "tags": ["<category>"],
          "parameters": {
            "<param>": <value>
          },
          "returns": {
            "()": <expected>
          }
        }
      ]
    }
  ]
}
```

Rules:
- Cover happy path, edge cases, and error cases
- Use descriptive `tags` for each case: `["basic"]`, `["boundary"]`, `["empty"]`, `["error"]`
- Parameters and returns use JSON representations of Rust types
- `"()"` key in returns means the function returns the value directly
- Arrays represent `Vec` or tuple types
- Objects with string keys represent `HashMap` or struct fields
- `null` represents `None`

### Step 5: Generate BUILD.bazel

```starlark
load("//component/generation/starlark:defs.bzl", "rust_autotest", "rust_autotest_template")

package(default_visibility = ["//visibility:public"])

##### Test [ Test ]

filegroup(
    name = "cases",
    srcs = ["cases.json"],
)

rust_autotest_template(
    name = "template",
    src = "<name>.template.rs",
    deps = [
        "//path/to:module_under_test",
    ],
)
```

Add only the deps needed to compile the template functions.

### Step 6: Register in Parent Test BUILD

Check if a parent `BUILD.bazel` in the test directory needs a `rust_autotest` rule that references this template and cases. Look at sibling test directories for the pattern.

### Step 7: Verify

```bash
bazel build //<test_path>:template
bazel test //<test_path>/...
```

Report any failures and fix them.

## Example

For module `Molten/system/arena/cache` with public function:

```rust
pub fn evict(arena: &mut Arena, threshold: usize) -> Result<usize, Error>
```

Generate:

**`Molten/test/resource/system/arena/cache/cache.template.rs`**
```rust
use system::arena::cache;
use system::arena::Arena;

fn evict(arena: Arena, threshold: usize) -> usize {
    let mut arena = arena;
    cache::evict(&mut arena, threshold).unwrap()
}
```

**`Molten/test/resource/system/arena/cache/cases.json`**
```json
{
  "functions": [
    {
      "function": "evict",
      "cases": [
        {
          "tags": ["basic"],
          "parameters": {
            "arena": {"entries": [["a", 1], ["b", 2], ["c", 3]]},
            "threshold": 2
          },
          "returns": {
            "()": 1
          }
        },
        {
          "tags": ["empty"],
          "parameters": {
            "arena": {"entries": []},
            "threshold": 0
          },
          "returns": {
            "()": 0
          }
        },
        {
          "tags": ["boundary"],
          "parameters": {
            "arena": {"entries": [["a", 1]]},
            "threshold": 100
          },
          "returns": {
            "()": 0
          }
        }
      ]
    }
  ]
}
```

## Tag Conventions

| Tag | Meaning |
|---|---|
| `basic` | Happy path, straightforward input |
| `empty` | Empty collections, zero values |
| `boundary` | Edge cases, limits, extremes |
| `error` | Expected error conditions |
| `complete` | Full coverage of a specific behavior |
| `polymorphic` | Tests derivation or type polymorphism |
