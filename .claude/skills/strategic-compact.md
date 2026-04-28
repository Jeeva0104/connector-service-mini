---
name: strategic-compact
invoke: strategic-compact
description: Strategic Compact
user_invocable: true
origin: everything-ai-need
---

# Strategic Compact

## When to Activate

- Session is getting long (>50 exchanges)
- Context window is filling
- Starting a new major feature

---

## Compaction Strategy

1. **Summarize completed work**
   ```
   "We've implemented:
   - UserService trait with CRUD operations
   - JWT authentication middleware
   - PostgreSQL repository with SQLx"
   ```

2. **Identify current state**
   ```
   "Currently working on:
   - OrderService implementation
   - Pending: PaymentService"
   ```

3. **Archive old context**
   ```
   "UserService is complete and tested.
   Reference: src/services/user_service.rs"
   ```

---

## Session Summary Format

```markdown
## Session Summary

### Completed
- [x] User module (service, repo, handler)
- [x] JWT auth middleware

### In Progress
- [ ] Order module (service drafted, tests pending)

### Next
- [ ] Payment module
- [ ] Integration tests
```
