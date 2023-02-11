use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTResult {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Option<Vec<GPTChoice>>,
    pub usage: Option<GPTUsage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTUsage {
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTChoice {
    pub text: Option<String>,
    pub index: Option<i32>,
    pub finish_reason: Option<String>,
}