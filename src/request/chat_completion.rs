use crate::request::Usage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: u64,
    pub message: ChatMessage,
    pub finish_reason: String,
}
