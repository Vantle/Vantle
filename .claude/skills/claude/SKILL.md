---
name: claude
description: Optimize Claude Code configuration. Audits settings.local.json permissions, CLAUDE.md rules, skill definitions, and subagent instructions for redundancy, staleness, and completeness. Use when user says 'optimize claude', 'clean up config', 'audit settings', 'improve claude setup', or 'claude skill'.
allowed-tools: Read, Glob, Grep, Edit, Write
disable-model-invocation: true
---

# Claude Code Configuration Optimizer

## Activation

- [ ] EVALUATE: Does this request involve optimizing Claude Code config, permissions, CLAUDE.md, or skills?
- [ ] DECIDE: "This task requires /claude because..."
- [ ] EXECUTE: Follow the workflow below

Audit and optimize all Claude Code configuration files for this project.

## Targets

| File | What to Audit |
|------|---------------|
| `.claude/settings.local.json` | Permission patterns |
| `.claude/settings.json` | Hooks configuration |
| `CLAUDE.md` | Project rules and conventions |
| `.claude/skills/*/SKILL.md` | Skill definitions |
| `.claude/agents/*.md` | Subagent instructions |

## Process

### Step 1: Read All Configuration

Read every file in `.claude/` and `CLAUDE.md`:

```
Glob: .claude/**/*
Read: CLAUDE.md
```

### Step 2: Audit Permissions (settings.local.json)

Analyze the `permissions.allow`, `permissions.deny`, and `permissions.ask` arrays:

**Redundancy**: Find patterns that are subsumed by broader patterns.
Example: `Bash(git -C /Users/.../Vantle log:*)` is redundant when `Bash(git -C *:*)` already exists.

**Consolidation**: Identify groups of specific entries that could be one wildcard.
Example: Multiple `Bash(git log:*)`, `Bash(git diff:*)`, `Bash(git show:*)` could potentially be consolidated if all git read commands should be allowed.

**Missing**: Identify tools or commands that are frequently prompted (the user keeps having to approve). Suggest adding them to `allow`.

**Misplaced**: Identify entries in `allow` that should be in `ask` (destructive commands) or entries in `ask` that could safely be in `allow`.

### Step 3: Audit CLAUDE.md

Read `CLAUDE.md` and check for:

**Staleness**: Rules that reference patterns, files, or conventions no longer present in the codebase.

**Contradictions**: Rules that conflict with each other or with actual codebase patterns.

**Vagueness**: Rules that are ambiguous and could be interpreted multiple ways. Suggest precise rewording.

**Gaps**: Patterns observed in the codebase that are not captured by any rule. Propose specific additions.

**Verbosity**: Sections that could be more concise without losing meaning.

Cross-reference against actual codebase:
- Do naming rules match actual identifiers?
- Do module rules match actual module structure?
- Do code quality rules match actual error handling patterns?

### Step 4: Audit Skills

For each skill in `.claude/skills/*/SKILL.md`:

**allowed-tools**: Are all necessary tools listed? Are there tools listed that the skill never uses?

**Description**: Will the description trigger correctly? Does it cover all the phrases users would say?

**Workflow**: Are the steps complete and in the right order? Are there steps that reference nonexistent files, commands, or patterns?

**Activation**: Does the activation checklist match the description?

**Supporting files**: Does the skill have supporting files (like `security.md`, `performance.md`) that are stale or incomplete?

### Step 5: Audit Subagents

For each agent in `.claude/agents/*.md`:

**Scope**: Is the scope well-defined? Does it overlap with any skill?

**allowed-tools**: Are they sufficient for the agent's task?

**Instructions**: Are the violation categories comprehensive? Are there false positive patterns?

### Step 6: Audit Hooks

For each hook in `.claude/settings.json`:

**Correctness**: Does the command parse stdin correctly (jq for tool_input)?

**Coverage**: Are there missing hooks that would be valuable?

**Performance**: Are hooks fast enough not to slow down editing?

## Output Format

```markdown
## Claude Configuration Audit

### Permissions (settings.local.json)

| Issue | Type | Current | Suggestion |
|-------|------|---------|------------|
| Redundant git path entries | Redundancy | 3 specific entries | Remove, covered by `Bash(git -C *:*)` |

### CLAUDE.md

| Section | Issue | Suggestion |
|---------|-------|------------|
| Naming | Missing exception for... | Add exception clause |

### Skills

| Skill | Issue | Suggestion |
|-------|-------|------------|
| /commit | Missing `Bash(git stash:*)` in allowed-tools | Add it |

### Subagents

| Agent | Issue | Suggestion |
|-------|-------|------------|
| convention | Missing check for... | Add pattern |

### Hooks

| Hook | Issue | Suggestion |
|------|-------|------------|
| format | Could also format .bazel files | Add buildifier |

**Apply N optimizations?**
```

### Step 7: Apply (on user approval)

Edit each file with the approved optimizations. Make targeted edits, not wholesale rewrites.

## Arguments

The user may specify scope:

```
/claude                  # audit everything
/claude permissions      # audit only settings.local.json
/claude rules            # audit only CLAUDE.md
/claude skills           # audit only skills
/claude agents           # audit only subagents
/claude hooks            # audit only hooks
```
