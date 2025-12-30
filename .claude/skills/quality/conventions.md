# Vantle/Molten Code Conventions

This reference documents the project-specific conventions from CLAUDE.md with concrete examples.

## Naming Conventions

### Single-Word Names Only

Use concise, context-precise single words for all identifiers. NEVER use underscores. If multiple words are needed, use module namespacing to provide context.

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
pub struct ChannelSpec { ... }          // Should be channel::Specification
pub struct TraceArgs { ... }            // Should be trace::Arguments
pub fn coalesce_particles(...) { ... }  // Should be just coalesce()
fn get_user_data() { ... }              // Should be user::get() or just get()
let error_message = "...";              // Should be just message or error
```

### Full Descriptive Names

Never abbreviate. Use full words.

```rust
// GOOD
let specification = ...;
let configuration = ...;
let parameter = ...;

// BAD
let spec = ...;   // Use specification
let cfg = ...;    // Use configuration
let param = ...;  // Use parameter
```

### Directory and File Names

Use singular form for all directory and file names.

```text
GOOD: Molten/test/resource/system/hypergraph/
BAD:  Molten/tests/resources/system/hypergraph/
```

## Module Organization

### No `mod` Directive

Bazel handles module resolution. Use `pub use` for re-exports.

```rust
// GOOD
pub use component;
pub use translate;

// BAD - never use mod
mod component;
mod translate;
```

### Maximum Re-export Depth is One Level

Only re-export direct children, never deeper paths.

```rust
// GOOD - in module root
pub use component;
pub use translate;

// BAD - too deep
pub use component::graph;
pub use component::graph::state;
pub use component::graph::state::particle;
```

### Never Glob Re-export

```rust
// GOOD
pub use component;

// BAD - never use glob
pub use component::*;
```

### Deep Imports for Local Use Only

You can import deeply for local use, just don't re-export.

```rust
// GOOD - local usage in function body
use component::graph::state::particle::Particle;
fn process(p: Particle) { ... }

// BAD - re-exporting deep path
pub use component::graph::state::particle;
```

## Code Style

### No Comments

Code must be self-documenting through clear naming. No inline comments, no function comments.

```rust
// GOOD - self-documenting code
fn validate(input: &str) -> Result<Token, parse::Error> {
    input
        .chars()
        .all(|c| c.is_alphanumeric())
        .then(|| Token::new(input))
        .ok_or(parse::Error::Character)
}

// BAD - comments explaining code
fn validate(input: &str) -> Result<Token, parse::Error> {
    // Check if all characters are alphanumeric
    for c in input.chars() {
        if !c.is_alphanumeric() {
            return Err(parse::Error::Character); // Return error for invalid char
        }
    }
    Ok(Token::new(input)) // Return success
}
```

Exception: Doc comments (`///`) on public APIs for generated docs only.

### Turbofish Over Type Annotations

Prefer `::<Type>` at call sites over type annotations on bindings.

```rust
// GOOD
let items = iter.collect::<Vec<_>>();
let map = entries.collect::<HashMap<_, _>>();
let parsed = value.parse::<i32>()?;

// BAD
let items: Vec<_> = iter.collect();
let map: HashMap<_, _> = entries.collect();
let parsed: i32 = value.parse()?;
```

### Early Returns Over Deep Nesting

```rust
// GOOD - early returns
fn process(input: Option<&str>) -> Result<Output, Error> {
    let input = input.ok_or(Error::Missing)?;

    if input.is_empty() {
        return Err(Error::Empty);
    }

    let parsed = parse(input)?;
    Ok(transform(parsed))
}

// BAD - deep nesting
fn process(input: Option<&str>) -> Result<Output, Error> {
    if let Some(input) = input {
        if !input.is_empty() {
            if let Ok(parsed) = parse(input) {
                return Ok(transform(parsed));
            } else {
                return Err(Error::Parse);
            }
        } else {
            return Err(Error::Empty);
        }
    } else {
        return Err(Error::Missing);
    }
}
```

### Functional Combinators Over If-Else

```rust
// GOOD - functional style
fn find(id: &str) -> Result<Item, Error> {
    cache
        .get(id)
        .or_else(|| database.fetch(id))
        .ok_or(Error::NotFound)
}

fn transform(value: Option<i32>) -> Option<String> {
    value
        .filter(|v| *v > 0)
        .map(|v| v * 2)
        .map(|v| format!("{}", v))
}

// BAD - explicit if-else
fn find(id: &str) -> Result<Item, Error> {
    if let Some(item) = cache.get(id) {
        Ok(item)
    } else if let Some(item) = database.fetch(id) {
        Ok(item)
    } else {
        Err(Error::NotFound)
    }
}
```

### Small, Focused Functions

Each function should do one thing. If a function is longer than 20-30 lines, consider splitting it.

```rust
// GOOD - small, focused functions
fn parse(input: &str) -> Result<Ast, Error> {
    let tokens = tokenize(input)?;
    let tree = build(tokens)?;
    validate(tree)
}

fn tokenize(input: &str) -> Result<Vec<Token>, Error> { ... }
fn build(tokens: Vec<Token>) -> Result<Ast, Error> { ... }
fn validate(tree: Ast) -> Result<Ast, Error> { ... }

// BAD - monolithic function doing everything
fn parse(input: &str) -> Result<Ast, Error> {
    // 100+ lines of tokenizing, building, and validating all mixed together
}
```

## Error Handling

### Always Use Miette

```rust
// GOOD - miette with rich diagnostics
use miette::{Diagnostic, Result};
use thiserror::Error;

mod parse {
    #[derive(Error, Diagnostic, Debug)]
    pub enum Error {
        #[error("Invalid token at position {position}")]
        #[diagnostic(
            code(parse::token),
            help("Expected one of: {expected}")
        )]
        Token {
            position: usize,
            expected: String,
        },

        #[error("Unexpected end of input")]
        #[diagnostic(
            code(parse::eof),
            help("The expression appears to be incomplete")
        )]
        Eof,
    }
}

// BAD - plain errors without diagnostics
#[derive(Debug)]
pub enum ParseError {  // BAD: compound name
    InvalidToken(usize),  // BAD: compound variant
    UnexpectedEof,  // BAD: compound variant
}
```

## Formatting

Always run `rustfmt --edition 2021` before building. No clippy suppressions allowed.

```bash
# Format all Rust files
rustfmt --edition 2021 $(find . -name "*.rs" -not -path "./bazel-*")
```
