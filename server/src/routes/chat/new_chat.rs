use async_openai::types::CreateChatCompletionRequestArgs;
use axum::{extract::State, Json};

use crate::models::chat::ChatDetails;
use crate::Result;
use crate::{http::AppState, utils::llm_cfg};

// #[axum::debug_handler]
pub async fn create_new_chat(
    State(state): State<AppState>,
    query: String,
) -> Result<Json<ChatDetails>> {
    let pool = &state.pg_pool;

    let title = generate_title(query).await?;

    let details = sqlx::query_as!(
        ChatDetails,
        r#"INSERT INTO ai.chats (title) VALUES ($1) RETURNING *"#,
        title
    )
    .fetch_one(pool)
    .await?;
print!("here");

    // Moka cache invalidate
    let _ = state.cache.chat_history.invalidate(&state.user).await;

    Ok(Json(details))
}

async fn generate_title(query: String) -> Result<String> {
    let llm_api = llm_cfg::init_openai_clients();
    let openai_client = llm_api.get("openai").unwrap();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .max_tokens(512u32)
        .messages([
            async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
            .content("You generate titles with fewer than eight words for chat messages based on user-provided content or queries.")
            .build()?.into(),
            async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(query)
            .build()?.into(),
        ])
        .build()?;
    let response = openai_client.chat().create(request).await?;

    Ok(response.choices[0]
        .message
        .content
        .as_ref()
        .unwrap()
        .to_string())
}
