---
name: blueprint
invoke: blueprint
description: Project Blueprints
user_invocable: true
origin: everything-ai-need
---

# Project Blueprints

## When to Activate

- Starting a new Rust project
- Scaffolding a new module
- Setting up project structure
- Creating consistent project layouts

---

## Web API Blueprint

### Directory Structure

```
my-api/
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── .env.example
├── .github/
│   └── workflows/
│       └── ci.yml
├── migrations/
│   └── 001_initial.sql
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   ├── state.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   └── user_service_impl.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user_repo.rs
│   │   └── user_repo_impl.rs
│   └── models/
│       ├── mod.rs
│       └── user.rs
└── tests/
    └── integration_tests.rs
```

### Cargo.toml

```toml
[package]
name = "my-api"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
tower-http = { version = "0.5", features = ["trace", "cors"] }
tracing = "0.1"
tracing-subscriber = "0.3"
config = "0.14"
validator = { version = "0.16", features = ["derive"] }

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

### Quick Scaffolding Commands

```bash
cargo new my-api --bin
cd my-api

# Add dependencies
cargo add tokio axum sqlx serde serde_json thiserror anyhow

# Create directory structure
mkdir -p src/{handlers,services,repositories,models}
mkdir -p migrations tests .github/workflows

# Create files
touch src/{handlers,services,repositories,models}/mod.rs
touch src/{config,error,state}.rs
```

---

## CLI Tool Blueprint

### Directory Structure

```
my-cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   └── process.rs
│   └── config.rs
└── tests/
    └── cli_tests.rs
```

### Cargo.toml

```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mycli"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

### CLI Structure

```rust
// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "A CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init {
        #[arg(default_value = ".")]
        path: String,
    },
    /// Process files
    Process {
        #[arg(required = true)]
        files: Vec<String>,
    },
}
```

---

## Async Service Blueprint

### Directory Structure

```
my-service/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── worker.rs
│   ├── queue.rs
│   └── processor.rs
└── tests/
    └── worker_tests.rs
```

### Worker Pattern

```rust
// src/worker.rs
use tokio::sync::mpsc;
use std::sync::Arc;

pub struct Worker {
    rx: mpsc::Receiver<Job>,
    processor: Arc<dyn Processor>,
}

impl Worker {
    pub fn new(rx: mpsc::Receiver<Job>, processor: Arc<dyn Processor>) -> Self {
        Self { rx, processor }
    }

    pub async fn run(mut self) {
        while let Some(job) = self.rx.recv().await {
            match self.processor.process(job).await {
                Ok(_) => tracing::info!("Job completed"),
                Err(e) => tracing::error!("Job failed: {}", e),
            }
        }
    }
}

#[async_trait::async_trait]
pub trait Processor: Send + Sync {
    async fn process(&self, job: Job) -> Result<(), Error>;
}
```

---

## Library Blueprint

### Directory Structure

```
my-lib/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types.rs
│   ├── parser.rs
│   └── utils.rs
├── examples/
│   ├── basic.rs
│   └── advanced.rs
├── benches/
│   └── benchmark.rs
└── tests/
    └── integration_tests.rs
```

### Cargo.toml

```toml
[package]
name = "my-lib"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A useful library"
license = "MIT OR Apache-2.0"
repository = "https://github.com/you/my-lib"

[dependencies]

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmark"
harness = false
```

---

## Workspace Blueprint

### Directory Structure

```
my-project/
├── Cargo.toml          # Workspace root
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── api/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── cli/
│       ├── Cargo.toml
│       └── src/main.rs
└── shared/
    ├── Cargo.toml
    └── src/lib.rs
```

### Root Cargo.toml

```toml
[workspace]
members = ["crates/*", "shared"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
thiserror = "1"
anyhow = "1"
tracing = "0.1"

# Internal crates
my-core = { path = "crates/core" }
my-shared = { path = "shared" }
```

---

## Best Practices

1. **Consistent naming**: Use `snake_case` for files and modules
2. **Module structure**: Each directory has `mod.rs`
3. **Separate concerns**: Handler → Service → Repository
4. **Error types**: Central `error.rs` with `thiserror`
5. **Config**: Use `config` crate for environment-based config
6. **Tests**: Co-locate unit tests, separate integration tests
7. **Documentation**: README with setup instructions
