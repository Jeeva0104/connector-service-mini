---
name: iterative-retrieval
invoke: iterative-retrieval
description: Iterative Retrieval
user_invocable: true
origin: everything-ai-need
---

# Iterative Retrieval

## When to Activate

- Delegating to planner agent
- Breaking down complex Rust refactors
- When context is too large for single prompt

---

## The Pattern

```
Round 1: High-level question
→ "Describe the module structure"

Round 2: Drill down
→ "Show the UserService trait and its implementations"

Round 3: Implementation details
→ "How is the repository pattern implemented?"

Round 4: Specific code
→ "Show the SQLx query for find_by_id"
```

---

## Subagent Orchestration

```rust
// Don't send entire codebase
// Send architecture summary first

let context = format!(
    "Module: {}\nTraits: {}\nImplementations: {}",
    module_summary, traits_list, impls_list
);

// Subagent requests more details as needed
```

---

## Best Practices

- Always start with module/trait overview
- Let subagent request specific files
- Cache retrieved context for follow-up queries
