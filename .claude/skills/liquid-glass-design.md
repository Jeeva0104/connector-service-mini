---
name: liquid-glass-design
invoke: liquid-glass-design
description: UI Design System
user_invocable: true
origin: everything-ai-need
---

# UI Design System

## When to Activate

- Building Rust web app UI
- Creating component libraries
- Setting up design tokens

---

## Design Tokens

```rust
// Design system constants
pub mod tokens {
    pub const PRIMARY: &str = "#e11d48";
    pub const SECONDARY: &str = "#4f46e5";
    pub const SPACING_UNIT: u16 = 8;
    pub const RADIUS: &str = "8px";
}
```

---

## Component Pattern

```rust
use leptos::*;

#[component]
pub fn Button(
    #[prop(into)] children: Children,
    #[prop(default = false)] primary: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class = if primary { "btn-primary" } else { "btn-secondary" };

    view! {
        <button class={class} on:click=move |_| on_click.map(|cb| cb.call(()))>
            {children()}
        </button>
    }
}
```

---

## CSS-in-Rust Pattern

```rust
// Using stylers or similar
#[styled_component(Card)]
pub fn card() -> Html {
    let style = css!("
        background: white;
        border-radius: 8px;
        padding: 16px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    ");

    html! { <div class={style}>{ "Content" }</div> }
}
```
