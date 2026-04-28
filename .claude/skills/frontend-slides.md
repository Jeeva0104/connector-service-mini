---
name: frontend-slides
invoke: frontend-slides
description: HTML Presentation Builder
user_invocable: true
origin: everything-ai-need
---

# HTML Presentation Builder

## When to Activate

- Creating Rust conference talks
- Internal tech presentations
- Demo recordings

---

## HTML Slide Structure

```html
<!DOCTYPE html>
<html>
<head>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    * { box-sizing: border-box; margin: 0; }
    .slide {
      width: 100vw; height: 100vh;
      display: flex; flex-direction: column;
      justify-content: center; align-items: center;
      padding: 2rem;
    }
    .slide h1 { font-size: 3rem; }
    .slide code { background: #f4f4f4; padding: 0.2em 0.4em; }
  </style>
</head>
<body>
  <section class="slide">
    <h1>Fearless Concurrency with Rust</h1>
    <p>Zero-cost abstractions for parallel code</p>
  </section>

  <section class="slide">
    <h2>The Problem</h2>
    <pre><code>// Data races in C++</code></pre>
  </section>
</body>
</html>
```

---

## Strict Viewport-Fit Rules

- Use `100vw`/`100vh` for slides
- Never hardcode pixel widths
- Test at multiple aspect ratios

---

## PPTX Conversion

1. Open HTML in browser
2. Print to PDF (Save as PDF)
3. Use `pandoc` or online converter to PPTX
