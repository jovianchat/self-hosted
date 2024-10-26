use std::collections::HashMap;

use async_openai::{config::OpenAIConfig, Client};


pub fn init_openai_clients() -> HashMap<&'static str, Client<OpenAIConfig>> {
    let mut llm_clients = HashMap::new();

    let openai_config = OpenAIConfig::new().with_api_key(env_var("OPENAI_API_KEY"));
    let openai_client = Client::with_config(openai_config);
    llm_clients.insert("openai", openai_client);

    let hyperbolic_url = "https://api.hyperbolic.xyz/v1";
    let hyperbolic_config = OpenAIConfig::new().with_api_key(env_var("HYPERBOLIC_API_KEY")).with_api_base(hyperbolic_url);
    let hyperbolic_client = Client::with_config(hyperbolic_config);
    llm_clients.insert("hyperbolic", hyperbolic_client);

    llm_clients
}

fn env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{}: {}", name, e))
        .expect("Missing environment variable")
}