use server::{
    http,
    utils::{db::Db, env_vars::Configuration, telemetry},
};

#[tokio::main]
async fn main() {
    telemetry::init_tracing();

    tracing::debug!("Initializing configuration");
    let env_vars = Configuration::new();

    tracing::debug!("Initializing db pool");
    let db = Db::new(&env_vars.db_dsn, env_vars.db_pool_max_size)
        .await
        .expect("Failed to initialize db");
    tracing::debug!("Running migrations");
    db.migrate().await.expect("Failed to run migrations");

    tracing::info!("Starting server on {}", env_vars.listen_address);
    http::serve(env_vars.listen_address, db.pool).await
}
