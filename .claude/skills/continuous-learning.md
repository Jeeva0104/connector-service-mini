---
name: continuous-learning
invoke: continuous-learning
description: Brief description of when this applies
user_invocable: true
origin: everything-ai-need
---

# Continuous Learning for Rust

## When to Activate

- After completing a complex Rust feature
- When you solve a borrow checker error
- When you discover a useful trait pattern
- After debugging async issues

---

## Pattern Extraction Process

1. **Recognize the pattern** — Was this borrow checker error common?
2. **Document the solution** — How did you fix it?
3. **Generalize** — Can this help in similar situations?
4. **Save as skill** — Add to `skills/custom/` directory

---

## Example: Borrow Checker Pattern

```rust
// Problem you solved
// E0502: cannot borrow `x` as mutable more than once

// Your solution (pattern to save)
// Use split_mut() for Vec, or collect indices first
```

**Save as:** `skills/custom/vec-mutable-iteration.md`

---

## Pattern Template

```markdown
---
name: pattern-name
description: Brief description of when this applies
origin: extracted-session
---

# Pattern Name

## When to Use
- Specific scenario 1
- Specific scenario 2

## The Pattern
```code example```

## Why It Works
Explanation of the Rust mechanics
```

---

## Best Practices

- Extract patterns immediately after solving problems
- Focus on borrow checker, lifetimes, trait usage
- Share extracted patterns with the team
