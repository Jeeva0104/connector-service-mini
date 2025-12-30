pub mod http_utils;
pub mod routes;
pub mod services;
pub mod state;
pub mod types;

use actix_web::{web, App, HttpResponse, HttpServer};
use state::AppState;

pub fn mk_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app_state = AppState::new();

    App::new()
        // .service(routes::Vehicles::server(app_state.clone()))
        .service(routes::Authorize::server(app_state.clone()))
        .route("/health", web::get().to(health_check))
}

pub async fn start_application() -> std::io::Result<()> {
    env_logger::init();

    println!("Starting Vehicle Management API at http://localhost:5000");
    println!("Flow: vehicle_management");
    println!("Available endpoints:");
    println!("  POST /vehicles/v1 - Create vehicle (with full server_wrap)");
    println!("  POST /authorize/v1 - Create authorize (with full server_wrap)");

    println!("  GET  /health      - Health check");

    HttpServer::new(|| mk_app())
        .bind("127.0.0.1:5000")?
        .run()
        .await
}

async fn health_check() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Vehicle Management API",
        "flow": "vehicle_management"
    })))
}
