---
name: content-engine
invoke: content-engine
description: Content Engine for Rust
user_invocable: true
origin: everything-ai-need
---

# Content Engine for Rust

## When to Activate

- Announcing new Rust releases
- Sharing Rust tips and tricks
- Building community engagement
- Creating announcement threads
- Cross-posting content

---

## Platform Formats

### Twitter/X (Thread Format)

**Constraints:**
- 280 characters per tweet
- 15-20 tweets max for a thread
- Code must fit in tweet

**Example Thread:**
```
🦀 New release: my-crate v1.0.0

A thread on what's new 🧵

1/5

---

✨ Feature: Async support

Now powered by tokio for maximum performance:

```rust
let result = my_crate::process().await?;
```

2/5

---

🔧 Improvement: 50% faster

Benchmarked with criterion:
- Before: 120ms
- After: 60ms

3/5

---

⚠️ Breaking Change

The `init()` function is now async:

OLD: my_crate::init()
NEW: my_crate::init().await

4/5

---

📦 Get it:

cargo add my-crate

Full changelog: github.com/you/my-crate/releases

5/5
```

---

### Reddit

**Constraints:**
- Code blocks supported
- Longer explanations OK
- Include GitHub link

**Example Post:**
```markdown
I built a Rust library for X — here's what I learned

After 3 months of work, I released v1.0 of my-crate. It's a library for doing X in Rust with async support.

Key features:
- Zero allocations in hot path
- Full tokio integration
- Type-safe API

Here's a basic example:

```rust
use my_crate::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await;
    let result = client.process("input").await;
    println!("{}", result);
}
```

The hardest part was implementing the internal state machine. Rust's ownership made it tricky but the compiler caught so many bugs.

GitHub: [link]
Docs: [link]
Crates.io: [link]

Happy to answer questions!
```

---

### Discord

**Constraints:**
- <2000 characters per message
- Code blocks OK
- Use formatting for emphasis

**Example Message:**
```
🎉 **my-crate v1.0.0 is out!**

Highlights:
• Async/await support
• 50% performance improvement
• New `process_batch()` method

Quick start:
```rust
cargo add my-crate
```

```rust
let result = my_crate::process().await?;
```

Check the full changelog: <link>
```

---

## Content Workflow

### 1. Start with Long-form (Blog)

Write comprehensive blog post first:
- Full explanation
- Multiple code examples
- Architecture decisions
- Trade-offs

### 2. Extract Snippets

Pull out 5-10 tweetable moments:
- Key features
- Performance numbers
- Interesting code snippets
- Tips learned

### 3. Adapt for Platforms

| Platform | Adaptation |
|----------|-----------|
| Twitter | Thread, shorten code, add emoji |
| Reddit | Full explanation, keep code blocks |
| Discord | Bullet points, inline code |
| HN | Technical focus, no marketing |

### 4. Schedule Posts

```
Day 1: Twitter thread + Reddit
Day 2: Discord communities
Day 3: Hacker News (if significant)
Day 7: Follow-up tips thread
```

---

## Templates

### Release Announcement

```markdown
🦀 [crate] v[version]

What's new:
- [Feature 1]
- [Feature 2]
- [Performance improvement]

Breaking changes:
- [Change 1]

Upgrade:
cargo add [crate]@[version]

Changelog: [link]
```

### Tip/Trick

```markdown
💡 Rust tip: [topic]

[Code example]

Why this matters:
[1-2 sentence explanation]

[Hashtags]
```

### Behind the Scenes

```markdown
Building [crate] — what I learned

The challenge:
[Problem description]

The solution:
[Approach]

The result:
[Outcome]

[Code snippet]
```

---

## Platform-Specific Tips

### Twitter
- Use emoji for visual breaks
- End tweets with "🧵" for threads
- Tag relevant accounts (@rustlang, @tokio_rs)
- Use hashtags: #rustlang #async

### Reddit
- Post to r/rust for general content
- Post to r/programming for broader reach
- Include benchmarks if claiming performance
- Be ready for detailed questions

### Discord
- Post to relevant channels (#showcase, #releases)
- Use proper code formatting
- Pin important announcements
- Engage with reactions

### Hacker News
- Technical, not marketing
- "Show HN:" prefix for projects
- Be prepared for critical feedback
- Engage in comments

---

## Content Calendar

### Release Week
- Day 0: Blog post, Twitter thread, Reddit
- Day 1: Discord announcement
- Day 2: Reply to questions/comments
- Day 3: Follow-up tip thread
- Day 7: "Thanks for the feedback" post

### Ongoing
- Weekly: Tips and tricks
- Monthly: Behind-the-scenes
- Quarterly: Major updates
