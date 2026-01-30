use schemars::JsonSchema as SchemarsJsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum LlmBackend {
    Ollama,
    OpenAI,
}

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

#[derive(Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub input: Vec<OpenAIInput>,
    pub response_format: OpenAIResponseFormat,
}

#[derive(Serialize)]
pub struct OpenAIInput {
    pub role: String,
    pub content: Vec<OpenAIContent>,
}

#[derive(Serialize)]
pub struct OpenAIContent {
    pub r#type: String,
    pub text: String,
}

#[derive(serde::Serialize)]
pub struct OpenAIResponseFormat {
    pub r#type: String, // "json_schema"
    pub json_schema: OpenAIJsonSchema,
}

#[derive(serde::Serialize)]
pub struct OpenAIJsonSchema {
    pub name: String,
    pub schema: serde_json::Value,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub output_parsed: Value,
}

// Trait for schema support
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
