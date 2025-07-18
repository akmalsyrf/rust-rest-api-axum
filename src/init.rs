use tracing_subscriber::{EnvFilter, FmtSubscriber};

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions},
};
use std::{str::FromStr, time::Duration};


pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");

    // tracing::error!("error log");
    // tracing::warn!("warn log");
    // tracing::info!("info log");
    // tracing::debug!("debug log");
    // tracing::trace!("tracing log");
}

pub async fn database_connection() -> MySqlPool {
    tracing::debug!("Setting up database connection");
    let db_url = dotenvy::var("DATABASE_URL").expect("Failed to get database url from env");

    let options = MySqlConnectOptions::from_str(&db_url)
        .expect("failed to parse url");
        
    // The MySqlConnectOptions does not have a `disable_statement_logging` method directly.
    // Logging is configured differently or is off by default for statements.
    // If you need to suppress logs, you would typically manage this via the RUST_LOG env var for sqlx::query.
    // For this reason, the line has been removed as it's not a direct 1-to-1 mapping.

    let mysql_pool = MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("failed to connect to the database");

    tracing::debug!("Successfully connected");

    sqlx::migrate!()
        .run(&mysql_pool)
        .await
        .expect("Failed to migrate");

    tracing::debug!("Successfully migrated");

    mysql_pool
}