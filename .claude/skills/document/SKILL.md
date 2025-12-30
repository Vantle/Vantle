---
name: document
description: Review documentation for accuracy, beauty, helpfulness, and efficiency. Analyzes markdown files, validates examples, and suggests improvements. Use when user mentions 'docs', 'documentation', 'readme', 'review docs', or asks about doc quality.
allowed-tools: Read, Glob, Grep, Bash(bazel build:*), Bash(bazel test:*), Bash(ls:*)
---

# Documentation Review

## Activation

- [ ] EVALUATE: Does this request involve documentation, readme files, or doc quality?
- [ ] DECIDE: "This task requires /document because..."
- [ ] EXECUTE: Follow the workflow below

Expert documentation reviewer. Evaluate markdown files for accuracy, beauty, helpfulness, and efficiency.

## Process

### 1. Discover

Use Glob `**/*.md` to find all markdown files. Note which components have Readme.md.

### 2. Analyze

Read `style.md` and `structure.md` for detailed requirements. For each file, evaluate:

**Accuracy**: Code examples match implementation, paths exist, versions current
**Beauty**: Centered headers, nav bars, golden ratio images (61.8%), tables
**Helpfulness**: Clear explanations, practical examples, complete coverage, links
**Efficiency**: Concise writing, scannable structure, no redundancy

### 3. Validate

Use `ls` to verify paths exist. Check bazel targets. Ensure code examples compile.

### 4. Report

```markdown
## Documentation Review

| File | Accuracy | Beauty | Helpful | Efficient |
|------|----------|--------|---------|-----------|
| path/Readme.md | PASS | WARN | PASS | PASS |

**Overall**: PASS/WARN/FAIL

## Findings

### [file.md]

**[Criterion]** [line]: Issue
**Fix**: Suggestion

## Missing Documentation

- [component/path] - Reason docs would help

Create documentation for any of these?
```

## Non-Negotiable Violations

1. Code examples that don't compile or match current API
2. Broken links or nonexistent file paths
3. Inconsistent header styling (must use centered HTML)
4. Missing navigation links
5. Outdated versions
6. Walls of text without structure
