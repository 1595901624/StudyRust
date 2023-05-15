use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Choices {
    pub delta: Option<Delta>,
    pub index: Option<i64>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choices>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatProcess {
    pub role: Option<String>,
    pub id: Option<String>,
    pub parent_message_id: Option<String>,
    pub text: Option<String>,
    pub detail: Option<Detail>,
}

