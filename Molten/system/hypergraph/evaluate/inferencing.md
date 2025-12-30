# Inferencing

This document describes the hypergraph inference system, which propagates wave patterns through particle relationships to create new edges representing derived knowledge.

## Architecture Overview

The inference system consists of four layered functions:

```text
fixed (optional wrapper, iterates until no new edges)
  └── infer (single pass + polymorphism)
        ├── Enumerates bipartite matchings
        ├── Computes remainders
        ├── Calls absorb for each matching
        └── For each edge: applies rule polymorphically to ancestors
              ├── Independence check (particles must be in different united sets)
              └── absorb (single rule application)
                    ├── World-unique matching enumeration
                    └── translate (edge creation primitive)
                          ├── Returns Translation::Existing or Translation::New
                          ├── Deduplication (returns Existing for duplicates)
                          ├── Future/past tracking updates
                          └── Lineage unification (unite)
```

| Function | Responsibility |
|----------|----------------|
| `translate` | Creates edges, returns `Translation` enum (New/Existing) |
| `absorb` | Applies a single rule to source nodes, only returns newly created edges |
| `infer` | Single pass: matches rules + applies polymorphically to ancestors |
| `fixed` | Iterates `infer` until fixed point (no new edges) |

---

## United vs World Model

Two distinct tracking mechanisms manage computational lineage:

### United

Tracks computational lineage as an equivalence relation. Particles in the same `united` set share a parent-child relationship in the computation graph.

**Constraint**: Particles cannot interact via rules with their ancestors or descendants. This preserves locality and causality.

### World

Assigns a unique identity to each particle. Worlds can differ even for united particles after joins or splits.

**Constraint**: When matching multiple particles to a rule, each must come from a different world.

### Lineage Rules

| Operation | Effect |
|-----------|--------|
| `[A, B] → C` | C united with A, C united with B. A and B remain independent. |
| `[X] → [Y, Z]` | Y united with X, Z united with X. Y and Z are NOT united (can interact). |
| Initial `focus` | New particle gets new world, own united set (initially independent). |

**Diagram: Lineage After Split**

```text
[X] → [Y, Z]

        X (world₀)
       / \
      Y   Z
   (w₁)   (w₂)

Y ↔ Z: ✓ Can interact (siblings, not united)
X ↔ Y: ✗ Cannot interact (parent-child, united)
X ↔ Z: ✗ Cannot interact (parent-child, united)
```

> 💡 **Physical Intuition**: You cannot use two potential outcomes from the same parallel computation thread simultaneously. This would violate locality/causality by collapsing distinct branches of computation into one.

---

## `fn infer(&mut self, refractions: Related<Wave<T>>) -> Result<Inference>`

### Overview

The `infer` function performs a single pass of hypergraph inference:

1. For each rule, enumerate bipartite matchings against independent graph particles
2. Compute remainders and create edges via `absorb`
3. For each newly created edge, backtrack through `past` edges to find ancestors
4. Apply the same rule polymorphically to ancestors, carrying remainders through

### Phase 1: Bipartite Matching with Superset Constraint

For each wave pattern in `refractions`, enumerate all bipartite matchings where graph particles are supersets of their matched wave particles.

**Method Signatures:**

```rust
fn independent(&self, rank: usize) -> impl Iterator<Item = BTreeSet<Label>>
fn bipartite(&self, combination: BTreeSet<Label>, rule: &Wave) -> Result<impl Iterator<Item = Wave>>
```

#### Superset Semantics with Multiplicity

Particles are multisets. The superset relation respects element counts:

| Comparison | Result |
|------------|--------|
| `{Apple×2} ⊇ {Apple×1}` | ✓ True |
| `{Apple×2, Banana×1} ⊇ {Apple×1, Banana×1}` | ✓ True |
| `{Apple×2} ⊇ {Apple×1, Banana×1}` | ✗ False (missing Banana) |
| `{Apple×3} ⊇ {Apple×1, Banana×1}` | ✗ False (missing Banana) |

#### Independent Particle Enumeration

Before matching, enumerate all groups of mutually independent particles whose size matches the wave rank.

**Independence Definition**: Two particles are independent if they belong to different equivalence classes in `united`. A group is mutually independent if all pairs are independent.

**Algorithm:**

```text
fn independent(&self, rank: usize) -> impl Iterator<Item = BTreeSet<Label>>:
  classes = collect equivalence classes from united map

  if rank > classes.len():
    return empty iterator

  for each combination of rank classes:
    for each selection in cartesian_product(class.members for class in combination):
      yield selection as BTreeSet<Label>
```

**Example:**

```text
united = {
  L₀ → {L₀, L₁},    # Class 1
  L₂ → {L₂},        # Class 2
  L₃ → {L₃, L₄}     # Class 3
}

independent(rank=2):
  {Class1, Class2} → (L₀,L₂), (L₁,L₂)
  {Class1, Class3} → (L₀,L₃), (L₀,L₄), (L₁,L₃), (L₁,L₄)
  {Class2, Class3} → (L₂,L₃), (L₂,L₄)
```

#### Bipartite Matching

For each independent combination, find all ways to match graph particles to wave particles where each graph particle is a superset.

**Key Properties:**

▸ Order does not matter (combinations, not permutations)
▸ Empty wave trivially matches with no particles
▸ All valid matchings proceed to edge creation

**Diagram:**

```text
Wave                           Graph Particles
┌─────────────┐               ┌─────────────────┐
│  p₁ = {a}   │───────────────│  q₁ = {a,x,y}   │  q₁ ⊇ p₁ ✓
└─────────────┘               └─────────────────┘
┌─────────────┐               ┌─────────────────┐
│  p₂ = {b,c} │───────────────│  q₂ = {b,c,z}   │  q₂ ⊇ p₂ ✓
└─────────────┘               └─────────────────┘

Full Matching: {(p₁,q₁), (p₂,q₂)}
```

---

### Phase 2: Remainder Computation and Edge Creation

For each bipartite matching, compute remainders and create edges.

#### Remainder Computation

The remainder is the set difference (with multiplicity) between the graph particle and the matched wave particle:

```text
remainder = particle \ pattern
```

**Example:**

```text
Particle:  {Apple×3, Banana×1}
Pattern:   {Apple×1}
Remainder: {Apple×2, Banana×1}
```

#### Broadcasting

All remainders from all matched pairs are concatenated and broadcast to ALL destination particles:

```text
Rule: [A, B, C] → [D, E]
Matching: (A→A.X, B→B.Y, C→C.Z)

Remainders: [X, Y, Z]
All remainders concatenated: X.Y.Z
Result: [D.X.Y.Z, E.X.Y.Z]
```

> 💡 **Rationale**: Broadcasting preserves complete polymorphic context. Both destinations carry full provenance, enabling pattern matching in subsequent inference steps.

**Operators:**

| Operator | Meaning |
|----------|---------|
| `.` | Particle continuation (multiset union within single state) |
| `,` | New orthogonal particle (separate world from computation result) |

#### World Mapping and Deduplication

For each computed result particle:

1. **Search** for existing isomorphic particles in the graph
2. **Enumerate** all valid world combinations (destinations must be in different worlds)
3. **Create** new particles only for unmatched components
4. **Create** one edge per valid world mapping

**Example: Multiple World Mappings**

```text
Result particles: [C, D]

Existing graph:
  ▸ L₁ = C in world₀
  ▸ L₂ = D in world₁
  ▸ L₃ = C in world₂
  ▸ L₄ = D in world₃

Valid mappings (destinations independent):
  ✓ {L₁, L₂} → Edge 1
  ✓ {L₃, L₄} → Edge 2
  ✓ {L₁, L₄} → Edge 3
  ✓ {L₃, L₂} → Edge 4

Creates 4 edges, one per valid combination.
```

**Example: Partial Match**

```text
Result particles: [C, D]

Existing:
  ▸ L₁ = D in world₀ (no C exists)

Action:
  1. Create L₂ = C in new world₁
  2. Create edge pointing to {L₂, L₁}
```

---

### Phase 3: Polymorphic Application via Backtracking

After creating edges, backtrack through the graph's history to apply the same rule polymorphically to ancestors.

#### Purpose

The type system and computation are unified. Abstract rules (e.g., `[And.Boolean.Boolean] → Boolean`) must apply to concrete states (e.g., `And.True.True`) that were deduced to match the abstract pattern.

#### Example: Polymorphism

```text
Initial graph: And.True.True

Step 1: Type deduction (via rules like [True] → Boolean)
  And.True.True → And.Boolean.Boolean (edge created)

Step 2: Abstract rule matches
  Rule: [And.Boolean.Boolean] → Boolean
  Matches the deduced And.Boolean.Boolean

Step 3: Backtrack via past edges
  Find: And.True.True is an ancestor of And.Boolean.Boolean

Step 4: Polymorphic application
  Apply rule to And.True.True:
    Matching: And.True.True against And.Boolean.Boolean
    Check: And.True.True ⊇ And.Boolean.Boolean (True ⊇ Boolean)
    Remainder: True.True
    Result: Boolean.True.True → True.True (And consumed)

Creates edge: And.True.True → True.True
```

#### Backtracking Algorithm

Traverse `past` edges (NOT `united` sets) to find computational ancestors. When edges have multiple sources (joins), ancestry is tracked **per position** and expanded combinatorially.

**Single-particle ancestry:**

```text
fn ancestors(label: Label, past: &Map<Label, Set<Label>>) -> Vec<Label>:
  result = []
  visited = {label}
  queue = [label]

  while queue not empty:
    current = queue.pop()
    for incoming in past[current]:
      edge = graph.edge(incoming)
      for source in edge.inference.source:
        if source not in visited:
          visited.insert(source)
          result.push(source)
          queue.push(source)

  return result
```

**Join-aware combinatorial expansion:**

When backtracking through a join edge `[D, B] → C`, each source position has its own ancestor chain. Polymorphic application requires enumerating all combinations:

```text
fn combinations(edge: Edge, past: &Map<Label, Set<Label>>) -> Vec<Vec<Label>>:
  # Collect ancestor chain per source position (includes source itself)
  chains = []
  for source in edge.inference.source:
    chain = [source] + ancestors(source, past)
    chains.push(chain)

  # Cartesian product across all positions
  return product(chains)
```

**Example: Join with ancestry**

```text
Given:
  A → K → D       (chain)
  [D, B] → C      (join)

Ancestry per position:
  Position 0 (D): [D, K, A]
  Position 1 (B): [B]

Combinatorial expansion:
  ✓ (D, B) — direct sources
  ✓ (K, B) — K ancestor of D
  ✓ (A, B) — A ancestor of K

Each combination is a candidate for polymorphic rule application.
```

**Diagram: Join ancestry expansion**

```text
      A                         Combinations to try:
      │
      ▼                           (A, B)
      K              B              │
      │              │              ▼
      ▼              │            (K, B)
      D ─────────────┘              │
      │       join                  ▼
      ▼                           (D, B)
      C ◄── rule matched here
```

#### Applying Rule to Ancestors

For each ancestral combination:

1. **Independence check**: Verify no two particles in the combination share a `united` set (locality constraint). Skip combinations where particles are ancestors/descendants of each other.
2. Check if each particle is a superset of the corresponding matched pattern
3. If all positions match, compute remainders and apply the rule
4. Call `absorb` to create the edge (only newly created edges are returned)

```text
# Independence check pseudocode
for (i, label) in combination:
  class = united.find(|members| members.contains(label))
  for other in combination[i+1..]:
    if class.contains(other):
      skip_combination()  # Particles share lineage
```

> **Important**: This check prevents applying rules to combinations where one particle is an ancestor of another. After edge creation, particles become united, so backtracking must verify that ancestral combinations remain independent.

**Diagram:**

```text
      And.True.True (ancestor)
            │
            │  past edge (type deduction)
            ▼
      And.Boolean.Boolean
            │
            │  rule matches here
            ▼
         Boolean

───────────────────────────────────────

Polymorphic application:

      And.True.True ─────────────────► True.True
                         (new edge)
```

---

## `fn fixed(&mut self, refractions: Related<Wave<T>>) -> Result<Inference>`

### Overview

The `fixed` function wraps `infer` and iterates until no new edges are created (fixed point).

```rust
fn fixed(&mut self, refractions: Related<Wave<T>>) -> Result<Inference> {
    let mut all = Inference::new();

    loop {
        let inference = self.infer(refractions.clone())?;

        if inference.edges.is_empty() {
            break;
        }

        all.edges.extend(inference.edges);
    }

    Ok(all)
}
```

### Termination

> ⚠️ **Warning**: `fixed` may not terminate for cyclic rule patterns. Use with caution.

**Example of Non-Termination:**

```text
[A] → B
[B] → A

This creates an infinite loop: A → B → A → B → ...
```

> 💡 **Recommendation**: Use `infer` (single pass) by default. Use `fixed` only when you need complete deductive closure and have verified termination.

---

## `fn absorb(&mut self, source: BTreeSet<Label>, rule: Relation<Wave>) -> Result<impl Iterator<Item = Label>>`

### Overview

The `absorb` function applies a single rule to source nodes, creating edges with world-unique matching.

### Algorithm

```text
fn absorb(source: BTreeSet<Label>, rule: Relation<Wave>) -> Result<Iterator<Label>>:
  # Step 1: Compute result particles
  residual = compute_remainders(source, rule.source)
  destinations = rule.sink.join(residual)  # broadcast remainders

  # Step 2: Enumerate world-unique matchings
  matchings = enumerate_matchings(destinations, graph)

  # Step 3: Handle no-match case
  if matchings.empty():
    # Create new particles for each destination
    labels = destinations.map(|p| self.focus(p))
    matchings = [labels]

  # Step 4: Create edges via translate
  edges = []
  for matching in matchings:
    edge = self.translate(source, matching, rule)
    edges.push(edge)

  return edges.into_iter()
```

### World-Unique Matching

Each destination particle must map to a node in a different world:

```text
Destinations: [A, A]  (two copies of A)

Available:
  ▸ L₁ = A in world₀
  ▸ L₂ = A in world₁
  ▸ L₃ = A in world₀

Valid matchings:
  ✓ {L₁, L₂} (world₀, world₁)
  ✓ {L₂, L₃} (world₁, world₀)

Invalid:
  ✗ {L₁, L₃} (both world₀ — violates uniqueness)
```

---

## `fn translate(&mut self, source: BTreeSet<Label>, destinations: BTreeSet<Label>, rule: Relation<Wave>) -> Result<Translation>`

### Overview

The `translate` function is the primitive for edge creation. It returns a `Translation` enum indicating whether a new edge was created or an existing one was found:

```rust
enum Translation {
    Existing(Label),  // Edge already exists with this source, sink, and rule
    New(Label),       // New edge was created
}
```

This distinction is critical for fixed-point termination: `absorb` only returns newly created edges, so when all edges already exist, `infer` returns empty and `fixed` terminates.

### Algorithm

```text
fn translate(source, destinations, rule) -> Result<Translation>:
  # Deduplication check
  existing = edges.find(|edge|
    edge.source == source &&
    edge.sink == destinations &&
    edge.relation == rule
  )

  if existing:
    return Translation::Existing(existing.label)

  # Create new edge
  label = new_label()
  edge = Edge { label, inference: { source, sink: destinations }, relation: rule }
  edges.insert(edge)

  # Update tracking
  for origin in source:
    future[origin].insert(label)
  for destination in destinations:
    past[destination].insert(label)

  # Unite lineages
  for origin in source:
    for destination in destinations:
      unite(origin, destination)

  return Translation::New(label)
```

---

## Summary

| Function | Input | Output | Key Responsibility |
|----------|-------|--------|-------------------|
| `translate` | source labels, dest labels, rule | `Translation` enum | Edge creation with deduplication |
| `absorb` | source labels, rule | new edge labels only | Single rule application with world-unique matching |
| `infer` | rule set | inference result | Single pass + polymorphic backtracking |
| `fixed` | rule set | inference result | Iterate until no new edges |

**Key Invariants:**

◆ Destinations must be in different worlds (locality)
◆ Particles cannot interact with ancestors/descendants (causality via united sets)
◆ Ancestral combinations must be independent (no shared united sets)
◆ Remainders are broadcast to all destinations (polymorphic context preservation)
◆ Duplicate edges return `Translation::Existing` (fixed-point termination)
