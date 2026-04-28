---
name: rust
invoke: rust
description: Rust Development Guide
user_invocable: true
origin: everything-ai-need
---

# Rust Development Guide

Complete guide for Rust development including language fundamentals, web development, databases, testing, and deployment.

---

## Core Language Patterns

### Ownership & Borrowing

```rust
// Move semantics
let s1 = String::from("hello");
let s2 = s1; // s1 moved, no longer valid

// Clone when you need both
let s3 = s1.clone();

// References & Borrowing Rules
fn calculate_length(s: &String) -> usize { s.len() }
fn change(s: &mut String) { s.push_str(", world"); }
```

### Copy vs Clone

```rust
// Copy types (stack-only): i32, f64, bool, char, tuples of Copy types
let x = 5;
let y = x; // copied, x still valid

// Non-Copy types need explicit .clone()
let v1 = vec![1, 2, 3];
let v2 = v1.clone();
```

### Traits

```rust
pub trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> String;
    fn describe(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

// impl Trait vs dyn Trait vs Generic Bounds
fn make_sound<T: Animal>(animal: &T) { ... }  // Generic (preferred for performance)
fn make_sound(animal: &impl Animal) { ... }   // impl Trait
fn make_sounds(animals: &[Box<dyn Animal>]) { ... }  // dyn Trait (runtime polymorphism)

// Object Safety Rules - A trait is object-safe when:
// - No methods return `Self`
// - No generic type parameters on methods

// Blanket Implementations
impl<T: Display> Printable for T {}

// Standard traits to implement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// From/Into - type conversions
impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point { x, y }
    }
}
let p: Point = (1.0, 2.0).into();

// Builder Pattern with Traits
pub trait Builder {
    type Output;
    fn build(self) -> Result<Self::Output, BuildError>;
}
```

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

// Use ? operator for propagation
pub async fn get_user(id: u64) -> Result<User, AppError> {
    let user = db.find_user(id).await?;
    user.ok_or_else(|| AppError::NotFound(format!("user {id}")))
}

// Option Combinators
let upper = name.map(|n| n.to_uppercase());
let len = name.as_ref().map(|n| n.len()).unwrap_or(0);
let result = name.ok_or(AppError::NotFound("name".into()))?;
let fallback = name.unwrap_or_else(|| "anonymous".to_string());
```

### Pattern Matching

```rust
match user.role {
    Role::Admin => grant_all_access(),
    Role::Editor => grant_edit_access(),
    Role::Viewer | Role::Guest => grant_read_access(),
}

// if let for single variant
if let Some(user) = find_user(id) { process(user); }

// while let for iterative unwrapping
while let Some(task) = queue.pop() { execute(task); }

// Destructuring structs
let User { name, email, .. } = user;
```

### Lifetimes

```rust
// Explicit lifetime when returning reference tied to input
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct holding references
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}
```

---

## Async Rust

### Tokio Runtime Setup

```rust
// Binary entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // async code here
    Ok(())
}

// Custom runtime (for fine-tuned control)
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}
```

### async fn and .await

```rust
// Declare async functions
async fn fetch_user(id: u64) -> Result<User, AppError> {
    let user = db.find(id).await?;
    user.ok_or(AppError::NotFound(format!("user {id}")))
}

// BAD - sequential (waits for each before starting next)
let a = fetch_a().await?;
let b = fetch_b().await?;

// GOOD - concurrent
let (a, b) = tokio::join!(fetch_a(), fetch_b());
```

### Spawning Tasks

```rust
// Spawn independent task (fire and forget)
let handle = tokio::spawn(async move {
    process_data(data).await
});

// Await the result
let result = handle.await??; // first ? for JoinError, second for task error

// Spawn multiple and collect
let handles: Vec<_> = items
    .into_iter()
    .map(|item| tokio::spawn(async move { process(item).await }))
    .collect();

let results: Vec<_> = futures::future::join_all(handles).await;
```

### Channels

```rust
use tokio::sync::{mpsc, oneshot, broadcast, watch};

// mpsc - multiple producers, single consumer
let (tx, mut rx) = mpsc::channel::<Message>(100);
tokio::spawn(async move { tx.send(msg).await.unwrap(); });
while let Some(msg) = rx.recv().await { process(msg); }

// oneshot - single value, one-shot response
let (tx, rx) = oneshot::channel::<Response>();
tx.send(response).unwrap();
let response = rx.await.unwrap();

// broadcast - multiple consumers
let (tx, _) = broadcast::channel::<Event>(16);
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();
tx.send(event).unwrap();

// watch - latest value only
let (tx, rx) = watch::channel(initial_value);
tx.send(new_value).unwrap();
let current = rx.borrow().clone();
```

### tokio::select! - Concurrent Branches

```rust
tokio::select! {
    result = fetch_primary() => {
        handle_result(result)
    }
    result = fetch_fallback() => {
        handle_result(result)
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        Err(AppError::Timeout)
    }
}
```

### Timeouts

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(30),
    fetch_external_api(),
)
.await
.map_err(|_| AppError::Timeout)?;
```

### spawn_blocking - CPU-Bound Work

Never block the async executor with CPU-intensive work:

```rust
// BAD - blocks the executor thread
async fn compute() {
    let result = heavy_computation(); // blocks!
}

// GOOD - offload to thread pool
async fn compute() -> Result<Output, AppError> {
    tokio::task::spawn_blocking(|| {
        heavy_computation()
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}
```

### Shared State

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

// Arc<Mutex<T>> for shared mutable state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub cache: Arc<Mutex<HashMap<String, Value>>>,
}

// Prefer Arc<RwLock<T>> when reads far outnumber writes
let data = Arc::new(RwLock::new(HashMap::new()));
let read = data.read().await;
let mut write = data.write().await;
```

### Common Async Pitfalls

```rust
// PITFALL 1: Holding a Mutex guard across .await
// BAD
let guard = mutex.lock().await;
some_async_fn().await; // guard held across await = potential deadlock
drop(guard); // too late

// GOOD
{
    let guard = mutex.lock().await;
    let value = guard.clone();
} // guard dropped before await
some_async_fn_with(value).await;

// PITFALL 2: Using std::sync::Mutex in async context
// Use tokio::sync::Mutex for async code

// PITFALL 3: Not using .await on futures (silently dropped)
// BAD
fetch_data(); // future created but never awaited
// GOOD
fetch_data().await;
```

---

## Backend Architecture Patterns

### AppState - Shared Application State

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub user_service: Arc<dyn UserService>,
    pub email_service: Arc<dyn EmailService>,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let pool = create_pool(&config.database_url).await?;
        let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
        let email_svc = Arc::new(SmtpEmailService::new(&config.smtp));
        let user_svc = Arc::new(UserServiceImpl::new(user_repo, email_svc.clone()));

        Ok(Self {
            db: pool,
            user_service: user_svc,
            email_service: email_svc,
            config: Arc::new(config),
        })
    }
}
```

### Layered Architecture

```
Handler -> Service -> Repository -> Database
  (HTTP)  (business)  (data)
```

All layers communicate through traits:

```rust
// Layer 1: Handler (HTTP only, no business logic)
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.register(req).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

// Layer 2: Service (business logic)
#[async_trait]
pub trait UserService: Send + Sync {
    async fn register(&self, req: RegisterRequest) -> Result<User, AppError>;
}

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
    email: Arc<dyn EmailService>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn register(&self, req: RegisterRequest) -> Result<User, AppError> {
        req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Validation("email taken".into()));
        }
        let hash = hash_password(&req.password)?;
        let user = self.repo.insert(&req.email, &req.name, &hash).await?;
        self.email.send_welcome(&user.email).await?;
        Ok(user)
    }
}

// Layer 3: Repository (data access only)
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn insert(&self, email: &str, name: &str, hash: &str) -> Result<User, AppError>;
}
```

### Dependency Injection via Traits

```rust
// Test with mock implementations
#[cfg(test)]
mod tests {
    use mockall::mock;
    use super::*;

    mock! {
        UserRepo {}
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
            async fn insert(&self, email: &str, name: &str, hash: &str) -> Result<User, AppError>;
        }
    }

    #[tokio::test]
    async fn register_duplicate_email_returns_error() {
        let mut mock = MockUserRepo::new();
        mock.expect_find_by_email()
            .returning(|_| Ok(Some(existing_user())));

        let svc = UserServiceImpl::new(Arc::new(mock), mock_email());
        let result = svc.register(register_request()).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }
}
```

---

## Web Development (Axum)

### Router & Handlers

```rust
use axum::{Router, routing::{get, post, delete}};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .route("/users/:id/orders", get(list_user_orders))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}
```

### Axum Extractors

```rust
use axum::{
    extract::{Path, Query, State, Json},
    response::{IntoResponse, Response},
    http::StatusCode,
};

// Thin handler - delegates all logic to service
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.get(id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = state.user_service.list(params).await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}
```

### Error Handling

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into()),
        };
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}
```

### Actix-web Patterns

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Result};

pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    let user = state.user_service.create(body.into_inner()).await
        .map_err(|e| actix_web::error::ErrorUnprocessableEntity(e))?;
    Ok(HttpResponse::Created().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

### Middleware

```rust
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
    auth::RequireAuthorizationLayer,
};

let app = Router::new()
    .merge(public_routes())
    .merge(
        private_routes()
            .layer(RequireAuthorizationLayer::bearer("secret-token"))
    )
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .layer(CorsLayer::permissive());
```

---

## API Design

### RESTful Resource Naming

```rust
// GOOD - resource-based
Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
    .route("/users/:id/orders", get(list_user_orders))

// BAD - action-based (RPC style over HTTP)
Router::new()
    .route("/getUser", get(get_user))
    .route("/createUser", post(create_user))
```

### Versioning Strategies

```rust
// URL Path Versioning (Recommended)
Router::new()
    .nest("/api/v1", v1_routes())
    .nest("/api/v2", v2_routes())

fn v1_routes() -> Router {
    Router::new()
        .route("/users", get(v1_list_users))
}

// Header Versioning (Alternative)
async fn api_handler(
    headers: HeaderMap,
) -> Response {
    match headers.get("API-Version") {
        Some(v) if v == "v2" => v2_handler().await,
        _ => v1_handler().await,
    }
}
```

### Request/Response Types

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

// Request with validation
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[serde(default)]
    pub role: Option<Role>,
}

// Response with serialization
#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub created_at: String,
}

// Pagination
#[derive(Deserialize, Validate)]
pub struct ListParams {
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 { 20 }

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
```

### OpenAPI with utoipa

```toml
[dependencies]
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
```

```rust
use utoipa::OpenApi;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_user,
        list_users,
    ),
    components(
        schemas(CreateUserRequest, UserResponse, AppError)
    ),
    tags(
        (name = "users", description = "User management")
    )
)]
struct ApiDoc;

// Add route annotations
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
    )
)]
pub async fn create_user(...) -> Result<..., AppError> { ... }

// Serve Swagger UI
let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
```

### Error Response Format

```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorDetail { code: "NOT_FOUND".into(), message: msg, field: None }
            ),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorDetail { code: "VALIDATION_ERROR".into(), message: msg, field: None }
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail { code: "INTERNAL_ERROR".into(), message: "internal server error".into(), field: None }
            ),
        };
        (status, Json(ErrorResponse { error })).into_response()
    }
}
```

### Request Validation

```rust
use validator::Validate;
use axum_valid::Valid;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 100))]
    pub name: String,
}

// Use Valid<Json<T>> extractor - auto-validates
pub async fn create_user(
    Valid(Json(payload)): Valid<Json<CreateUserRequest>>,
) -> Result<impl IntoResponse, AppError> { ... }
```

---

## Database Access

### SQLx - Compile-Time Verified Queries

```toml
[dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "uuid", "chrono"] }
```

```rust
// query_as! - compile-time verified, maps to struct
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, email, name, created_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert(pool: &PgPool, email: &str, name: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (email, name) VALUES ($1, $2) RETURNING id, email, name, created_at",
        email,
        name
    )
    .fetch_one(pool)
    .await
}
```

### Connection Pool Setup

```rust
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .after_connect(|conn, _meta| Box::pin(async move {
            conn.execute("SET TIME ZONE 'UTC'").await?;
            Ok(())
        }))
        .connect(database_url)
        .await
}
```

### Repository Trait Pattern

```rust
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn insert(&self, req: &NewUser) -> Result<User, AppError>;
    async fn update(&self, id: i64, req: &UpdateUser) -> Result<User, AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)
    }
    // ...
}
```

### Transactions

```rust
pub async fn transfer(pool: &PgPool, from: i64, to: i64, amount: Decimal) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amount, from)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amount, to)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;
    Ok(())
}
```

### Diesel ORM

```toml
[dependencies]
diesel = { version = "2.1", features = ["postgres", "r2d2", "chrono"] }
```

```rust
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User { pub id: i32, pub email: String, pub name: String }

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> { pub email: &'a str, pub name: &'a str }

// Query
pub fn find_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
    users::table.find(user_id).first(conn)
}

// Insert
pub fn create_user(conn: &mut PgConnection, email: &str, name: &str) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(NewUser { email, name })
        .get_result(conn)
}
```

### PostgreSQL Patterns

```rust
// JSONB Handling
use serde_json::Value;

#[derive(sqlx::FromRow)]
struct Event {
    id: i64,
    data: Value,  // JSONB
}

sqlx::query!(
    "INSERT INTO events (data) VALUES ($1)",
    serde_json::json!({"user_id": 1, "action": "login"})
)
.execute(&pool)
.await?;

// Query JSONB
sqlx::query_as!(
    Event,
    "SELECT * FROM events WHERE data->>'action' = $1",
    "login"
)
.fetch_all(&pool)
.await?;
```

```sql
-- Full-Text Search Migration
CREATE INDEX idx_users_search ON users
USING gin (to_tsvector('english', name || ' ' || email));
```

```rust
// Full-Text Search
pub async fn search_users(pool: &PgPool, query: &str) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE to_tsvector('english', name || ' ' || email) @@ plainto_tsquery('english', $1)
        ORDER BY ts_rank(to_tsvector('english', name || ' ' || email), plainto_tsquery('english', $1)) DESC
        LIMIT 20
        "#,
        query
    )
    .fetch_all(pool)
    .await
}
```

```rust
// Listen/Notify
use sqlx::postgres::PgListener;

let mut listener = PgListener::connect_with(&pool).await?;
listener.listen("user_events").await?;

loop {
    let notification = listener.recv().await?;
    println!("Channel: {}", notification.channel());
    println!("Payload: {}", notification.payload());
}
```

```rust
// Query Optimization with EXPLAIN ANALYZE
let plan: String = sqlx::query_scalar(
    "EXPLAIN (ANALYZE, FORMAT JSON) SELECT * FROM users WHERE email = $1"
)
.bind(email)
.fetch_one(&pool)
.await?;
```

### Database Migrations

```bash
# Install sqlx CLI
cargo install sqlx-cli --features native-tls,postgres

# Create a migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check status
sqlx migrate info
```

```
migrations/
├── 20240101000001_create_users_table.sql
└── 20240102000000_add_user_indexes.sql
```

```sql
-- 20240101000001_create_users_table.sql
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Down migration (for revert)
-- DROP TABLE IF EXISTS users;
```

```rust
// Embedding Migrations in Binary
sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to run migrations");
```

### ClickHouse Analytics

```toml
[dependencies]
clickhouse = "0.11"
```

```rust
use clickhouse::{Client, Row};

let client = Client::default()
    .with_url("http://localhost:8123")
    .with_database("analytics")
    .with_user("default")
    .with_password("password");

// Define Schema
#[derive(Row, Serialize, Deserialize)]
struct Event {
    #[serde(with = "clickhouse::serde::time::datetime64::nanos")]
    timestamp: OffsetDateTime,
    user_id: u64,
    event_type: String,
    properties: String,
}

// Batch Inserts
let mut inserter = client.inserter("events")?
    .with_max_entries(100_000)
    .with_max_duration(Duration::from_secs(5));

for event in events {
    inserter.write(&event).await?;
}
inserter.end().await?;

// Querying
let mut cursor = client
    .query("SELECT ?fields FROM events WHERE event_type = ?")
    .bind("click")
    .fetch::<Event>()?;

while let Some(event) = cursor.next().await? {
    println!("{:?}", event);
}
```

---

## Testing

### Unit Tests

Co-locate unit tests with the code they test:

```rust
// src/services/user_service.rs
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_passes() {
        assert!(validate_email("alice@example.com"));
    }

    #[test]
    fn email_without_at_fails() {
        assert!(!validate_email("notanemail.com"));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn panics_on_bad_index() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }
}
```

### Integration Tests

Place in `tests/` directory - each file is a separate test binary:

```rust
// tests/user_api.rs
use my_crate::{AppState, create_app};

#[tokio::test]
async fn test_create_user() {
    let state = AppState::test().await;
    let app = create_app(state);

    let response = app
        .oneshot(Request::post("/users").body(Body::from(
            r#"{"email":"test@example.com","name":"Test"}"#,
        )).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

### Async Tests

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = fetch_data().await;
    assert!(result.is_ok());
}

// With custom runtime config
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_concurrent() { ... }
```

### Mocking with mockall

```rust
use mockall::{automock, predicate::*};

#[automock]
pub trait UserRepository {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, DbError>;
    async fn insert(&self, user: &NewUser) -> Result<User, DbError>;
}

#[tokio::test]
async fn test_user_service_get() {
    let mut mock_repo = MockUserRepository::new();
    mock_repo
        .expect_find_by_id()
        .with(eq(1u64))
        .times(1)
        .returning(|_| Ok(Some(User { id: 1, name: "Alice".into() })));

    let service = UserService::new(Arc::new(mock_repo));
    let user = service.get(1).await.unwrap();
    assert_eq!(user.name, "Alice");
}
```

### Property-Based Testing with proptest

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_doesnt_crash(s in "\\PC*") {
        let _ = parse_input(&s);
    }

    #[test]
    fn encode_decode_roundtrip(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let encoded = encode(&data);
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(data, decoded);
    }
}
```

### Benchmarks with criterion

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_sort(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).rev().collect();

    c.bench_function("sort 1000 items", |b| {
        b.iter(|| {
            let mut d = black_box(data.clone());
            d.sort();
        })
    });
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);
```

Run: `cargo bench`

### Test Coverage with cargo-tarpaulin

```bash
# Install
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage/

# CI threshold
cargo tarpaulin --fail-under 80
```

### Cargo Test Flags

```bash
cargo test                          # run all tests
cargo test --lib                    # unit tests only
cargo test --test integration_test  # specific integration test file
cargo test user                     # filter by name
cargo test -- --nocapture           # show println! output
cargo test -- --test-threads=1      # run single-threaded (useful for DB tests)
cargo test -- --ignored             # run #[ignore]d tests
```

### Test Helpers & Fixtures

```rust
// tests/helpers/mod.rs
pub struct TestDb {
    pub pool: PgPool,
    pub container: DockerContainer,
}

impl TestDb {
    pub async fn new() -> Self {
        // spin up test postgres
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // cleanup
    }
}
```

---

## Security

### unsafe Code Rules

Every `unsafe` block MUST have a comment explaining why it is safe:

```rust
// SAFETY: `ptr` is guaranteed non-null by the caller contract documented
// in the public API. The lifetime 'a ensures the reference is valid.
let value = unsafe { &*ptr };
```

**unsafe checklist before merging:**
- [ ] Is the invariant documented in a `// SAFETY:` comment?
- [ ] Are all pointer dereferences bounds-checked?
- [ ] Are all lifetimes correct?
- [ ] Could this be written in safe Rust instead?
- [ ] Has a second reviewer approved the unsafe block?

### cargo audit - Required in CI

```bash
# Install
cargo install cargo-audit

# Run audit
cargo audit

# CI configuration (.github/workflows/security.yml)
- name: Security audit
  run: cargo audit --deny warnings
```

### RUSTSEC Policy

- Check https://rustsec.org/ for advisories
- RUSTSEC advisories with severity `high` or `critical` must be resolved within 48 hours
- Use `cargo audit fix` for automatic patching when available
- Pin dependency versions if advisory has no fix yet

### cargo-deny - Dependency Policy

```toml
# deny.toml
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]
deny = ["GPL-3.0"]

[bans]
multiple-versions = "warn"
```

```bash
cargo install cargo-deny
cargo deny check
```

### Input Validation

```rust
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(length(min = 8), custom = "validate_password_strength")]
    pub password: String,
}

// Always validate at system boundaries
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<Response, AppError> {
    payload.validate().map_err(AppError::Validation)?;
    // ...
}
```

### No unwrap() in Production

```rust
// BAD
let value = some_option.unwrap();
let conn = pool.get().unwrap();

// GOOD
let value = some_option.ok_or(AppError::NotFound("value".into()))?;
let conn = pool.get().map_err(AppError::Database)?;

// In tests only - use expect with descriptive message
let value = some_option.expect("test fixture should always have value");
```

### Secrets Management

```rust
use secrecy::{ExposeSecret, Secret};

pub struct Config {
    pub database_url: Secret<String>,
    pub api_key: Secret<String>,
}

// Only expose when needed
let url = config.database_url.expose_secret();
```

- Never log secret values - use `secrecy` crate
- Load secrets from environment, never hardcode
- Use `.env` only for local dev, never commit it

### SQL Injection Prevention

```rust
// SAFE - parameterized queries with sqlx
sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
    .fetch_optional(&pool)
    .await?;

// NEVER string interpolate into queries
// let query = format!("SELECT * FROM users WHERE email = '{}'", email); // UNSAFE
```

---

## Deployment

### Multi-Stage Docker Build (Production)

```dockerfile
# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build application
COPY src ./src
RUN cargo build --release

# Runtime stage (distroless)
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/myapp /myapp

USER nonroot:nonroot

EXPOSE 8080

ENTRYPOINT ["/myapp"]
```

### Docker Compose for Development

```yaml
# docker-compose.yml
version: "3.8"

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.dev
    volumes:
      - .:/app
      - cargo-cache:/usr/local/cargo/registry
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db:5432/myapp
      - RUST_LOG=debug
    depends_on:
      - db

  db:
    image: postgres:16-alpine
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  cargo-cache:
  postgres_data:
```

### Release Profile Optimization

```toml
# Cargo.toml
[profile.release]
opt-level = 3          # max optimization
lto = "thin"           # link-time optimization (thin is faster, full is smaller)
strip = true           # strip symbols
panic = "abort"        # smaller binary, no unwinding
codegen-units = 1      # slower compile, better optimization
```

### Cross-Compilation

```bash
# Using cross (Docker-based)
cargo install cross
cross build --release --target x86_64-unknown-linux-musl

# Using cargo-zigbuild (faster, no Docker)
cargo install cargo-zigbuild
cargo zigbuild --release --target x86_64-unknown-linux-musl

# Static Binary for Alpine
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS='-C target-feature=+crt-static' \
  cargo build --release --target x86_64-unknown-linux-musl
```

### GitHub Actions Deployment

```yaml
# .github/workflows/deploy.yml
name: Build and Deploy

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Build release
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: myapp
          path: target/release/myapp

  docker:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4

      - name: Docker Login
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}

      - name: Docker Build and Push
        run: |
          docker build -t ${{ secrets.DOCKER_USERNAME }}/myapp:${{ github.sha }} .
          docker push ${{ secrets.DOCKER_USERNAME }}/myapp:${{ github.sha }}
```

### Binary Size Optimization

```bash
# Check binary size
cargo bloat --release

# Profile with cargo-bloat
cargo install cargo-bloat
cargo bloat --release -n 20
```

### Health Check Endpoint

```rust
// src/handlers/health.rs
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

// In Dockerfile HEALTHCHECK
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

---

## Coding Standards

### Formatting - rustfmt

Always use `rustfmt`. Never manually format code.

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
```

Run: `cargo fmt` (format) / `cargo fmt --check` (CI gate)

### Linting - Clippy

Run clippy with warnings as errors in CI:
```bash
cargo clippy -- -D warnings
```

Recommended lint config in `Cargo.toml` or `lib.rs`:
```rust
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
```

### Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Functions, methods, variables | `snake_case` | `get_user`, `user_id` |
| Types, structs, enums, traits | `CamelCase` | `UserService`, `AppError` |
| Constants, statics | `SCREAMING_SNAKE_CASE` | `MAX_RETRIES` |
| Modules, files | `snake_case` | `user_service.rs` |
| Crates | `kebab-case` | `my-crate` |
| Lifetimes | Short lowercase | `'a`, `'db`, `'static` |
| Type parameters | Single uppercase or descriptive | `T`, `E`, `K`, `V`, `Item` |

### Module Organization

```
src/
├── lib.rs          # library root, re-exports public API
├── main.rs         # binary entry point (thin - delegates to lib)
├── error.rs        # AppError definition
├── config.rs       # configuration types
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── order.rs
├── services/
│   ├── mod.rs
│   └── user_service.rs
├── repositories/
│   ├── mod.rs
│   └── user_repo.rs
└── handlers/       # HTTP handlers (thin)
    ├── mod.rs
    └── users.rs
```

### File Size Guidelines
- Target: 200-400 lines per file
- Maximum: 800 lines (split if exceeded)
- Each file should have a single clear responsibility

### Visibility Rules

```rust
pub struct User { ... }        // public to all
pub(crate) fn internal() {}   // public within crate
pub(super) fn parent_only() {} // public to parent module
fn private() {}               // private (default)
```

**Rule**: Keep visibility as restricted as possible. Only `pub` what external consumers need.

### Documentation

```rust
/// Creates a new user with the given email.
///
/// # Errors
/// Returns [`AppError::Validation`] if email is invalid.
/// Returns [`AppError::Database`] if the insert fails.
///
/// # Examples
/// ```
/// let user = UserService::create("alice@example.com").await?;
/// ```
pub async fn create(email: &str) -> Result<User, AppError> { ... }
```

- Document all `pub` items
- Include `# Errors` section on fallible functions
- Include `# Panics` section if the function can panic
- Run `cargo doc --no-deps --open` to verify

### Code Quality Checklist

- [ ] Functions < 50 lines
- [ ] No nesting > 4 levels deep
- [ ] No `unwrap()` in non-test code
- [ ] All `pub` items have doc comments
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No dead code (`#[allow(dead_code)]` only with comment explaining why)

---

## TDD Workflow

### Red -> Green -> Refactor

```rust
// 1. RED: Write a Failing Test First
#[test]
fn parse_valid_email_succeeds() {
    let result = Email::parse("alice@example.com");
    assert!(result.is_ok());
}

#[test]
fn parse_email_without_at_fails() {
    let result = Email::parse("notanemail");
    assert!(matches!(result, Err(EmailError::InvalidFormat)));
}

// Run: cargo test -> should FAIL

// 2. GREEN: Minimal Implementation
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("invalid email format")]
    InvalidFormat,
}

pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::InvalidFormat)
        }
    }
}

// Run: cargo test -> should PASS

// 3. REFACTOR: Apply Rust Idioms
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Run: cargo test -> still PASS
```

### Watch Mode

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-run tests on file change
cargo watch -x test

# Run specific test on change
cargo watch -x "test email"
```

### Test Naming Convention

```rust
// Pattern: {subject}_{condition}_{expected_result}
#[test] fn parse_valid_email_returns_ok() {}
#[test] fn parse_empty_string_returns_invalid_format_error() {}
#[test] fn user_service_create_duplicate_email_returns_validation_error() {}
#[test] fn transfer_insufficient_funds_returns_balance_error() {}
```

### Bug Fix TDD Flow

1. Write a test that **reproduces the bug**:
```rust
#[test]
fn parse_email_with_leading_spaces_succeeds() {
    // Bug: "  alice@example.com" was failing
    assert!(Email::parse("  alice@example.com").is_ok());
}
```
2. Run test -> RED (confirms bug)
3. Fix the bug
4. Run test -> GREEN
5. Add edge case tests

---

## Verification Pipeline

```bash
# Run in this order - fail fast
cargo fmt --check          # 1. formatting
cargo clippy -- -D warnings # 2. linting (warnings = errors)
cargo test                  # 3. all tests
cargo audit                 # 4. security audit
```

### Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/usr/bin/env bash
set -e

echo "Running Rust verification..."

cargo fmt --check || { echo "Run 'cargo fmt' to fix formatting"; exit 1; }
cargo clippy -- -D warnings || { echo "Fix clippy warnings before committing"; exit 1; }
cargo test --quiet || { echo "Tests failed"; exit 1; }

echo "All checks passed!"
```

```bash
chmod +x .git/hooks/pre-commit
```

### GitHub Actions CI

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Tests
        run: cargo test

      - name: Security audit
        run: |
          cargo install cargo-audit --quiet
          cargo audit

      - name: Coverage
        run: |
          cargo install cargo-tarpaulin --quiet
          cargo tarpaulin --fail-under 80
```

### Checkpoint Verification

```bash
#!/usr/bin/env bash
# scripts/verify.sh
set -e

echo "=== Rust Verification Suite ==="

echo "1. Formatting..."
cargo fmt --check

echo "2. Linting..."
cargo clippy -- -D warnings

echo "3. Tests..."
cargo test

echo "4. Doc tests..."
cargo test --doc

echo "5. Security audit..."
cargo audit

echo "6. Coverage..."
cargo tarpaulin --fail-under 80 --out Stdout

echo "=== All checks passed! ==="
```

### Continuous Verification with cargo-watch

```bash
# Auto-run full verification on change
cargo watch -x "fmt --check" -x "clippy -- -D warnings" -x test

# Lighter loop during development (format + test only)
cargo watch -x fmt -x test
```

---

## Configuration Management

```toml
[dependencies]
config = "0.14"
serde = { version = "1", features = ["derive"] }
dotenvy = "0.15"
```

```rust
use config::{Config as ConfigLoader, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub smtp: SmtpConfig,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        ConfigLoader::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
```

---

## Frontend Patterns

### Leptos (Rust Framework)

```rust
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Count: " {count}
        </button>
    }
}
```

### Yew (Rust Framework)

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello from Yew!" }</h1>
    }
}
```

### WASM Integration

```rust
// Expose Rust function to JS
#[wasm_bindgen]
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}
```

### Full-Stack Rust Architecture

```
backend/     # Axum/Actix API
frontend/    # Leptos/Yew app
shared/      # Common types (serde)
```

---

## Best Practices Summary

1. **Ownership**: Use references to avoid unnecessary clones
2. **Error Handling**: Use `thiserror` for application errors
3. **Async**: Never hold locks across `.await` points
4. **Architecture**: Layered design - Handler -> Service -> Repository
5. **Testing**: Mock at trait boundaries
6. **Security**: Run `cargo audit` in CI, no `unwrap()` in prod
7. **Performance**: Use `spawn_blocking` for CPU work
8. **Documentation**: Document all public APIs
9. **Formatting**: Always use `rustfmt`, never manual formatting
10. **Dependencies**: Keep minimal and up-to-date, audit regularly
