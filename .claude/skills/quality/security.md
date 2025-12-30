# Security Review Checklist

OWASP-based security checklist for Rust code.

## Severity Levels

- **CRITICAL**: Immediate exploitation risk
- **HIGH**: Significant security risk
- **MEDIUM**: Potential weakness
- **LOW**: Defense-in-depth improvement

## 1. Secrets and Credentials

```rust
// CRITICAL - hardcoded secrets
const API_KEY: &str = "sk-1234567890abcdef";

// GOOD - environment variables
fn key() -> Result<String, Error> {
    std::env::var("API_KEY").map_err(|_| Error::Missing)
}
```

Look for: `password`, `secret`, `token`, `key`, `api_key`, `credential`, `auth`, Base64 strings, long alphanumeric strings (32+ chars).

## 2. Input Validation

Validate all external input at boundaries:

```rust
// GOOD - validate then use
fn process(input: &str) -> Result<Output, Error> {
    let validated = validate(input)?;
    internal_process(validated)
}

// Use newtypes to prevent mixing validated/unvalidated
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

```rust
// CRITICAL - user controls shell string
std::process::Command::new("sh")
    .arg("-c")
    .arg(format!("cat {}", filename))
    .spawn();

// GOOD - typed arguments
std::process::Command::new("cat")
    .arg(validated_filename)
    .spawn()?;
```

### Path Traversal

```rust
// CRITICAL - name could be "../../../etc/passwd"
let path = format!("/data/{}", name);

// GOOD - validate and canonicalize
let base = PathBuf::from("/data");
let canonical = base.join(name).canonicalize()?;
if !canonical.starts_with(&base) {
    return Err(Error::Traversal);
}
```

### SQL Injection

```rust
// CRITICAL
query(&format!("SELECT * FROM users WHERE name = '{}'", name))

// GOOD - parameterized
query("SELECT * FROM users WHERE name = $1", &[name])
```

## 4. Error Handling Security

```rust
// HIGH - leaks internal paths
#[error("Failed to read at {path}: {error}")]
Error { path: PathBuf, error: std::io::Error },

// GOOD - generic for external consumers
#[error("Configuration error")]
#[diagnostic(code(config::error), help("Contact administrator"))]
Error,
```

Log details internally, return generic errors externally:

```rust
fn handle(err: internal::Error) -> external::Error {
    tracing::error!(?err, "Internal error occurred");
    external::Error::Internal
}
```

## 5. Memory Safety

Review all `unsafe` blocks:

1. Is this truly necessary?
2. Are invariants documented?
3. Is unsafe surface minimized?

Handle integer overflow:

```rust
// Use checked_add, saturating_add instead of +
a.checked_add(b)
a.saturating_add(b)
```

## 6. Dependency Security

```bash
cargo audit   # Known vulnerabilities
cargo outdated  # Updates available
```

## 7. Cryptography

Use standard libraries (ring, sodiumoxide, argon2). Use `OsRng` for cryptographic randomness.

## 8. Concurrency

- Mutex/RwLock for shared mutable state
- Consistent lock ordering to prevent deadlocks

## Checklist

- [ ] No hardcoded secrets
- [ ] External input validated at boundaries
- [ ] No command/path/SQL injection
- [ ] Error messages don't leak sensitive info
- [ ] Unsafe blocks justified and minimal
- [ ] Integer overflow handled
- [ ] Dependencies secure
- [ ] Standard crypto libraries
- [ ] Proper synchronization
