use axum::{routing::get, Router};

mod api_configs;
mod prompt_engineering;
mod selected_api_prompt;
pub use selected_api_prompt::get_selected_fn;

pub fn router() -> Router {
    Router::new()
        .route(
            "/prompt-engineering",
            get(prompt_engineering::get_configs_handler)
                .post(prompt_engineering::add_new_config)
                .delete(prompt_engineering::delete_config)
                .patch(prompt_engineering::update_config),
        )
        .route(
            "/llm-api-config",
            get(api_configs::get_configs_handler)
                .post(api_configs::add_new_config)
                .delete(api_configs::delete_config)
                .patch(api_configs::update_config),
        )
        .route(
            "/selected-api-prompt",
            get(selected_api_prompt::get_handler).put(selected_api_prompt::update_handler),
        )
}
