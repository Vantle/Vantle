# `fn infer(&mut self, refractions: Related<Wave<T>>, strategy: Strategy) -> Result<Inference, Error>`

## Overview

The `infer` function performs hypergraph inference by propagating wave patterns through particle relationships, creating new edges that represent derived knowledge. The algorithm enumerates bipartite matchings between waves and independent particles, then backtracks through the graph to discover transitive relationships.

## Algorithm Design

### Phase 1: Full Bipartite Matching with Superset Constraint

For each wave `w` in `refractions`, enumerate all **full** bipartite matchings where graph particles must be supersets of their matched wave particles.

```
Input: Wave w = {p₁, p₂, ..., pₙ}
Output: Set of full matchings M = {m₁, m₂, ..., mₖ}

For each combination of n independent graph particles {q₁, q₂, ..., qₙ}:
  For each permutation π of {1, 2, ..., n}:
    If ∀i: qπ(i) ⊇ pᵢ:  # Each graph particle is a superset
      Create full matching m: {(p₁,qπ(1)), (p₂,qπ(2)), ..., (pₙ,qπ(n))}
      Add m to M
```

**Key Constraint**: For matching (pᵢ, qⱼ), we require qⱼ ⊇ pᵢ (graph particle contains all elements of wave particle).

---

#### Phase 1.1: Independent Particle Group Enumeration

Before attempting bipartite matching, we must enumerate all possible groups of independent particles whose size matches the rank of the wave.

**Method Signature:**
```rust
fn independent(&self, rank: usize) -> impl Iterator<Item = Vec<Label>>
```

**Purpose**: Generate all k-tuples of mutually independent particle labels, where k = rank (the size of the wave being matched).

**Independence Definition**: Two particles are independent if they belong to different equivalence classes in the `united` map. A group of k particles is mutually independent if all pairs are independent.

**Algorithm:**
```
fn independent(&self, rank: usize) -> impl Iterator<Item = Vec<Label>>:
  # Get all equivalence class representatives
  equivalence_classes = []
  For each (representative, members) in self.united:
    equivalence_classes.push((representative, members))

  # Generate all combinations of k equivalence classes
  For each combination C of rank classes from equivalence_classes:
    # For each selected class, we need to pick exactly one member
    # This gives us the cartesian product across selected classes

    # Generate cartesian product of members from selected classes
    For each selection S in cartesian_product(C.members):
      # S is a k-tuple of labels, one from each selected class
      # All labels in S are mutually independent
      yield S
```

**Example 1: Simple Independence**
```
Hypergraph state:
  united = {
    L₀ → {L₀, L₁},    # Equivalence class 1: L₀ and L₁ are united
    L₂ → {L₂},        # Equivalence class 2: L₂ alone
    L₃ → {L₃, L₄, L₅} # Equivalence class 3: L₃, L₄, L₅ are united
  }

Call: independent(rank=2)

Step 1: Select 2 equivalence classes from 3 available
  Combinations: {class1, class2}, {class1, class3}, {class2, class3}

Step 2: For each combination, generate cartesian product

  Combination {class1, class2}:
    class1 members: {L₀, L₁}
    class2 members: {L₂}
    Cartesian product:
      → (L₀, L₂)
      → (L₁, L₂)

  Combination {class1, class3}:
    class1 members: {L₀, L₁}
    class3 members: {L₃, L₄, L₅}
    Cartesian product:
      → (L₀, L₃)
      → (L₀, L₄)
      → (L₀, L₅)
      → (L₁, L₃)
      → (L₁, L₄)
      → (L₁, L₅)

  Combination {class2, class3}:
    class2 members: {L₂}
    class3 members: {L₃, L₄, L₅}
    Cartesian product:
      → (L₂, L₃)
      → (L₂, L₄)
      → (L₂, L₅)

Total yield: 11 tuples of mutually independent particles
```

**Example 2: Rank 3 Enumeration**
```
Hypergraph state:
  united = {
    L₀ → {L₀, L₁},
    L₂ → {L₂},
    L₃ → {L₃},
    L₄ → {L₄}
  }

Call: independent(rank=3)

Step 1: Select 3 equivalence classes from 4 available
  Combinations:
    {class1, class2, class3}
    {class1, class2, class4}
    {class1, class3, class4}
    {class2, class3, class4}

Step 2: Generate cartesian products

  {class1, class2, class3} → {L₀,L₁} × {L₂} × {L₃}:
    → (L₀, L₂, L₃)
    → (L₁, L₂, L₃)

  {class1, class2, class4} → {L₀,L₁} × {L₂} × {L₄}:
    → (L₀, L₂, L₄)
    → (L₁, L₂, L₄)

  {class1, class3, class4} → {L₀,L₁} × {L₃} × {L₄}:
    → (L₀, L₃, L₄)
    → (L₁, L₃, L₄)

  {class2, class3, class4} → {L₂} × {L₃} × {L₄}:
    → (L₂, L₃, L₄)

Total yield: 7 tuples of mutually independent particles
```

**Diagram: Independence Structure**
```
Equivalence Classes (united map):

   ┌─────────────┐
   │  Class 1    │
   │  ┌──┐ ┌──┐  │
   │  │L₀│ │L₁│  │  (united together)
   │  └──┘ └──┘  │
   └─────────────┘

   ┌─────────────┐
   │  Class 2    │
   │  ┌──┐       │
   │  │L₂│       │  (independent)
   │  └──┘       │
   └─────────────┘

   ┌─────────────┐
   │  Class 3    │
   │  ┌──┐ ┌──┐ ┌──┐│
   │  │L₃│ │L₄│ │L₅││  (united together)
   │  └──┘ └──┘ └──┘│
   └─────────────┘

Independent selections (rank=2):
  Pick 2 classes, then 1 member from each:

  ┌──┐          ┌──┐
  │L₀│─────────│L₂│    ✓ (different classes)
  └──┘          └──┘

  ┌──┐          ┌──┐
  │L₀│─────────│L₁│    ✗ (same class - not independent)
  └──┘          └──┘

  ┌──┐          ┌──┐
  │L₀│─────────│L₃│    ✓ (different classes)
  └──┘          └──┘
```

**Implementation Considerations:**

1. **Equivalence Class Extraction**: Build a map from representative labels to all members of that class.

2. **Combination Generation**: Use `itertools::combinations` or similar to select k classes from available classes.

3. **Cartesian Product**: Use nested iteration or `itertools::multi_cartesian_product` to generate all member selections.

4. **Iterator Efficiency**: Return an iterator to avoid materializing all tuples in memory. Use lazy evaluation.

5. **Empty Handling**: If `rank > number_of_equivalence_classes`, the iterator should be empty (impossible to find k independent particles).

**Usage in Matching:**
```rust
let wave = /* Wave with n particles */;
let rank = wave.particles().count();

for independent_group in graph.independent(rank) {
  // independent_group: Vec<Label> of size rank
  // All labels in independent_group are mutually independent
  // Now attempt bipartite matching between wave and independent_group
  if let Some(matching) = try_bipartite_match(&wave, &independent_group) {
    // Process valid matching
  }
}
```

**Diagram: Full Bipartite Matching**
```
Wave w                      Graph Particles
┌─────────┐                ┌─────────────┐
│ p₁={a}  │────────────────│ q₁={a,x,y}  │  q₁ ⊇ p₁ ✓
└─────────┘                └─────────────┘
┌─────────┐                ┌─────────────┐
│ p₂={b,c}│────────────────│ q₂={b,c,z}  │  q₂ ⊇ p₂ ✓
└─────────┘                └─────────────┘
┌─────────┐                ┌─────────────┐
│ p₃={d}  │────────────────│ q₃={d,w}    │  q₃ ⊇ p₃ ✓
└─────────┘                └─────────────┘

Full Matching: {(p₁,q₁), (p₂,q₂), (p₃,q₃)}
ALL wave particles matched, each to a superset in the graph.
```

**Example of Invalid Matching:**
```
Wave w                      Graph Particles
┌─────────┐                ┌─────────────┐
│ p₁={a,b}│───────✗────────│ q₁={a}      │  q₁ ⊉ p₁ ✗ (missing b)
└─────────┘                └─────────────┘

This is NOT a valid matching because q₁ is not a superset of p₁.
```

### Phase 2: Edge Creation with Remainder Computation and World State Management

For each bipartite matching, compute the remainder from the matching, combine it with the destination/refraction to form the result state, then create edges with world state management.

**Key Principle**: The result is **remainder + destination**, where remainder = superset elements not in the matched wave particle.

```
For each matching m in M:
  # Step 1: Compute remainders from the superset matching
  remainders = []
  For each (wave_particle, graph_particle) in m:
    remainder = graph_particle \ wave_particle  # Set difference
    remainders.append(remainder)

  # Step 2: Combine remainders with destination/refraction
  # If wave pattern is [A,A] → B and we match A.C, A.D:
  #   remainders = [C, D]
  #   result = [B.C, B.D] (destination B combined with each remainder)
  #
  # If wave pattern is [A,A] → [B,C] and we match A.D, A.E.F:
  #   remainders = [D, E.F]
  #   result = [D.B, C.E.F] (pairwise combination)

  result_states = combine(destination_wave, remainders)

  # Step 3: For each result state, find or create particles in worlds
  refraction_labels = []
  For each result_state in result_states:
    # Search for isomorphic state match across all worlds
    isomorphic_matches = find_isomorphic_states(result_state, graph.worlds)

    If isomorphic_matches.is_empty():
      # No isomorphic match: create new particle in new world
      new_label = graph.focus(result_state)
      refraction_labels.append([new_label])
    Else:
      # Found isomorphic matches: use all valid combinations
      refraction_labels.append(isomorphic_matches)

  # Step 4: Create edges for all world state combinations
  For each combination in cartesian_product(refraction_labels):
    edge = Edge {
      label: new_label(),
      sources: source_nodes from matching,
      refractions: combination,
      source: original wave pattern,
      refraction: result_states
    }
    Add edge to graph
    Update future[source_nodes] to include edge.label
    Update past[combination] to include edge.label
```

**Example 1: Simple Remainder**
```
Wave Pattern: [A,A] → B

Matching: (A→A.C, A→A.D)
  Remainders: [C, D]
  Result: B.C, B.D
  Creates edge: {A.C, A.D} → {B.C, B.D}

Diagram:
┌─────┐  ┌─────┐
│ A.C │  │ A.D │  (source: matched wave particles)
└──┬──┘  └──┬──┘
   └───┬────┘
       │
    ┌──▼───┐
    │ edge │
    └──┬───┘
       │
   ┌───┴────┐
   │        │
┌──▼───┐ ┌─▼────┐
│ B.C  │ │ B.D  │  (refraction: destination + remainders)
└──────┘ └──────┘
```

**Example 2: Multiple Destinations**
```
Wave Pattern: [A,A] → [B,C]

Matching: (A→A.D, A→A.E.F)
  Remainders: [D, E.F]
  Result: D.B, C.E.F  (pairwise: remainder[0]+dest[0], dest[1]+remainder[1])
  Creates edge: {A.D, A.E.F} → {D.B, C.E.F}
```

**Example 3: Isomorphic World State Matching**
```
Initial state:
  world₁: {A, B}
  world₂: {A, B}

Result state to create: {A, B, C}

Since both world₁ and world₂ have A+B isomorphically:
  Create world₃ with particle C

  Create TWO edge relationships (all valid combinations):
    Edge 1: {world₁.A, world₂.B, world₃.C}
    Edge 2: {world₂.A, world₁.B, world₃.C}

Diagram:
         world₁           world₂           world₃
         ┌────┐          ┌────┐          ┌────┐
         │ A  │          │ A  │          │ C  │
         └─┬──┘          └─┬──┘          └─┬──┘
           │               │               │
         ┌─▼──┐          ┌─▼──┐           │
         │ B  │          │ B  │           │
         └─┬──┘          └─┬──┘           │
           │               │               │
     ┌─────┴───────┬───────┴───────┬───────┘
     │             │               │
  ┌──▼─────────────▼───────────────▼──┐
  │ Edge 1: {w₁.A, w₂.B, w₃.C}       │
  └───────────────────────────────────┘
  ┌───────────────────────────────────┐
  │ Edge 2: {w₂.A, w₁.B, w₃.C}       │
  └───────────────────────────────────┘

Both edges represent valid isomorphic state combinations.
```

**World State Management Rules:**

1. **Isomorphic Match Search**: For each result state, search all existing world states for particles that match the structure.

2. **Partial Match Handling**: If worlds have partial matches (e.g., world₁ has A+B, world₂ has A+B, but result needs A+B+C), create new particles for missing components (C in world₃).

3. **Combination Enumeration**: When multiple worlds can satisfy different parts of the result state isomorphically, enumerate ALL valid combinations as separate edges.

4. **World Creation**: Create new worlds only when no isomorphic match exists for a component of the result state.

### Phase 3: Backtracking Through History

For each newly created edge, backtrack along paths in the graph to propagate the inference through time.

```
For each edge e in new_edges:
  For each source_node n in e.sources:
    past_paths = backtrack(n, strategy.depth)

    For each path p in past_paths:
      Construct new edge from p.origin to e.refractions
```

**Backtracking Algorithm:**
```
fn backtrack(node: Label, depth: usize) -> Vec<Path>:
  If depth = 0:
    return [Path::single(node)]

  paths = [Path::single(node)]

  For each edge_label in past[node]:
    edge = graph.edge(edge_label)
    For each source in edge.sources:
      sub_paths = backtrack(source, depth - 1)
      For each sub_path in sub_paths:
        paths.push(sub_path.prepend(node))

  return paths
```

**Diagram: Backtracking**
```
Depth 0:          Depth 1:          Depth 2:
                                    ┌────┐
                                    │ n₀ │ (origin)
                                    └─┬──┘
                                      │
                  ┌────┐           ┌──▼─┐
                  │ n₁ │◄──────────│ e₁ │
                  └─┬──┘           └────┘
                    │
┌────┐           ┌──▼─┐
│ n₂ │◄──────────│ e₂ │
└─┬──┘           └────┘
  │
┌─▼──┐
│ e₃ │ (current edge)
└─┬──┘
  │
┌─▼──┐
│ n₃ │ (refraction)
└────┘

Paths discovered:
  [n₀, e₁, n₁, e₂, n₂, e₃, n₃]
  [n₁, e₂, n₂, e₃, n₃]
  [n₂, e₃, n₃]
```

### Phase 4: Rule Application with Subset Difference

For each backtracked path, apply the originating node's rule using the bipartite subset difference.

```
For each path p in backtracked_paths:
  origin_node = graph.node(p.origin)

  # Get the original wave-to-refraction relationship
  original_wave = extract_wave_from_edge(p.first_edge)
  original_refractions = extract_refractions_from_edge(p.first_edge)

  # Compute subset difference between node groups
  origin_group = united[p.origin]  # All nodes united with origin
  current_group = united[current_edge.sources]

  difference = symmetric_difference(origin_group, current_group)

  # Apply origin's rule to difference set
  new_wave = apply_rule(origin_node, difference)
  new_refractions = project_refractions(current_edge.refractions, difference)

  # Create inferred edge
  inferred_edge = Edge {
    label: new_label(),
    sources: origin_group,
    refractions: new_refractions,
    source: new_wave,
    refraction: compute_refraction(new_wave, new_refractions)
  }

  Add inferred_edge to graph
```

**Diagram: Rule Application with Subset Difference**
```
Original Rule (from origin):
┌──────────────┐
│ Wave: {a,b}  │
│    Rule R    │
│ Refr: {c,d}  │
└──────────────┘

Current Context:
┌──────────────┐         ┌──────────────┐
│ Origin group │         │Current group │
│  {n₁, n₂}    │         │  {n₃, n₄}    │
└──────────────┘         └──────────────┘
        ⊕                        ⊕
    Difference: {n₁, n₂, n₃, n₄}

Application:
┌──────────────────────────────┐
│ Apply R to difference        │
│                              │
│ Sources: {n₁, n₂}            │
│          ↓                   │
│       Rule R                 │
│          ↓                   │
│ New Refr: {n₅, n₆}           │
│   (projected through diff)   │
└──────────────────────────────┘
```

### Phase 5: Iterative Expansion

Continue the inference process until all combinations of independent nodes in the past from the origins are covered.

```
worklist = initial_edges
visited = ∅
inferred_edges = ∅

While worklist is not empty and |inferred_edges| < strategy.breadth:
  edge = worklist.pop()

  If edge in visited:
    continue
  visited.add(edge)

  # Perform phases 1-4 for this edge
  new_inferences = infer_from_edge(edge, strategy)

  inferred_edges.union(new_inferences)
  worklist.extend(new_inferences)

Return Inference { edges: inferred_edges }
```

**Diagram: Full Inference Propagation**
```
Initial State:                  After Iteration 1:              After Iteration 2:

┌────┐                          ┌────┐                          ┌────┐
│ w₁ │───►refractions          │ w₁ │                          │ w₁ │
└────┘                          └─┬──┘                          └─┬──┘
                                  │                               │
                               ┌──▼──┐                         ┌──▼──┐
                               │ e₁  │                         │ e₁  │
                               └──┬──┘                         └──┬──┘
                                  │                               │
                               ┌──▼──┐                         ┌──▼──┐
                               │ n₁  │◄──┐                     │ n₁  │◄──┐
                               └─────┘   │                     └──┬──┘   │
                                         │                        │      │
                               ┌─────┐   │                     ┌──▼──┐   │
                               │ n₂  │◄──┘                     │ e₂  │   │
                               └─────┘                         └──┬──┘   │
                                                                  │      │
                                                               ┌──▼──┐   │
                                                               │ n₂  │◄──┤
                                                               └──┬──┘   │
                                                                  │      │
                                                               ┌──▼──┐   │
                                                               │ e₃  │   │
                                                               └──┬──┘   │
                                                                  │      │
                                                               ┌──▼──┐   │
                                                               │ n₃  │◄──┘
                                                               └─────┘

Edges created: ∅                Edges: {e₁}                    Edges: {e₁, e₂, e₃}
Nodes: {n₁, n₂}                 Nodes: {n₁, n₂}                Nodes: {n₁, n₂, n₃}
```

## Strategy Parameters

### Breadth
Maximum number of inferred edges to generate. Controls horizontal expansion of inference space.

**Effect of breadth = 3:**
```
     ┌───┐
     │ w │
     └─┬─┘
   ┌───┼───┬───┐
   ▼   ▼   ▼   ▼
  e₁  e₂  e₃  e₄ (truncated)
       ↑
     breadth limit
```

### Depth
Maximum depth to backtrack through past edges. Controls temporal/historical exploration.

**Effect of depth = 2:**
```
Level 0:         e₀ (current)
                  ↑
Level 1:         e₁
                  ↑
Level 2:         e₂
                  ↑
Level 3:         e₃ (not explored)
                  ↑
               depth limit
```

## Complexity Analysis

**Time Complexity:**
- Full bipartite matching: O(C(m,n) × n!) where m = graph particles, n = wave size
  - C(m,n) combinations to select n particles from m available
  - n! permutations to try all possible pairings
  - For each pairing, O(n × particle_size) to verify superset constraints
- Backtracking: O(breadth × depth × edges_per_node)
- Overall: O(breadth × depth × C(m,n) × n! × n × particle_size)

**Space Complexity:**
- Matching storage: O(C(m,n) × n! × n)
- Path storage: O(breadth × depth)
- Edge storage: O(breadth)
- Overall: O(breadth × depth + C(m,n) × n! × n)

## Implementation Notes

1. **Independence Detection**: Use the `united` map to determine which particles are independent (belong to different equivalence classes).

2. **Full Matching Enumeration**:
   - Generate all C(m,n) combinations of n independent graph particles
   - For each combination, generate all n! permutations
   - Verify superset constraint: graph_particle ⊇ wave_particle for each pair
   - Only valid full matchings proceed to edge creation

3. **Superset Verification**: For particles represented as sets, use set containment check. The graph particle must contain all elements present in the corresponding wave particle.

4. **Edge Deduplication**: Before adding an edge, check if an equivalent edge already exists (same source and refraction waves).

5. **Cycle Detection**: Track visited edges during backtracking to prevent infinite loops.

6. **Breadth Limiting**: Use a priority queue or scoring function to select the most promising inferences when approaching the breadth limit.

## Example Execution

```rust
// Initial hypergraph with particle data
let mut graph = Hypergraph::new();
let n1 = graph.focus(Particle::new(vec![1, 2]));      // {1, 2}
let n2 = graph.focus(Particle::new(vec![3]));         // {3}
let n3 = graph.focus(Particle::new(vec![1, 2, 4]));   // {1, 2, 4} - superset of n1
let n4 = graph.focus(Particle::new(vec![3, 5]));      // {3, 5} - superset of n2

// Create a wave with pattern {1,2} → {3}
let wave_particle_1 = Particle::new(vec![1, 2]);
let wave_particle_2 = Particle::new(vec![3]);
let wave = Wave::from([wave_particle_1, wave_particle_2]);

let refraction = Wave::from([/* refraction pattern */]);
let mut relations = Related::none();
relations.relate(&wave, &refraction);

// Perform inference
let strategy = Strategy { breadth: 10, depth: 3 };
let inference = graph.infer(relations, strategy)?;

// The algorithm will:
// 1. Find full matchings where graph particles are supersets:
//    - wave_particle_1 {1,2} matches n1 {1,2} (n1 ⊇ {1,2}) ✓
//    - wave_particle_1 {1,2} matches n3 {1,2,4} (n3 ⊇ {1,2}) ✓
//    - wave_particle_2 {3} matches n2 {3} (n2 ⊇ {3}) ✓
//    - wave_particle_2 {3} matches n4 {3,5} (n4 ⊇ {3}) ✓
//
// 2. Generate valid full matchings:
//    - ({1,2}→n1, {3}→n2)
//    - ({1,2}→n1, {3}→n4)
//    - ({1,2}→n3, {3}→n2)
//    - ({1,2}→n3, {3}→n4)
//
// 3. Create edges for each matching, then backtrack and apply rules
```

## Visualization: Complete Inference Graph

```
Legend: ○ = Particle/Node, → = Edge, ◇ = Wave Pattern
        ⊇ = superset relationship

Time flows downward ↓

t₀:  ◇ Wave Pattern w={p₁{a}, p₂{b,c}}
                │
                │ Full bipartite matching (all particles matched)
                │
                ▼
     Find graph particles that are supersets:
     ○n₁{a,x} ⊇ p₁{a} ✓       ○n₃{b,c,d} ⊇ p₂{b,c} ✓
     ○n₂{a,y} ⊇ p₁{a} ✓       ○n₄{b,c,e} ⊇ p₂{b,c} ✓

                │
t₁:  Valid full matchings:
     ┌──────────┼──────────┬──────────┬──────────┐
     M₁:(n₁,n₃) M₂:(n₁,n₄) M₃:(n₂,n₃) M₄:(n₂,n₄)
     │          │          │          │
     └──────────┴──────────┴──────────┘
                │
                ▼ Create edges for each matching
     ┌──────────┼──────────┬──────────┐
     e₁         e₂         e₃         e₄
     │          │          │          │
     │          └──────────┴──────────┘
     │ backtrack through past edges (depth=2)
     │
t₂:  ▼
     Find historical source nodes and apply rules
     with subset difference
     │
     ▼ Create inferred edges
     e₅, e₆, e₇, ... (limited by breadth strategy)

Inference result: edges = {e₁, e₂, e₃, e₄, e₅, e₆, e₇, ...}
```

**Full Matching Example with Concrete Data:**
```
Wave: w = {p₁={1,2}, p₂={3}}

Graph particles:
  n₁ = {1,2}      ← n₁ ⊇ p₁ ✓
  n₂ = {1,2,4}    ← n₂ ⊇ p₁ ✓
  n₃ = {3}        ← n₃ ⊇ p₂ ✓
  n₄ = {3,5}      ← n₄ ⊇ p₂ ✓
  n₅ = {1}        ← n₅ ⊉ p₁ ✗ (missing 2)
  n₆ = {6}        ← n₆ ⊉ p₂ ✗ (missing 3)

Valid full matchings (each matches ALL wave particles):
  M₁: (p₁→n₁, p₂→n₃)   creates edge e₁: {n₁,n₃} with wave pattern
  M₂: (p₁→n₁, p₂→n₄)   creates edge e₂: {n₁,n₄} with wave pattern
  M₃: (p₁→n₂, p₂→n₃)   creates edge e₃: {n₂,n₃} with wave pattern
  M₄: (p₁→n₂, p₂→n₄)   creates edge e₄: {n₂,n₄} with wave pattern

Each edge captures: "These graph particles match the wave pattern"
```
