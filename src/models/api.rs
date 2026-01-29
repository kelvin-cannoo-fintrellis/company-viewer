use schemars::JsonSchema as SchemarsJsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Ollama Chat API models
#[derive(Serialize)]
pub struct OllamaChatRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<Message<'a>>,
    pub stream: bool,
    pub format: Value,
}

#[derive(Serialize)]
pub struct Message<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Deserialize)]
pub struct OllamaChatResponse {
    pub message: MessageResponse,
}

#[derive(Deserialize)]
pub struct MessageResponse {
    pub content: String,
}

// Trait for types that can provide their JSON schema
pub trait JsonSchema {
    fn schema() -> Value;
}

// Blanket implementation for any type that implements schemars::JsonSchema
impl<T: SchemarsJsonSchema> JsonSchema for T {
    fn schema() -> Value {
        let schema = schemars::schema_for!(T);
        serde_json::to_value(schema).unwrap()
    }
}
