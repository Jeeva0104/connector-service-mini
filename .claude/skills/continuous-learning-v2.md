---
name: continuous-learning-v2
invoke: continuous-learning-v2
description: Instinct-Based Learning (v2)
user_invocable: true
origin: everything-ai-need
---

# Instinct-Based Learning (v2)

## When to Activate

- Tracking effectiveness of Rust patterns
- Deciding between `impl Trait` vs `dyn Trait`
- Evaluating error handling strategies

---

## Confidence Scoring

| Pattern | Confidence | Evidence |
|---------|------------|----------|
| Use `Arc<Mutex<T>>` for shared state | 95% | 50+ projects |
| Prefer `thiserror` over custom enums | 90% | 30+ libraries |
| `tokio::select!` for timeouts | 85% | 20+ services |

---

## Evolution Rules

1. **High confidence (>90%)** — Make into a rule
2. **Medium confidence (70-90%)** — Document as guideline
3. **Low confidence (<70%)** — Flag for review

---

## Instinct Format

```markdown
---
confidence: 85
evidence_count: 20
trigger: "Need concurrent branches with timeout"
action: "Use tokio::select! with sleep future"
---

## Why This Works
Select polls all futures, returns when first completes.
```
