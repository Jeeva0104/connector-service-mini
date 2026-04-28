---
name: tdd-workflow
invoke: tdd-workflow
description: TDD Workflow
user_invocable: true
origin: everything-ai-need
---

# TDD Workflow

## When to Activate

- Starting any new feature
- Fixing bugs (write test that reproduces bug first)
- Refactoring existing code
- Reviewing test coverage

---

## The Cycle: Red → Green → Refactor

### Step 1 — RED: Write a Failing Test

```rust
#[test]
fn parse_valid_email_succeeds() {
    let result = Email::parse("alice@example.com");
    assert!(result.is_ok());
}
```

**Key points:**
- Test describes expected behavior
- Test name is descriptive: `{subject}_{condition}_{expected}`
- Run test to confirm it FAILS

### Step 2 — GREEN: Minimal Implementation

```rust
impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::InvalidFormat)
        }
    }
}
```

**Key points:**
- Write just enough code to pass the test
- Don't worry about elegance yet
- All tests should pass

### Step 3 — REFACTOR: Improve with Tests Green

```rust
impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        let s = s.trim();
        let parts: Vec<&str> = s.splitn(2, '@').collect();

        match parts.as_slice() {
            [local, domain] if !local.is_empty() && domain.contains('.') => {
                Ok(Email(s.to_string()))
            }
            _ => Err(EmailError::InvalidFormat),
        }
    }
}
```

**Key points:**
- Improve code quality
- Remove duplication
- Tests must stay green

---

## Language-Specific Practices

### Rust
- Use `cargo test` for all tests
- Co-locate unit tests in `#[cfg(test)]` modules
- Use `mockall` for mocking traits
- `#[tokio::test]` for async tests

### Python
- Use `pytest` framework
- Tests in `tests/` directory or `test_*.py` files
- Use `unittest.mock` or `pytest-mock`
- Fixtures for shared setup

### TypeScript/JavaScript
- Use `jest` or `vitest`
- Tests in `*.test.ts` or `__tests__/` directory
- Mock with `jest.mock()` or `vi.mock()`

---

## Test Naming Convention

```
{subject}_{condition}_{expected_result}

Examples:
- parse_valid_email_returns_ok
- parse_empty_string_returns_invalid_format_error
- user_service_create_duplicate_email_returns_validation_error
- transfer_insufficient_funds_returns_balance_error
```

---

## Bug Fix TDD Flow

1. **Write test that reproduces the bug**
```rust
#[test]
fn parse_email_with_leading_spaces_succeeds() {
    // Bug: "  alice@example.com" was failing
    assert!(Email::parse("  alice@example.com").is_ok());
}
```

2. **Run test → RED** (confirms bug exists)

3. **Fix the bug**

4. **Run test → GREEN**

5. **Add edge case tests**

---

## Watch Mode

```bash
# Rust
cargo watch -x test

# Python
pytest-watch

# TypeScript
jest --watch
```

---

## Anti-Patterns to Avoid

- ❌ Writing implementation before test
- ❌ Testing implementation details (internal state)
- ❌ Tests depending on each other (shared state)
- ❌ Asserting too little (passing tests that don't verify)
- ❌ Skipping test runs before committing

---

## Commit Checklist

- [ ] `cargo test` / `pytest` / `jest` passes
- [ ] New code has corresponding tests
- [ ] Bug fix has regression test
- [ ] Tests describe behavior not implementation
- [ ] All edge cases covered
