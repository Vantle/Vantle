# Plan: LICENSE.md and NOTICE.md as source of truth

## Context

The project uses Apache 2.0 but has no traditional LICENSE or NOTICE files in the repo root — the license text is hardcoded in `style::license()` (style.rs:797-906) and notice text is hardcoded in the `.document.rs` generators. This means:
1. GitHub can't detect the license
2. Downstream consumers don't get LICENSE/NOTICE with clones
3. Content can drift between the hardcoded Rust and legal intent

We'll add `LICENSE.md` and `NOTICE.md` as plain markdown files at repo root, then make the document generators consume them via a markdown-to-HTML pipeline. The hardcoded `style::license()` function gets deleted.

---

## Changes

### 1. Add `LICENSE.md` at repo root

Full Apache 2.0 text in markdown format. Content extracted from the current `style::license()` function. Structured with markdown headings, definition lists, and bullet points so it's readable as-is and parses to structured HTML.

### 2. Add `NOTICE.md` at repo root

```markdown
Copyright 2025 Vantle

Licensed under the Apache License, Version 2.0.
```

### 3. Add `pulldown-cmark` dependency

Add to `MODULE.bazel`:
```python
crate.spec(
    package = "pulldown-cmark",
    version = "0.13.0",
)
```

This is the standard Rust markdown parser — fast, no-alloc where possible, CommonMark compliant.

### 4. New `Element::Markdown` variant + `Body::markdown()` method

Add to `component/web/element.rs`:
```rust
Markdown { name: String },
```

Add to `component/web/body.rs`:
```rust
pub fn markdown(mut self, name: &str) -> Self {
    self.elements.push(Element::Markdown { name: name.into() });
    self
}
```

In the HTML renderer (`system/generation/web/html.rs`), handle `Element::Markdown`:
1. Look up data file by name (same pattern as `inject`)
2. Parse with `pulldown_cmark::Parser`
3. Render to HTML via `pulldown_cmark::html::push_html()`
4. Push raw HTML string

The `pulldown-cmark` crate has a built-in HTML renderer — no need for a separate module. Add `pulldown-cmark` as a dep of `system/generation/web:html`.

### 6. Update document generators

**`license.document.rs`** (root):
```rust
fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "License", "vantle", "license", |c| {
        c.title("License").rule().markdown("LICENSE.md")
    })
}
```

**`Molten/license.document.rs`**: Same pattern, context = "molten".

**`notice.document.rs`** (root):
```rust
fn main() -> miette::Result<()> {
    let arguments = html::Arguments::parse();
    style::page(&arguments, "Notice", "vantle", "notice", |c| {
        c.title("Notice").rule().markdown("NOTICE.md")
    })
}
```

**`Molten/notice.document.rs`**: Same pattern, context = "molten".

### 7. Update BUILD.bazel targets

Add `data = ["//:LICENSE.md"]` to all license document targets.
Add `data = ["//:NOTICE.md"]` to all notice document targets.

Root BUILD.bazel license target:
```python
document(
    srcs = ["license.document.rs"],
    data = ["//:LICENSE.md"],
    destination = "license.html",
    deps = [...],
)
```

Similarly for `Molten/BUILD.bazel`.

### 8. Delete `style::license()`

Remove the `pub fn license(body: Body) -> Body` function (style.rs:797-906). Remove the `Composition::compose` import from license generators since they no longer call `style::license`.

### 9. Add `pulldown-cmark` to html target deps

The `system/generation/web:html` library needs `pulldown-cmark`. Add it to the BUILD.bazel deps for that target. The document generators don't need any new deps since the markdown rendering happens in the html renderer.

---

## Files

| File | Change |
|------|--------|
| `LICENSE.md` | **New** — Apache 2.0 full text in markdown |
| `NOTICE.md` | **New** — Copyright notice |
| `MODULE.bazel` | Add `pulldown-cmark` crate spec |
| `component/web/element.rs` | Add `Markdown { name: String }` variant |
| `component/web/body.rs` | Add `markdown()` method |
| `system/generation/web/html.rs` | Handle `Element::Markdown` in renderer |
| `system/generation/web/BUILD.bazel` | Add `pulldown-cmark` dep to `html` target |
| `system/generation/web/style.rs` | Delete `license()` function |
| `license.document.rs` | Use `.markdown("LICENSE.md")` |
| `notice.document.rs` | Use `.markdown("NOTICE.md")` |
| `Molten/license.document.rs` | Use `.markdown("LICENSE.md")` |
| `Molten/notice.document.rs` | Use `.markdown("NOTICE.md")` |
| `BUILD.bazel` | Add `data` to license/notice targets |
| `Molten/BUILD.bazel` | Add `data` to license/notice targets |

## Verification

1. `bazel build //:documentation` — all pages build
2. `bazel test //...` — all tests pass
3. Read rendered `license.html` — verify structured HTML from markdown
4. Read rendered `notice.html` — verify notice content
5. Confirm `LICENSE.md` at root is detected by GitHub
