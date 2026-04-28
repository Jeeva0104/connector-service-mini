---
name: skill-stocktake
invoke: skill-stocktake
description: Skill Stocktake
user_invocable: true
origin: everything-ai-need
---

# Skill Stocktake

## When to Activate

- Starting a new Rust project
- Reviewing existing project skills
- Identifying knowledge gaps

---

## Inventory Template

```markdown
## Rust Skills Inventory

### Language Core [✓]
- [x] Ownership & borrowing
- [x] Lifetimes
- [x] Traits (all types)
- [x] Error handling (Result, Option)

### Async [✓]
- [x] Tokio runtime
- [x] Channels
- [x] Select!

### Web Frameworks [✓]
- [x] Axum
- [ ] Actix-web (gap!)

### Database [△]
- [x] SQLx
- [ ] Diesel (gap!)
- [ ] Connection pooling (weak)

### Deployment [✗]
- [ ] Docker multi-stage
- [ ] Cross-compilation
```

---

## Gap Analysis

| Skill | Needed | Available | Gap |
|-------|--------|-----------|-----|
| Diesel ORM | High | None | CRITICAL |
| WebSockets | Medium | Partial | MEDIUM |
| gRPC/Tonic | Low | None | LOW |

---

## Action Items

1. Create `skills/rust-diesel/` for ORM gap
2. Add `skills/rust-websockets/` for real-time features
