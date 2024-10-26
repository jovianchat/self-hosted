use crate::{
    models::llm::{ApiConfig, PromptConfig},
    Context, Result,
};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SelectedApiPrompt {
    api_id: u32,
    model: String,
    prompt_id: u32,
}

pub fn get_selected_fn() -> Result<(ApiConfig, String, PromptConfig)> {
    let content = read_toml()?;
    let apis = super::api_configs::get_configs_fn()?;
    let selected_api = apis
        .into_iter()
        .find(|api| api.id == content.api_id)
        .expect("Selected API not found");
    let model = content.model;
    let prompt = super::prompt_engineering::get_configs_fn()?;
    let selected_prompt = prompt
        .into_iter()
        .find(|prompt| prompt.id == content.prompt_id)
        .expect("Selected Prompt not found");
    Ok((selected_api, model, selected_prompt))
}

pub async fn get_handler() -> Result<Json<SelectedApiPrompt>> {
    let content = read_toml()?;
    Ok(Json(content))
}

pub async fn update_handler(Json(content): Json<SelectedApiPrompt>) -> Result<StatusCode> {
    save_toml(content)?;
    Ok(StatusCode::OK)
}

fn read_toml() -> Result<SelectedApiPrompt> {
    let content = std::fs::read_to_string("admin/selected-api-prompt.toml")
        .expect("Failed to read prompt-engineering configs from file");
    let toml: SelectedApiPrompt = toml::from_str(&content)
        .expect("Failed to parse prompt-engineering configs from toml file");
    Ok(toml)
}
fn save_toml(toml: SelectedApiPrompt) -> Result<()> {
    let content =
        toml::to_string(&toml).context("Failed to parse prompt-engineering config to toml")?;
    std::fs::write("admin/selected-api-prompt.toml", content)
        .context("Failed to write prompt-engineering config to file")?;
    Ok(())
}
