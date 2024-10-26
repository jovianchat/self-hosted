pub mod http;
pub mod models;
mod routes;
pub mod utils;

pub use utils::error::Error as Error;
pub use anyhow::Context as Context;
pub type Result<T, E = Error> = std::result::Result<T, E>;