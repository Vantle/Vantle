# Vantle Documentation Style

This guide defines the visual style for all Vantle documentation.

## Headers

### Main Title

Use centered HTML for the main title with inline style:

```html
<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Title</h1>
```

### Tagline

Centered emphasized text below the title:

```html
<p align="center"><em>Brief description of the component</em></p>
```

### Section Headers

Use standard markdown headers for sections:

```markdown
## Section Name

### Subsection Name

#### Detail Name
```

## Navigation

### Navigation Bar

Centered paragraph with bold links separated by `&nbsp;|&nbsp;`:

```html
<p align="center">
  <a href="path/to/Doc.md"><strong>Name</strong></a> &nbsp;|&nbsp; <a href="path/to/Other.md"><strong>Other</strong></a>
</p>
```

### Standard Links

Include links to:
- Parent Readme.md (`../../Readme.md` for Vantle root)
- MODULE.bazel for dependencies
- License.md for licensing
- Related component documentation

## Visual Elements

### Horizontal Rules

Use `---` to separate major sections:

```markdown
---

## New Section
```

Always place a horizontal rule:
- After the navigation bar
- After the introduction paragraph
- Between major conceptual sections

### Images

Use golden ratio proportions (61.8% width):

```html
<p align="center"><img src="path/to/image.png" alt="Description" width="61.8%"/></p>
```

### Tables

Use markdown tables for structured information:

```markdown
| Column | Column | Column |
|--------|--------|--------|
| Data   | Data   | Data   |
```

Appropriate uses:
- Syntax reference (symbol, action, elaboration)
- Command arguments
- Configuration options
- Status summaries

### Code Blocks

Always specify the language hint:

```markdown
```rust
fn example() {}
```

```bash
bazel run //target:name
```

```json
{"key": "value"}
```
```

### Blockquotes

Use for notes, warnings, or asides:

```markdown
> Note: Important information about this feature.
```

## Text Style

### Emphasis

- **Bold** for key terms on first use
- *Italic* for emphasis or technical terms
- `inline code` for commands, paths, function names

### Lists

- Use bullet points for unordered items
- Use numbered lists for sequential steps
- Keep list items concise and parallel

### Links

Use relative paths for internal links:

```markdown
[Link Text](../path/to/file.md)
```

## Footer

End with copyright line:

```markdown
(c) 2025 Vantle
```

## Golden Ratio Reference

The golden ratio (phi = 1.618) governs proportions:

| Application | Ratio | Value |
|-------------|-------|-------|
| Image width | 1/phi | 61.8% |
| Margin | phi^-3 | 23.6% |
| Padding | phi^-2 | 38.2% |

## Style Checklist

- [ ] Centered HTML title with correct style attribute
- [ ] Centered tagline with `<em>`
- [ ] Navigation bar with bold links
- [ ] Horizontal rules between sections
- [ ] Code blocks with language hints
- [ ] Tables for structured data
- [ ] Golden ratio image widths
- [ ] Relative internal links
- [ ] Copyright footer
