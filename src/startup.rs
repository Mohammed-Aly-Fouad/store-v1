use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Duration};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    // SAFETY: We use `let _ =` instead of `.expect()` here to prevent the application 
    // from panicking if the global logger is initialized more than once 
    // (e.g., during parallel automated test runs).
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub async fn database_connection() -> PgPool {
    tracing::debug!("Setting up database connection");
    if let Err(e) = dotenvy::dotenv() {
    println!("Could not load .env file: {:?}", e);
}
    let db_url = dotenvy::var("DATABASE_URL").expect("Failed to get database url from env");

    let options = PgConnectOptions::from_str(&db_url)
        .expect("failed to parse url")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("failed to connect to the database");

    tracing::debug!("Successfully connected");

    // ---- تشغيل الـ migrations هنا ----
    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to run database migrations");

    tracing::debug!("Migrations applied successfully");

    pg_pool
}