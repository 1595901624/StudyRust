use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};

use crate::model::GPTResult;

mod model;

#[tokio::main]
async fn main() {
    // 请填入自己账号的openai 的 api_key
    let api_key = "";

    // 对话内容
    let prompt = "chatgpt是什么";

    // 完成时要生成的最大 token 数
    let max_tokens = 4000;

    // 训练模型
    // 功能最强大的 GPT-3 模型。可以完成其他模型可以完成的任何任务，通常具有更高的质量、更长的输出和更好的指令遵循。还支持在文本中插入补全。
    let model = "text-davinci-003";

    let url = "https://api.openai.com/v1/completions";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());

    let body = format!(r#"{{"model": "{}", "prompt": "{}", "max_tokens": {}, "temperature": 0}}"#, model, prompt, max_tokens);
    let response = reqwest::Client::builder().build().unwrap()
        .post(url)
        .headers(headers)
        .body(body)
        .send().await.unwrap()
        .json::<GPTResult>().await.unwrap();

    // dbg!(&response);

    let choices = response.choices.unwrap();
    let choice = choices.get(0).unwrap();
    println!("{:?}", choice.text.as_ref().unwrap().trim());
}
