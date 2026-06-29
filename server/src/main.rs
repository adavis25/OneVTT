mod handlers;
mod models;
mod state;

use axum::{routing::{delete, get, post}, Router};
use sqlx::sqlite::SqlitePoolOptions;
use state::AppState;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let db_path = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:world.db?mode=rwc".to_string());

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    info!("Database connected and migrations applied");

    let (tx, _rx) = broadcast::channel::<String>(100);

    let state = Arc::new(AppState { db, tx });

    let app = Router::new()
        .route("/health", get(handlers::health::health_handler))
        .route("/ws", get(handlers::ws::ws_handler))
        .route("/api/worlds", get(handlers::world::list_worlds))
        .route("/api/worlds", post(handlers::world::create_world))
        .route("/api/worlds/:id", delete(handlers::world::delete_world))
        .route("/api/actors", get(handlers::actors::list_actors))
        .route("/api/actors", post(handlers::actors::create_actor))
        .nest_service(
            "/",
            ServeDir::new("../client/dist")
                .fallback(ServeFile::new("../client/dist/index.html")),
        )
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to port");

    info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}