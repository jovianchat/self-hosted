use axum::{middleware::from_fn, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;

mod layers;
pub mod middleware;
use crate::{models::cache::CacheState, routes};

#[derive(Clone)]
pub struct AppState {
    pub user: String,
    pub pg_pool: PgPool,
    pub cache: CacheState,
}

pub async fn serve(listen_addr: std::net::SocketAddr, pg_pool: PgPool) {
    let cache = CacheState::new();
    let app_state = AppState { user: "admin".to_string(), pg_pool, cache };

    let app = Router::new()
        .merge(routes::router_with_state())
        .with_state(app_state.clone())
        .merge(routes::router())
        .merge(middleware::router())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::cors())
                .layer(from_fn(middleware::auth)),
        );

    let listener = tokio::net::TcpListener::bind(listen_addr)
        .await
        .expect("Failed to bind Listen address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server")
}
