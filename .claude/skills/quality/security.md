# Security Review Checklist

This reference provides a comprehensive security checklist based on OWASP guidelines and Rust-specific security patterns.

## Severity Levels

- **CRITICAL**: Immediate exploitation risk, must fix before any deployment
- **HIGH**: Significant security risk, should fix before release
- **MEDIUM**: Potential security weakness, fix in near term
- **LOW**: Minor issue or defense-in-depth improvement

## 1. Secrets and Credentials

### Never Hardcode Secrets

```rust
// CRITICAL - hardcoded secrets
const API_KEY: &str = "sk-1234567890abcdef";
const DATABASE_URL: &str = "postgres://user:password@localhost/db";

// GOOD - use environment variables
fn key() -> Result<String, Error> {
    std::env::var("API_KEY")
        .map_err(|_| Error::Missing)
}
```

### Check for Common Secret Patterns

Look for these patterns in code:
- `password`, `passwd`, `pwd`
- `secret`, `token`, `key`, `api_key`, `apikey`
- `credential`, `auth`
- Base64-encoded strings that look like tokens
- Long alphanumeric strings (32+ chars)

### Git History

Secrets may have been committed and removed. Check:
```bash
git log -p --all -S 'password' -- '*.rs'
git log -p --all -S 'secret' -- '*.rs'
```

## 2. Input Validation

### Validate at System Boundaries

All external input must be validated:
- Command line arguments
- Environment variables
- File contents
- Network input
- User-provided data

```rust
// GOOD - validate input at boundary
fn process(input: &str) -> Result<Output, Error> {
    let validated = validate(input)?;  // Validate first
    internal_process(validated)         // Then use
}

fn validate(input: &str) -> Result<input::Validated, Error> {
    if input.len() > MAX {
        return Err(Error::Length);
    }
    if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::Characters);
    }
    Ok(input::Validated(input.to_string()))
}

// BAD - using input directly
fn process(input: &str) -> Output {
    internal_process(input)  // No validation!
}
```

### Newtype Pattern for Validated Data

Use newtypes to ensure validated data stays validated:

```rust
// GOOD - newtype prevents mixing validated/unvalidated
mod path {
    pub struct Validated(PathBuf);

    impl Validated {
        pub fn new(path: &str) -> Result<Self, Error> {
            let path = PathBuf::from(path);
            if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
                return Err(Error::Traversal);
            }
            Ok(Self(path))
        }
    }
}
```

## 3. Injection Prevention

### Command Injection

Never construct shell commands from user input:

```rust
// CRITICAL - command injection
fn run(filename: &str) {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("cat {}", filename))  // User controls filename!
        .spawn();
}

// GOOD - use typed arguments
fn run(filename: &str) -> Result<(), Error> {
    let validated = validate(filename)?;
    std::process::Command::new("cat")
        .arg(validated)  // As typed argument, not shell string
        .spawn()?;
    Ok(())
}
```

### Path Traversal

Prevent `../` attacks:

```rust
// CRITICAL - path traversal
fn read(name: &str) -> Result<String, Error> {
    let path = format!("/data/{}", name);  // name could be "../../../etc/passwd"
    std::fs::read_to_string(path)
}

// GOOD - validate and canonicalize
fn read(name: &str) -> Result<String, Error> {
    let base = PathBuf::from("/data");
    let requested = base.join(name);
    let canonical = requested.canonicalize()?;

    if !canonical.starts_with(&base) {
        return Err(Error::Traversal);
    }

    std::fs::read_to_string(canonical)
}
```

### SQL Injection (if applicable)

Always use parameterized queries:

```rust
// CRITICAL - SQL injection
fn find(name: &str) -> Result<User, Error> {
    query(&format!("SELECT * FROM users WHERE name = '{}'", name))
}

// GOOD - parameterized query
fn find(name: &str) -> Result<User, Error> {
    query("SELECT * FROM users WHERE name = $1", &[name])
}
```

## 4. Error Handling Security

### No Sensitive Info in Errors

```rust
// HIGH - leaks internal paths and sensitive data
mod database {
    #[error("Database error: {0}")]
    Error(String),  // BAD: Might contain connection strings!
}

mod config {
    #[error("Failed to read at {path}: {error}")]
    Error { path: PathBuf, error: std::io::Error },  // BAD: Leaks paths!
}

// GOOD - generic errors for external consumers
mod database {
    #[error("Database unavailable")]
    #[diagnostic(code(database::unavailable), help("Please try again later"))]
    Unavailable,
}

mod config {
    #[error("Configuration error")]
    #[diagnostic(code(config::error), help("Contact administrator"))]
    Error,
}
```

### Log Sensitive Errors Internally Only

```rust
fn handle(err: internal::Error) -> external::Error {
    tracing::error!(?err, "Internal error occurred");  // Full details to logs
    external::Error::Internal  // Generic message to user
}
```

## 5. Memory Safety

### Avoid Unsafe Unless Absolutely Necessary

```rust
// Review all unsafe blocks carefully
unsafe {
    // Every line here needs justification
}
```

Questions for each `unsafe` block:
1. Is this truly necessary?
2. Are all invariants documented?
3. Is the unsafe surface area minimized?
4. Has this been reviewed by multiple people?

### Integer Overflow

```rust
// MEDIUM - potential overflow in release builds
fn calculate(a: u32, b: u32) -> u32 {
    a + b  // Can overflow!
}

// GOOD - explicit handling
fn calculate(a: u32, b: u32) -> Option<u32> {
    a.checked_add(b)
}

fn calculate(a: u32, b: u32) -> u32 {
    a.saturating_add(b)  // Saturates at MAX
}
```

## 6. Dependency Security

### Check for Known Vulnerabilities

```bash
cargo audit  # Check for known vulnerabilities
cargo outdated  # Check for updates
```

### Review New Dependencies

Before adding a dependency, check:
1. Is it actively maintained?
2. Does it have a security policy?
3. What permissions does it need?
4. How many transitive dependencies does it add?

## 7. Cryptography

### Use Standard Libraries

```rust
// BAD - rolling your own crypto
fn hash(password: &str) -> String {
    // Custom hashing implementation
}

// GOOD - use established libraries
// ring, rust-crypto, sodiumoxide, argon2
```

### Secure Random Numbers

```rust
// BAD - predictable
use rand::Rng;
let token: u64 = rand::thread_rng().gen();

// GOOD - cryptographically secure
use rand::rngs::OsRng;
let mut token = [0u8; 32];
OsRng.fill_bytes(&mut token);
```

## 8. Concurrency Security

### Avoid Data Races

```rust
// Check for proper synchronization
// - Mutex/RwLock for shared mutable state
// - Atomic types for simple counters
// - Channels for message passing
```

### Deadlock Prevention

Review lock ordering and ensure consistent acquisition order.

## Review Checklist Summary

For each file, verify:

- [ ] No hardcoded secrets, API keys, or credentials
- [ ] All external input validated at boundaries
- [ ] No command injection vulnerabilities
- [ ] No path traversal vulnerabilities
- [ ] No SQL injection (if applicable)
- [ ] Error messages don't leak sensitive info
- [ ] Unsafe blocks are justified and minimal
- [ ] Integer operations handle overflow
- [ ] Dependencies are up-to-date and secure
- [ ] Cryptographic operations use standard libraries
- [ ] Concurrent code is properly synchronized
