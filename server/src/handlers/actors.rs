use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
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

struct ActorRow {
    id: String,
    name: String,
    actor_type: String,
    data: String,
    world_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateActorRequest {
    pub name: Option<String>,
    pub data: Option<serde_json::Value>,
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

// PATCH /api/actors/:id
pub async fn update_actor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateActorRequest>,
) -> impl IntoResponse {
    let data_str = payload.data.as_ref().map(|d| d.to_string());
    let name_bc = payload.name.clone();

    let result = sqlx::query!(
        "UPDATE actors SET name = COALESCE(?, name), data = COALESCE(?, data), updated_at = unixepoch() WHERE id = ?",
        payload.name,
        data_str,
        id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => (StatusCode::NOT_FOUND, Json(json!({}))),
        Ok(_) => {
            let event = json!({
                "type": "actor.updated",
                "data": { "id": id, "name": name_bc }
            });
            let _ = state.tx.send(event.to_string());
            (StatusCode::OK, Json(json!({ "id": id })))
        }
        Err(e) => {
            tracing::error!("Failed to update actor {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})))
        }
    }
}

// GET /api/actors/:id
pub async fn get_actor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let row = sqlx::query_as!(
        ActorRow,
        r#"SELECT id as "id!", name as "name!", actor_type as "actor_type!", data as "data!", world_id
           FROM actors WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db)
    .await;

    match row {
        Ok(Some(r)) => {
            let data: serde_json::Value = serde_json::from_str(&r.data).unwrap_or(json!({}));
            (StatusCode::OK, Json(json!({
                "id": r.id, "name": r.name, "actor_type": r.actor_type,
                "data": data, "world_id": r.world_id
            })))
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({}))),
        Err(e) => {
            tracing::error!("Failed to get actor {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})))
        }
    }
}

// DELETE /api/actors/:id
pub async fn delete_actor(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM actors WHERE id = ?", id)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => StatusCode::NOT_FOUND,
        Ok(_) => {
            let event = json!({ "type": "actor.deleted", "data": { "id": id } });
            let _ = state.tx.send(event.to_string());
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            tracing::error!("Failed to delete actor {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
