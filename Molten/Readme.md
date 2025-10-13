<p align="center"><img src="../resource/Molten.logo.png" alt="Molten logo" width="61.8%"/></p>
<h1 align="center" style="font-size:2.5rem;margin-top:-0.3em;">Molten</h1>
<p align="center"><em>Computational expression over hypergraphs</em></p>

<p align="center">
  <a href="../Info.md"><strong>Info</strong></a> &nbsp;|&nbsp; <a href="../Notice.md"><strong>Notice</strong></a> &nbsp;|&nbsp; <a href="../README.md"><strong>Vantle</strong></a> &nbsp;|&nbsp; <a href="../MODULE.bazel"><strong>Module</strong></a> &nbsp;|&nbsp; <a href="../License.md"><strong>License</strong></a>
</p>

---

This document describes **all of the semantics** of Molten's computational expression. Since Molten programs build **hypergraphs**, you are encouraged to embrace _polymorphism_ to keep your code efficient and reusable.

---

## Theory

- **Concept** – an atom such as `Human`, `Earth`, `Ready`. Stick to **one word per concept**; if you need multiple words, chain them with dots – for example `Earth.Location` or `Human.Man`. Avoid compound words. Invent new words. Ultimately, a concept is defined by its relationship to other concepts, the label is a comment for a consumer, such as yourself (help yourself understand your own graphs).
  > Avoid CamelCase (`LikeThis`) or underscores (`like_this`). Dots are the official way to express a path of related concepts.
  >
  > Why? CamelCase and other multi-word mash-ups often hide multiple ideas inside a single label—an indicator of weak or muddled abstraction. Molten encourages _one word → one concept_. If you genuinely need two ideas, write them as `First.Second` so their relationship is explicit in the graph.
- **Orthogonality** – an independent dimension of evaluation holding its own set of concepts. Evaluation starts with one orthogonality; some constructs create additional ones that may later merge back together.
- **Relation** - a source to **any expression** except a partition or `void`; use this to express polymorphic edges inside the hypergraph.

---

## Syntax

| Symbol  | Pronounce it as… | What it does                                                                                           |
| ------- | ---------------- | ------------------------------------------------------------------------------------------------------ |
| `.`     | “with”           | Group concepts together within the graph (no ordering).                                                |
| `,`     | “meanwhile”      | Split the current orthogonality into **additional** parallel orthogonality.                            |
| `[A]`   | “from A”         | A source expression of how to walk the hypergraph.                                                     |
| `( … )` | —                | Groups sub-expressions within a partition; used for precedence and clarity. Recursively applies rules. |

> File extensions: a `.lava` file indicates a runnable Molten _script_, while a `.magma` file indicates a reusable Molten _library_. The semantics are identical; the distinction exists purely to help humans reason about intent.

---

## Syntax

### Textual

- **Orderless.** Molten does not march through the source left-to-right. Each orthogonality advances _only_ when the rule in front of it is enabled. Rules are ordered based upon `,` `()`, and `[]` semantics.
- A **dot** `.` simply groups concepts _with_ one another inside the same orthogonality.
- A **comma** `,` clones the current orthogonality **once for every extra branch**, so you can have any number of orthogonalities running in parallel. They move forward independently until another rule brings them back together.
- A **bracket** `[…]` blocks its arriving orthogonality(ies) until they already hold every listed concept. Once satisfied it removes those concepts, inserts the ones that follow the bracket, and lets the orthogonality(ies) proceed.
- A **parentheses** `()` groups items within a partition; they have no effect on state by themselves.

---

## Compositions

### Join — `[A, B, …] C`

Any number of parallel orthogonalities – each carrying one of the required concepts (`A`, `B`, etc.) – must all reach the bracket. When they do, they **fuse** into a single orthogonality; the listed concepts disappear and `C` is added. Execution then proceeds in that unified orthogonality.

### Scope — `[A] ([B.C] D)`

An orthogonality holding `A` replaces it with the grouped expression `([B.C] D)`. Inside that scope, `B.C` is required; once present, it is swapped for `D`. The scope then collapses, leaving `D` in the orthogonality’s state.

## Examples

```molten
[] Human.Male, 
[] Earth.Location.America,
[Human.Male, Earth.Location.America] American.Citizen.Male
```

Note that this function is an _infinite generator_ of the `American.Citzen.Male` graph. First, it generates `Human.Male, Earth.Location.America`, which then joins to `American.Citizen.Male`. At any given state (with breadth-first evaluation), there is one or zero copies of `Human.Male, Earth.Location.America` and an every increasing copy of `American.Citizen.Male`.

### Additional

- [Turing machine](test/resource/system/graph/module/math/numeric/logic/boolean/symbolic/boolean.magma)
- [Joins](test/resource/system/graph/module/join/symbolic/join.magma)

## Forge

Forge `1.0.0` supports temporal runtime for `Molten`

### Invoke

Run the temporal interactive runtime:

```bash
bazel run //Molten/system/forge temporal
```

This starts an interactive session where you can enter Molten expressions line by line. Each expression is evaluated and the resulting hypergraph state is displayed.

© 2025 Vantle · @robert.vanderzee
