use once_cell::sync::Lazy;
use std::env;

use crate::models::api::LlmBackend;

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub backend: LlmBackend,
    pub openai_api_key: Option<String>,
    pub openai_model: String,
    pub ollama_model: String,
    pub ollama_url: String,
}

impl LlmConfig {
    fn from_env() -> Self {
        let backend = match env::var("LLM_BACKEND")
            .unwrap_or_else(|_| "ollama".to_string())
            .to_lowercase()
            .as_str()
        {
            "openai" => LlmBackend::OpenAI,
            _ => LlmBackend::Ollama,
        };

        Self {
            backend,
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
            openai_model: env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".to_string()),
            ollama_model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5:3b".to_string()),
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
        }
    }
}

pub static LLM_CONFIG: Lazy<LlmConfig> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    LlmConfig::from_env()
});
