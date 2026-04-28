---
name: article-writing
invoke: article-writing
description: Technical Writing for Rust
user_invocable: true
origin: everything-ai-need
---

# Technical Writing for Rust

## When to Activate

- Writing Rust tutorials
- Creating documentation
- Writing blog posts
- Explaining Rust concepts
- API documentation

---

## Article Structure Template

### 1. Problem Statement
What problem are you solving? Why does it matter?

### 2. Solution (Code First)
Show working code before explanation.

### 3. Why It Works
Explain the mechanisms and rationale.

### 4. Common Pitfalls
What mistakes do people make?

### 5. Further Reading
Where to learn more.

---

## Code-First Approach

**Good:** Show code, then explain
```markdown
Here's how to handle errors in Rust:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
}
```

The `thiserror` crate lets you define typed errors with minimal boilerplate.
```

**Bad:** Explain before showing
```markdown
Rust has a powerful error handling system. You should use the `thiserror` crate to define custom errors. Here's an example:
[code follows]
```

---

## Voice Guidelines

### Use "we/our" not "you/your"
- ✅ "We'll implement error handling"
- ❌ "You should implement error handling"

### Avoid filler words
- ❌ "simply", "just", "obviously", "clearly"
- ✅ Be direct and specific

### Show errors before fixes
```markdown
This code won't compile:

```rust
let s = String::from("hello");
let s2 = s;
println!("{}", s); // ERROR!
```

The fix is to clone:

```rust
let s2 = s.clone();
```
```

### Specify versions
```markdown
Tested with Rust 1.75 and tokio 1.35.
```

---

## Code Examples

### Complete, Runnable Examples

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");
    sleep(Duration::from_secs(1)).await;
    println!("Done!");
}
```

### Show Output

```rust
fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
```

```
Output:
[1, 2, 3]
```

### Progressive Complexity

Start simple, add complexity:

1. **Basic**: Simplest working version
2. **Real-world**: Production-ready with error handling
3. **Advanced**: Optimization or edge cases

---

## Section Templates

### Tutorial Opening
```markdown
# Building a REST API with Axum

In this tutorial, we'll build a production-ready REST API using Axum,
SQLx, and Tokio. By the end, you'll have a working API with proper
error handling, database integration, and testing.

**Prerequisites:**
- Rust 1.75+
- PostgreSQL installed
- Basic Rust knowledge

**Final code:** [github.com/user/repo](link)
```

### Concept Explanation
```markdown
## Understanding Lifetimes

Lifetimes ensure references are always valid. Consider this problem:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

This won't compile! The return type needs a lifetime:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` annotation means: the returned reference lives as long as
the shorter of the two inputs.
```

### API Documentation
```markdown
## `UserService::create`

Creates a new user with the given email and name.

```rust
pub async fn create(&self, req: CreateUserRequest) -> Result<User, AppError>
```

**Parameters:**
- `req` - The user creation request containing email and name

**Returns:**
- `Ok(User)` - The created user with assigned ID
- `Err(AppError::Validation)` - If email is invalid or already exists

**Example:**
```rust
let user = user_service.create(CreateUserRequest {
    email: "alice@example.com".to_string(),
    name: "Alice".to_string(),
}).await?;
```

**Errors:**
- Returns `Validation` error if email format is invalid
- Returns `Validation` error if email already registered
```

---

## Editing Checklist

- [ ] Code compiles and runs
- [ ] All examples tested
- [ ] Links work
- [ ] No filler words (simply, just, obviously)
- [ ] Consistent voice (we/our)
- [ ] Version numbers specified
- [ ] Errors shown before solutions
- [ ] Output shown for code examples
