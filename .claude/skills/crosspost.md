---
name: crosspost
invoke: crosspost
description: Crosspost
user_invocable: true
origin: everything-ai-need
---

# Crosspost

## When to Activate

- Announcing crate releases
- Sharing Rust milestones
- Community updates
- Publishing new versions

---

## Release Checklist

### Pre-release
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG.md updated
- [ ] README.md current
- [ ] Documentation builds (`cargo doc`)
- [ ] All tests pass (`cargo test`)
- [ ] Clippy clean (`cargo clippy`)
- [ ] `cargo publish --dry-run` passes

### Publish
- [ ] `cargo publish`
- [ ] Git tag pushed (`git tag v1.0.0 && git push origin v1.0.0`)
- [ ] GitHub Release created with notes

### Announce
- [ ] Twitter/X thread posted
- [ ] Reddit r/rust post
- [ ] Discord (Rust Community, Tokio, etc.)
- [ ] This Week in Rust (if significant)
- [ ] Internal team chat

---

## Platform Checklist

### crates.io
- [ ] Published successfully
- [ ] Keywords set in Cargo.toml
- [ ] Categories set
- [ ] Documentation link works
- [ ] Repository link works
- [ ] Description clear and concise

### GitHub Releases
- [ ] Release notes written
- [ ] Breaking changes highlighted
- [ ] Migration guide linked (if needed)
- [ ] Assets attached (if applicable)

### Reddit r/rust
- [ ] Title includes [crate] and version
- [ ] Post includes code example
- [ ] Links to crates.io and docs
- [ ] Explanation of what it does
- [ ] Responding to comments

### Twitter/X
- [ ] Thread planned (5-10 tweets)
- [ ] Emoji added for visual breaks
- [ ] Code snippets fit in tweets
- [ ] Links shortened
- [ ] Relevant accounts tagged

### Discord
- [ ] Posted to appropriate channels
- [ ] Code formatted with syntax highlighting
- [ ] Links verified

---

## Release Notes Template

```markdown
## [crate] v1.0.0

### What's New
- Feature A - description
- Feature B - description
- Performance improvement - numbers

### Breaking Changes
- API change X (migration: ...)
- Removed deprecated function Y

### Migration Guide
```rust
// Before
old_function();

// After
new_function().await?;
```

### Contributors
Thanks to @user1, @user2 for their contributions!

### Links
- crates.io: https://crates.io/crates/[crate]
- Docs: https://docs.rs/[crate]/1.0.0
- Changelog: https://github.com/[user]/[crate]/blob/main/CHANGELOG.md
```

---

## Timing Strategy

### Best Times to Post

| Platform | Best Time (UTC) |
|----------|----------------|
| Twitter | 13:00 - 15:00 |
| Reddit | 14:00 - 16:00 |
| HN | 12:00 - 14:00 |
| Discord | Any time |

### Posting Order
1. Publish to crates.io
2. Create GitHub release
3. Wait 10 minutes for docs.rs
4. Post Twitter thread
5. Post Reddit
6. Post Discord

---

## Response Management

### Twitter
- Respond to questions within 2 hours
- Like positive mentions
- Quote tweet with corrections/clarifications

### Reddit
- Respond to all top-level comments
- Edit post with FAQ if same questions repeat
- Be humble and open to feedback

### Discord
- React with emoji to show engagement
- Answer questions promptly
- Pin announcement if allowed

---

## Tracking

Create a simple tracker:

```markdown
# Release: v1.0.0

## Pre-release
- [ ] Tests pass
- [ ] Docs build
- [ ] CHANGELOG updated

## Publish
- [ ] crates.io
- [ ] GitHub tag
- [ ] GitHub release

## Announce
- [ ] Twitter
- [ ] Reddit
- [ ] Discord

## Engagement
- [ ] Twitter: 10+ likes
- [ ] Reddit: 5+ upvotes
- [ ] Discord: 5+ reactions
```
