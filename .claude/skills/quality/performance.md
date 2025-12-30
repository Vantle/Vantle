# Performance Review Patterns

This reference provides patterns for identifying and fixing performance issues in Rust code.

## Impact Levels

- **HIGH**: Visible latency impact, O(n^2+) complexity, or significant memory waste
- **MEDIUM**: Noticeable in profiling, affects throughput
- **LOW**: Micro-optimization, only matters at scale

## 1. Unnecessary Allocations

### Avoid Unnecessary Clone

```rust
// HIGH - cloning when borrowing would work
fn process(data: Vec<String>) {
    for item in data.clone() {  // Why clone?
        println!("{}", item);
    }
}

// GOOD - just iterate
fn process(data: &[String]) {
    for item in data {
        println!("{}", item);
    }
}
```

### Avoid Unnecessary to_string/to_owned

```rust
// MEDIUM - allocating when borrowing works
fn greet(name: &str) {
    let greeting = format!("Hello, {}", name.to_string());  // Redundant
}

// GOOD - format! handles &str
fn greet(name: &str) {
    let greeting = format!("Hello, {}", name);
}
```

### String Building

```rust
// HIGH - O(n^2) string concatenation
fn build(items: &[&str]) -> String {
    let mut result = String::new();
    for item in items {
        result = result + item;  // Creates new String each time!
    }
    result
}

// GOOD - preallocate and push
fn build(items: &[&str]) -> String {
    let total: usize = items.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total);
    for item in items {
        result.push_str(item);
    }
    result
}

// GOOD - use join for this case
fn build(items: &[&str]) -> String {
    items.join("")
}
```

### Vec Preallocation

```rust
// MEDIUM - many reallocations
fn collect(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..n {
        result.push(i as i32);  // Grows exponentially
    }
    result
}

// GOOD - preallocate when size is known
fn collect(n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push(i as i32);
    }
    result
}

// BEST - use collect
fn collect(n: usize) -> Vec<i32> {
    (0..n).map(|i| i as i32).collect()
}
```

## 2. Iteration Patterns

### Avoid Collect When Streaming Works

```rust
// MEDIUM - unnecessary intermediate collection
fn count(items: &[Item]) -> usize {
    items
        .iter()
        .filter(|i| i.active)
        .collect::<Vec<_>>()  // Why collect?
        .len()
}

// GOOD - stream directly
fn count(items: &[Item]) -> usize {
    items.iter().filter(|i| i.active).count()
}
```

### Avoid Multiple Passes

```rust
// MEDIUM - iterating twice
fn stats(values: &[i32]) -> (i32, i32) {
    let min = values.iter().min().copied().unwrap_or(0);
    let max = values.iter().max().copied().unwrap_or(0);
    (min, max)
}

// GOOD - single pass
fn stats(values: &[i32]) -> (i32, i32) {
    values.iter().fold((i32::MAX, i32::MIN), |(min, max), &v| {
        (min.min(v), max.max(v))
    })
}
```

### N+1 Query Patterns

```rust
// HIGH - N+1 queries
fn load(ids: &[Id]) -> Vec<User> {
    ids.iter()
        .map(|id| database.get(id))  // One query per ID!
        .collect()
}

// GOOD - batch query
fn load(ids: &[Id]) -> Vec<User> {
    database.get_many(ids)
}
```

## 3. Data Structure Choice

### Vec vs HashMap

```rust
// For small collections (< 50 items), Vec with linear search
// is often faster than HashMap due to cache locality

// GOOD for small sets
fn contains(items: &[u32], target: u32) -> bool {
    items.iter().any(|&x| x == target)
}

// GOOD for large sets
fn contains(items: &HashSet<u32>, target: u32) -> bool {
    items.contains(&target)
}
```

### HashMap vs BTreeMap

```rust
// HashMap: O(1) average, unordered, faster for random access
// BTreeMap: O(log n), ordered, faster for range queries

// Use HashMap for lookups
let cache: HashMap<Key, Value> = ...;

// Use BTreeMap when order matters or for range queries
let ordered: BTreeMap<Key, Value> = ...;
for (k, v) in ordered.range(start..end) { ... }
```

### Box vs Inline

```rust
// Boxing adds indirection; avoid for hot paths

// MEDIUM - unnecessary indirection
struct Node {
    value: Box<i32>,  // Why box a primitive?
    next: Option<Box<Node>>,
}

// GOOD - inline small types
struct Node {
    value: i32,
    next: Option<Box<Node>>,  // Box here is necessary for recursion
}
```

## 4. Async Patterns

### Blocking in Async Context

```rust
// HIGH - blocking the async runtime
async fn process(path: &Path) -> Result<Data, Error> {
    let content = std::fs::read_to_string(path)?;  // Blocks!
    Ok(parse(content))
}

// GOOD - use async IO
async fn process(path: &Path) -> Result<Data, Error> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(parse(content))
}

// GOOD - spawn blocking for CPU-intensive work
async fn process(data: &[u8]) -> Result<Output, Error> {
    let data = data.to_vec();
    tokio::task::spawn_blocking(move || {
        heavy_computation(&data)
    }).await?
}
```

### Unnecessary Await in Loop

```rust
// HIGH - sequential when parallel is possible
async fn fetch(urls: &[Url]) -> Vec<Response> {
    let mut results = Vec::new();
    for url in urls {
        results.push(fetch_one(url).await);  // Sequential!
    }
    results
}

// GOOD - parallel fetching
async fn fetch(urls: &[Url]) -> Vec<Response> {
    futures::future::join_all(urls.iter().map(fetch_one)).await
}
```

## 5. Algorithm Complexity

### Avoid O(n^2) When O(n log n) or O(n) Is Possible

```rust
// HIGH - O(n^2) nested loop
fn has_duplicates(items: &[i32]) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if items[i] == items[j] {
                return true;
            }
        }
    }
    false
}

// GOOD - O(n) with HashSet
fn has_duplicates(items: &[i32]) -> bool {
    let mut seen = HashSet::with_capacity(items.len());
    items.iter().any(|x| !seen.insert(x))
}

// GOOD - O(n log n) with sorting (if mutation is acceptable)
fn has_duplicates(items: &mut [i32]) -> bool {
    items.sort_unstable();
    items.windows(2).any(|w| w[0] == w[1])
}
```

### Look for Repeated Expensive Operations

```rust
// HIGH - repeated computation
fn process(items: &[Item]) -> Vec<Output> {
    items
        .iter()
        .map(|item| {
            let config = load_config();  // Loading every iteration!
            transform(item, &config)
        })
        .collect()
}

// GOOD - compute once
fn process(items: &[Item]) -> Vec<Output> {
    let config = load_config();
    items
        .iter()
        .map(|item| transform(item, &config))
        .collect()
}
```

## 6. Memory Layout

### Consider Cache Locality

```rust
// Array of Structs (AoS) - good for accessing all fields of one item
struct Particle {
    x: f32, y: f32, z: f32,
    vx: f32, vy: f32, vz: f32,
}
let particles: Vec<Particle> = ...;

// Struct of Arrays (SoA) - good for accessing one field of many items
struct Particles {
    x: Vec<f32>, y: Vec<f32>, z: Vec<f32>,
    vx: Vec<f32>, vy: Vec<f32>, vz: Vec<f32>,
}

// Choose based on access patterns
```

## Review Checklist Summary

For each file, check:

- [ ] No unnecessary `.clone()` or `.to_string()` calls
- [ ] Vecs preallocated when size is known
- [ ] No `.collect()` followed by iteration (stream instead)
- [ ] No N+1 patterns (batch operations)
- [ ] Appropriate data structure for access patterns
- [ ] No blocking operations in async contexts
- [ ] Parallel processing where applicable
- [ ] No O(n^2) when O(n log n) or O(n) is possible
- [ ] Expensive operations not repeated unnecessarily
- [ ] Memory layout appropriate for access patterns
