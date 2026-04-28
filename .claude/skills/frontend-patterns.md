---
name: frontend-patterns
invoke: frontend-patterns
description: Frontend Patterns for Rust Web Apps
user_invocable: true
origin: everything-ai-need
---

# Frontend Patterns for Rust Web Apps

## When to Activate

- Building Rust frontend with Leptos/Yew
- Integrating Rust WASM with JavaScript
- Setting up full-stack Rust applications
- Creating reactive UIs in Rust

---

## Leptos (Reactive Rust Framework)

### Basic Component

```rust
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </button>
        </div>
    }
}
```

### Signal-Based Reactivity

```rust
#[component]
fn UserProfile() -> impl IntoView {
    let (user, set_user) = create_signal(User::default());
    let (loading, set_loading) = create_signal(false);

    let fetch_user = move |_| {
        set_loading.set(true);
        spawn_local(async move {
            let result = fetch_user_api().await;
            set_user.set(result);
            set_loading.set(false);
        });
    };

    view! {
        <div>
            <button on:click=fetch_user>"Load User"</button>
            {move || if loading.get() {
                view! { <p>"Loading..."</p> }
            } else {
                view! { <p>{user.get().name}</p> }
            }}
        </div>
    }
}
```

### Router

```rust
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/users">"Users"</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/users" view=UserList/>
                    <Route path="/users/:id" view=UserDetail/>
                </Routes>
            </main>
        </Router>
    }
}
```

---

## Yew (Component-Based Framework)

### Function Component

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello from Yew!" }</h1>
        </div>
    }
}
```

### Stateful Component

```rust
use yew::prelude::*;

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <button {onclick}>{ "Increment" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
```

### Properties

```rust
#[derive(Properties, PartialEq)]
pub struct UserCardProps {
    pub name: String,
    pub email: String,
    #[prop_or_default]
    pub avatar: Option<String>,
}

#[function_component(UserCard)]
pub fn user_card(props: &UserCardProps) -> Html {
    html! {
        <div class="user-card">
            <h3>{ &props.name }</h3>
            <p>{ &props.email }</p>
        </div>
    }
}
```

---

## WASM Integration

### Expose Rust to JavaScript

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator { value: 0.0 }
    }

    pub fn add(&mut self, n: f64) {
        self.value += n;
    }

    pub fn result(&self) -> f64 {
        self.value
    }
}
```

### JavaScript Usage

```javascript
import init, { process_data, Calculator } from './pkg/my_wasm.js';

async function run() {
    await init();

    // Simple function
    console.log(process_data("hello")); // "HELLO"

    // Class
    const calc = new Calculator();
    calc.add(5);
    calc.add(3);
    console.log(calc.result()); // 8
}

run();
```

---

## Full-Stack Rust Architecture

```
my-project/
├── backend/          # Axum/Actix API
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── routes/
├── frontend/         # Leptos/Yew app
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── shared/           # Common types (serde)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── Cargo.toml        # Workspace root
```

### Workspace Configuration

```toml
# Cargo.toml (root)
[workspace]
members = ["backend", "frontend", "shared"]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

### Shared Types

```rust
// shared/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
```

---

## Best Practices

- Keep components small and focused
- Use signals for fine-grained reactivity
- Share types between frontend and backend
- Minimize JS interop (prefer pure Rust)
- Use `trunk` for Leptos/Yew development
- Test WASM with `wasm-pack test`

---

## Trunk Build Tool

```bash
# Install
cargo install trunk

# Development server
trunk serve

# Production build
trunk build --release
```

```toml
# Trunk.toml
[build]
target = "index.html"

[serve]
port = 8080
```
