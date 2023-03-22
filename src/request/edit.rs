use serde::Deserialize;

use crate::request::Usage;

#[derive(Debug, Deserialize)]
pub struct EditResponse {
    pub object: String,
    pub created: u64,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct EditChoice {
    pub text: String,
    pub index: u64,
}
