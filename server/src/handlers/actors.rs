use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ActorSummary {
    pub id: String,
    pub name: String,
    pub actor_type: String,
}

#[derive(Deserialize)]
pub struct CreateActorRequest {
    pub world_id: String,
    pub name: String,
    pub actor_type: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CreateActorResponse {
    pub id: String,
    pub name: String,
    pub actor_type: String,
}

#[derive(Deserialize)]
pub struct ListActorsQuery {
    pub world_id: String,
}

// GET /api/actors?world_id=:id
pub async fn list_actors(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListActorsQuery>,
) -> impl IntoResponse {
    let actors = sqlx::query_as!(
        ActorSummary,
        r#"SELECT id as "id!", name as "name!", actor_type as "actor_type!"
           FROM actors WHERE world_id = ? ORDER BY created_at ASC"#,
        params.world_id
    )
    .fetch_all(&state.db)
    .await;

    match actors {
        Ok(actors) => (StatusCode::OK, Json(actors)),
        Err(e) => {
            tracing::error!("Failed to list actors: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

// POST /api/actors
pub async fn create_actor(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateActorRequest>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let data = payload.data.unwrap_or_else(|| json!({})).to_string();

    let result = sqlx::query!(
        "INSERT INTO actors (id, name, actor_type, data, world_id) VALUES (?, ?, ?, ?, ?)",
        id,
        payload.name,
        payload.actor_type,
        data,
        payload.world_id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let event = json!({
                "type": "actor.created",
                "data": { "id": id, "name": payload.name, "actor_type": payload.actor_type }
            });
            let _ = state.tx.send(event.to_string());

            (
                StatusCode::CREATED,
                Json(CreateActorResponse {
                    id,
                    name: payload.name,
                    actor_type: payload.actor_type,
                }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to create actor: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateActorResponse {
                    id: String::new(),
                    name: String::new(),
                    actor_type: String::new(),
                }),
            )
        }
    }
}
