use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]  // ← was missing
pub struct WorldSummary {
    pub id: Option<String>,
    pub name: Option<String>,
    pub game_system: Option<String>,
    pub last_opened: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateWorldRequest {
    pub name: String,
    pub game_system: String,
}

#[derive(Serialize)]
pub struct CreateWorldResponse {
    pub id: String,
    pub name: String,
    pub game_system: String,
}

// GET /api/worlds — list all worlds
pub async fn list_worlds(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let worlds = sqlx::query_as!(
        WorldSummary,
        "SELECT id as \"id!\", name as \"name!\", game_system as \"game_system!\", last_opened
         FROM worlds
         ORDER BY last_opened DESC, created_at DESC"
    )
    .fetch_all(&state.db)
    .await;

    match worlds {
        Ok(worlds) => (StatusCode::OK, Json(worlds)),
        Err(e) => {
            tracing::error!("Failed to list worlds: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

// POST /api/worlds — create a new world
pub async fn create_world(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorldRequest>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let db_path = format!("worlds/{}.db", id);

    tokio::fs::create_dir_all("worlds").await.ok();

    let result = sqlx::query!(
        "INSERT INTO worlds (id, name, game_system, db_path)
         VALUES (?, ?, ?, ?)",
        id,
        payload.name,
        payload.game_system,
        db_path
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(CreateWorldResponse {
                id,
                name: payload.name,
                game_system: payload.game_system,
            }),
        ),
        Err(e) => {
            tracing::error!("Failed to create world: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateWorldResponse {
                    id: String::new(),
                    name: String::new(),
                    game_system: String::new(),
                }),
            )
        }
    }
}

// DELETE /api/worlds/:id — delete a world
pub async fn delete_world(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM worlds WHERE id = ?", id)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to delete world {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}