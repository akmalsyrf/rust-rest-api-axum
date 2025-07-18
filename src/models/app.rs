use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub connection_pool: MySqlPool,
}