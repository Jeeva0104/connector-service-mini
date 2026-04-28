---
name: deep-research
invoke: deep-research
description: Deep Research for Rust
user_invocable: true
origin: everything-ai-need
---

# Deep Research for Rust

## When to Activate

- Choosing between similar crates
- Understanding unsafe Rust
- Learning advanced patterns
- Evaluating async runtimes
- Making architectural decisions
- Researching best practices

---

## Research Sources

### Official Documentation
- **docs.rs** - Crate documentation
- **crates.io** - Download stats, versions
- **The Rust Book** - Language fundamentals
- **The Rustonomicon** - Unsafe Rust details
- **Rust By Example** - Practical patterns

### Community Resources
- **GitHub issues/PRs** - Known problems, roadmap
- **Rust Users Forum** - Community discussions
- **Discord servers** - Real-time help
- **This Week in Rust** - Ecosystem updates

### Analysis Tools
```bash
# Check for unsafe code
cargo install cargo-geiger
cargo geiger

# Compare crate sizes
cargo bloat --release

# Check dependencies
cargo tree
cargo tree -d  # duplicates
```

---

## Crate Evaluation Matrix

### Comparison Template

| Criteria | Crate A | Crate B | Crate C |
|----------|---------|---------|---------|
| Downloads/month | 100K | 50K | 10K |
| Last update | 2 days | 3 months | 1 year |
| Open issues | 10 | 50 | 100 |
| Unsafe code | No | Minimal | Yes |
| License | MIT | Apache-2 | GPL |
| Documentation | Excellent | Good | Poor |
| Maintenance | Active | Maintenance | Unmaintained |

### Key Metrics

1. **Maintenance Status**
   - Last commit date (< 3 months ideal)
   - Response time on issues
   - Release frequency

2. **Code Quality**
   - `cargo clippy` warnings
   - Test coverage
   - Documentation coverage
   - Unsafe code presence

3. **Ecosystem Fit**
   - Compatibility with your dependencies
   - Common patterns in similar projects
   - Integration with async runtime

---

## Research Process

### 1. Define Requirements

```markdown
## Requirements for [Decision]

Must have:
- [ ] Async support
- [ ] No unsafe code
- [ ] MIT/Apache license

Nice to have:
- [ ] Built-in metrics
- [ ] Pluggable backends

Deal breakers:
- [ ] GPL license
- [ ] Unmaintained
```

### 2. Identify Candidates

```bash
# Search crates.io
cargo search "async http client"

# Check reverse dependencies
cargo tree -i some-crate
```

### 3. Evaluate Each Option

```rust
// Create spike project for each option
cargo new --lib spike-crate-a
cd spike-crate-a
cargo add crate-a

// Implement common test case
// Compare ergonomics, performance
```

### 4. Document Findings

```markdown
# Decision: [Topic]

## Options Considered

### Option A: [crate-name]
**Pros:**
- Fast, 100K downloads/month
- Excellent docs

**Cons:**
- Uses some unsafe code
- Large dependency tree

**Verdict:** Rejected due to unsafe code requirement

### Option B: [crate-name]
**Pros:**
- Pure safe Rust
- Small footprint

**Cons:**
- Less mature
- Smaller community

**Verdict:** Accepted

## Decision
Use [Option B] because [reasoning].
```

---

## Specific Research Topics

### Async Runtime Comparison

| Feature | tokio | async-std | smol |
|---------|-------|-----------|------|
| Ecosystem | Largest | Medium | Small |
| Performance | Excellent | Good | Good |
| Learning curve | Moderate | Easy | Easy |
| Compatibility | Best | Good | Limited |

**Recommendation:** Use tokio for production, async-std for simplicity.

### Serialization: serde vs others

- **serde**: Industry standard, derive macros, fast
- **miniserde**: Smaller, no proc macros
- **nanoserde**: Minimal dependencies

**Use serde** unless binary size is critical.

### HTTP Clients

- **reqwest**: High-level, easy API
- **hyper**: Low-level, more control
- **ureq**: Synchronous, minimal deps

**Use reqwest** for async, **ureq** for sync scripts.

---

## Unsafe Rust Research

### When to Use Unsafe
- FFI bindings
- Performance-critical code
- Low-level memory operations

### Safety Checklist
- [ ] Invariant documented in `// SAFETY:` comment
- [ ] All preconditions checked
- [ ] Lifetimes correct
- [ ] No undefined behavior possible
- [ ] Reviewed by second person

### Resources
- The Rustonomicon (essential)
- RFC 2585 (unsafe guidelines)
- Miri tool for UB detection

---

## Decision Template

```markdown
# Architectural Decision Record: [Title]

## Status
Proposed / Accepted / Deprecated

## Context
What problem are we solving?

## Decision
What did we decide?

## Consequences
- Positive: ...
- Negative: ...

## Alternatives Considered
- Alternative A: rejected because ...
- Alternative B: rejected because ...

## References
- [Link 1]
- [Link 2]
```

---

## Quick Research Commands

```bash
# Check crate info
cargo info crate-name

# View source locally
cargo clone crate-name
cd crate-name

# Check for unsafe
cargo geiger --all-targets

# Compare binary sizes
cargo build --release
ls -lh target/release/

# Check compile times
cargo build --timings
```
