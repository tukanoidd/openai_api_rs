use serde::Deserialize;

use crate::request::Usage;

#[derive(Debug, Deserialize)]
pub struct TextCompletionResponse {
    pub choices: Vec<TextCompletionChoice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionChoice {
    pub finish_reason: String,
    pub index: u64,
    pub logprobs: Option<u8>,
    pub text: String,
}
