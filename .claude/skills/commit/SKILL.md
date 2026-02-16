---
name: commit
description: Create a commit following Vantle's release message standards. Analyzes staged changes and generates a structured commit message with component sections linking to Readme.md files. Use when user says 'commit', 'release', 'save changes', or is ready to commit staged work.
allowed-tools: Read, Glob, Grep, Bash(git status:*), Bash(git diff:*), Bash(git log:*), Bash(git add:*), Bash(git commit:*), Bash(git stash:*)
---

# Vantle Commit Standards

## Activation

- [ ] EVALUATE: Does this request involve committing, releasing, or saving staged changes?
- [ ] DECIDE: "This task requires /commit because..."
- [ ] EXECUTE: Follow the workflow below

Create commits following Vantle's structured release message format.

## Message Format

```
[Vantle] [Release] [Readme.md]: Brief title describing all changes

[Component] [path/to/Readme.md]: Component description:
- Feature or change 1
- Feature or change 2

[Component2] [path/to/Readme.md]: Another component:
- Change 1
- Change 2
```

## Structure Rules

### Header Line
- Format: `[Vantle] [Release] [Readme.md]: Title`
- Title should summarize ALL major changes in the commit
- Keep title concise but descriptive

### Component Sections
- Each major component gets its own section
- Format: `[Name] [path/to/Readme.md]: Description:`
- Link to the component's Readme.md file
- Use bullet points for specific changes
- Order sections logically: Build, Generation, then feature-specific components

### Standard Component Order
1. `[Build] [MODULE.bazel]` - Dependency and toolchain changes
2. `[Generation] [system/generation/Readme.md]` - Code generation framework
3. `[Observation] [system/observation/Readme.md]` - Telemetry and tracing
4. `[Molten] [Molten/Readme.md]` - Molten language and runtime

## Commit Process

### Step 1: Analyze Changes

```bash
git status --porcelain
git diff --cached --stat
```

### Step 2: Identify Components

For each changed area, determine:
1. Which component it belongs to
2. The path to that component's Readme.md
3. A concise description of changes

### Step 3: Generate Message

Build the commit message following the format above.

### Step 4: Create Commit

```bash
git commit -m "$(cat <<'EOF'
[Vantle] [Release] [Readme.md]: Title here

[Component] [path/Readme.md]: Description:
- Change 1
- Change 2
EOF
)"
```

## Example Commit

```
[Vantle] [Release] [Readme.md]: Observation telemetry with generation refinements

[Build] [MODULE.bazel]: Async and telemetry dependencies:
- tracing, tracing-subscriber, tracing-chrome
- prost, tonic, tokio (gRPC transport)
- dashmap, url

[Generation] [system/generation/Readme.md]: Code generation framework refinements:
- Streamlined module organization
- Improved error diagnostics with miette
- Format string interpolation cleanup

[Observation] [system/observation/Readme.md]: Trace streaming and recording framework:
- Portal server with gRPC transport
- #[trace] macro for function instrumentation
- Channel-based span filtering
- File and network trace sinks

[Molten] [Molten/Readme.md]: Forge observation integration:
- --address flag for trace streaming
- File and gRPC sink support
```

## Guidelines

1. **Never co-author** - Do not add co-author lines
2. **Link to Readme** - Every component section must reference its Readme.md
3. **Be specific** - Bullet points should describe actual changes, not vague statements
4. **Group logically** - Related changes go in the same component section
5. **Order consistently** - Follow the standard component order
