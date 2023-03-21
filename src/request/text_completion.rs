use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TextCompletionResponse {
    pub choices: Vec<TextCompletionChoice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: TextCompletionUsage,
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionChoice {
    pub finish_reason: String,
    pub index: u64,
    pub logprobs: Option<u8>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TextCompletionUsage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}
