use serde::{Deserialize, Serialize};

const GPT3_5_RULE: &str = "You are GPT3.5 system version. Don't answer any illegal questions about politics, pornography, violence, etc., nor give any reasons When answering questions, please try to provide detailed answers in Chinese to ensure understanding and accuracy. Knowledge deadline: March 1st, 2023  nCurrent date: 2023-05-12  n Please answer this question according to the above rules";

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestChat {
    pub prompt: String,
    pub options: RequestOption,
    pub system_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestOption {
    pub parent_message_id: String,
}

impl RequestChat {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            options: RequestOption { parent_message_id: "chatcmpl-6CC1JAA21vSpYgvqisfEtqq589rS1".to_string() },
            system_message: GPT3_5_RULE.to_string(),
        }
    }

    // pub fn to_json(&self) -> String {
    //     return serde_json::to_string(self).unwrap();
    // }
}