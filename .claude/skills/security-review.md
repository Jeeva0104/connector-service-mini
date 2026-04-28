---
name: security-review
invoke: security-review
description: Security Review Checklist
user_invocable: true
origin: everything-ai-need
---

# Security Review Checklist

## When to Activate

- Before production deployment
- During security audits
- When reviewing PRs containing unsafe code
- Adding new dependencies

---

## Code Security

### Unsafe Code
- [ ] Every `unsafe` block has a `// SAFETY:` comment explaining invariants
- [ ] All pointer dereferences are bounds-checked
- [ ] Lifetimes in unsafe code are correct
- [ ] Considered if safe Rust alternative exists
- [ ] Second reviewer approved the unsafe block

### Error Handling
- [ ] No `unwrap()` or `expect()` in production code
- [ ] All errors propagate properly with `?`
- [ ] Error messages don't leak sensitive info

### Input Validation
- [ ] All external inputs validated at system boundaries
- [ ] Use `validator` crate for struct validation
- [ ] SQL queries are parameterized (no string interpolation)
- [ ] File paths sanitized to prevent traversal attacks

### Secrets Management
- [ ] No hardcoded secrets in source code
- [ ] Use `secrecy` crate for sensitive strings
- [ ] Secrets loaded from environment variables
- [ ] `.env` file in `.gitignore`
- [ ] Never log secret values

---

## Dependency Security

### cargo audit
```bash
cargo install cargo-audit
cargo audit
```

- [ ] No RUSTSEC advisories (high/critical must be resolved)
- [ ] No yanked crates in dependency tree
- [ ] All dependencies actively maintained

### cargo deny
```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
```

```bash
cargo install cargo-deny
cargo deny check
```

- [ ] All licenses compatible with project
- [ ] No duplicate versions of same crate

### Dependency Minimization
- [ ] No unused dependencies (check with `cargo udeps`)
- [ ] No unnecessary feature flags enabled

---

## Database Security

- [ ] All queries use parameterized statements
- [ ] Connection pool limits configured
- [ ] Database user has minimal permissions
- [ ] SSL/TLS enabled for database connections

---

## Deployment Security

### Docker
- [ ] Container runs as non-root user
- [ ] No secrets in image layers
- [ ] Distroless or minimal base image
- [ ] HEALTHCHECK defined

### Environment
- [ ] Production secrets in secure vault (not env vars)
- [ ] Debug mode disabled in production
- [ ] Proper logging level (no debug logs in prod)

---

## Review Template

```markdown
# Security Review: [Feature Name]

## Unsafe Code
- Location: `src/ffi.rs:42`
- SAFETY comment: Yes/No
- Risk level: Low/Medium/High

## Dependencies
- New dependencies: [list]
- cargo audit: Pass/Fail
- cargo deny: Pass/Fail

## Input Handling
- Validated at boundary: Yes/No
- SQL injection risk: None/Low/Medium/High

## Secrets
- Hardcoded secrets: None found / [list]
- Properly guarded: Yes/No

## Verdict
[ ] Approved
[ ] Approved with changes
[ ] Blocked - requires changes
```

---

## Emergency Response

If critical vulnerability found:
1. Document with detailed report
2. Alert team immediately
3. Provide secure code example
4. Verify fix works
5. Rotate exposed secrets
