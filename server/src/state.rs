use sqlx::sqlite::SqlitePool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub tx: broadcast::Sender<String>,
}