# Performance Review Patterns

Patterns for identifying and fixing performance issues in Rust.

## Impact Levels

- **HIGH**: O(n^2+), visible latency, significant memory waste
- **MEDIUM**: Noticeable in profiling, affects throughput
- **LOW**: Micro-optimization, matters only at scale

## 1. Unnecessary Allocations

### Clone

```rust
// HIGH - unnecessary clone
for item in data.clone() { println!("{}", item); }

// GOOD - borrow
for item in &data { println!("{}", item); }
```

### to_string

```rust
// MEDIUM - redundant allocation
format!("Hello, {}", name.to_string())

// GOOD - format! handles &str
format!("Hello, {}", name)
```

### String Building

```rust
// HIGH - O(n^2) concatenation
result = result + item;

// GOOD - preallocate
let mut result = String::with_capacity(total);
result.push_str(item);

// BEST - join
items.join("")
```

### Vec Preallocation

```rust
// MEDIUM - many reallocations
let mut result = Vec::new();

// GOOD - preallocate when size known
let mut result = Vec::with_capacity(n);

// BEST - use collect
(0..n).map(|i| i as i32).collect()
```

## 2. Iteration Patterns

### Unnecessary Collect

```rust
// MEDIUM - intermediate collection
items.iter().filter(|i| i.active).collect::<Vec<_>>().len()

// GOOD - stream
items.iter().filter(|i| i.active).count()
```

### Multiple Passes

```rust
// MEDIUM - iterating twice
let min = values.iter().min();
let max = values.iter().max();

// GOOD - single pass
values.iter().fold((i32::MAX, i32::MIN), |(min, max), &v| {
    (min.min(v), max.max(v))
})
```

### N+1 Queries

```rust
// HIGH - one query per ID
ids.iter().map(|id| database.get(id)).collect()

// GOOD - batch
database.get_many(ids)
```

## 3. Data Structure Choice

- **Small collections (<50)**: Vec with linear search (cache locality)
- **Large collections**: HashSet/HashMap
- **Ordered + range queries**: BTreeMap
- **Avoid boxing primitives** on hot paths

## 4. Async Patterns

### Blocking in Async

```rust
// HIGH - blocks runtime
std::fs::read_to_string(path)?;

// GOOD - async IO
tokio::fs::read_to_string(path).await?;

// GOOD - spawn_blocking for CPU work
tokio::task::spawn_blocking(move || heavy_computation(&data)).await?
```

### Sequential vs Parallel

```rust
// HIGH - sequential
for url in urls { results.push(fetch(url).await); }

// GOOD - parallel
futures::future::join_all(urls.iter().map(fetch)).await
```

## 5. Algorithm Complexity

```rust
// HIGH - O(n^2)
for i in 0..items.len() {
    for j in (i+1)..items.len() { ... }
}

// GOOD - O(n) with HashSet
let mut seen = HashSet::with_capacity(items.len());
items.iter().any(|x| !seen.insert(x))

// GOOD - O(n log n) with sorting
items.sort_unstable();
items.windows(2).any(|w| w[0] == w[1])
```

### Repeated Expensive Operations

```rust
// HIGH - loading every iteration
items.iter().map(|item| {
    let config = load_config();  // Repeated!
    transform(item, &config)
})

// GOOD - compute once
let config = load_config();
items.iter().map(|item| transform(item, &config))
```

## 6. Memory Layout

- **AoS** (Array of Structs): Access all fields of one item
- **SoA** (Struct of Arrays): Access one field of many items

Choose based on access patterns.

## Checklist

- [ ] No unnecessary `.clone()` or `.to_string()`
- [ ] Vecs preallocated when size known
- [ ] No `.collect()` followed by iteration
- [ ] No N+1 patterns
- [ ] Appropriate data structure
- [ ] No blocking in async
- [ ] Parallel where applicable
- [ ] No O(n^2) when O(n log n) possible
- [ ] Expensive ops not repeated
