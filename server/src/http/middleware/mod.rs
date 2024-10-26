use axum::Router;

mod auth;
mod cors;

pub use auth::middleware as auth;
pub use auth::JWT as JWT;
pub use cors::middleware as cors;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
}
