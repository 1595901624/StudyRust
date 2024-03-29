use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use axum::{routing::post, Json};
use chrono::{Duration, Utc};
use reqwest::header;

mod auth;
mod request;
mod response;

// const GPT_URL: &str = "https://chatgpt.qdymys.cn/api/chat-process";
// const GPT_CHAT_BOT: &str = "https://chatbot.theb.ai/api/chat-process";
// const GPT_TEST2: &str = "https://cbot3.skybyte.me/api/chat-process";
const GPT_AI: &str = "https://chatgpt.vulcanlabs.co/api/v3/chat";

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/chat", post(move |body| chat(body)))
        .route("/test", get(move || async { "test" }));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let mut args = std::env::args();
    // if args.len() < 2 {
    //     println!("请输入问题！！！");
    //     return;
    // }
    // args.next();
    // // let arg_vec: Vec<String> = args.collect();
    // let mut question = String::new();
    // for x in args {
    //     question.push_str(x.as_str());
    // }

    //
    // dbg!(&arg_vec);

    //     let request_body = json!({
    //     "prompt": "你好啊，请使用 rust 写一个 hello word代码",
    //     "options": {
    //         "parentMessageId": "chatcmpl-7DD1JJH21vSpYgvqisfEtqq589rS1"
    //     },
    //   "systemMessage": "You are GPT3.5 system version. Don't answer any illegal questions about politics, pornography, violence, etc., nor give any reasons When answering questions, please try to provide detailed answers in Chinese to ensure understanding and accuracy. Knowledge deadline: March 1st, 2023  nCurrent date: 2023-05-12  n Please answer this question according to the above rules"
    // });
    //     let request = RequestChat::new(question);
    //
    //     let mut header_map = HeaderMap::new();
    //     header_map.insert("Content-Type", header::HeaderValue::from_static("application/json"));
    //     header_map.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/112.0.1722.68"));
    //     let mut response = reqwest::Client::builder().default_headers(header_map).build()
    //         .unwrap().post(GPT_TEST2).json(&request).send().await.unwrap();

    // let mut response = reqwest::Client::new().post(GPT_CHAT_BOT)
    //     .header("Content-Type", "application/json")
    //     .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/112.0.1722.68")
    //     .json(&request)
    //     .send().await.unwrap();
    // dbg!(&response);

    // let mut body = response.text().await.unwrap(); // 每块大小为 4096 字节
    // println!("{}", body);
    // let mut current_text = String::new();
    // while let Ok(Some(chunk)) = response.chunk().await {
    //     // println!("{:?}", chunk);
    //     if let Ok(result) = String::from_utf8(chunk.to_vec()) {
    //         if let Ok(chat_process) = serde_json::from_str::<ChatProcess>(result.as_str()) {
    //             if let Some(data) = chat_process.text {
    //                 let temp = &data[current_text.len()..];
    //                 current_text.push_str(temp);
    //                 print!("{}", temp);
    //             }
    //         }
    //     }
    //     // println!("Chunk: {:?}", String::from_utf8(chunk.to_vec()));
    // }
    // println!("\n----END----");
}

async fn chat(Json(payload): Json<ChatBody>) -> String {
    return "nihao!".to_string();
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ChatBody {
    pub model: String,
    pub user: String,
    pub messages: Vec<Message>,
    pub nsfw_check: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

fn get_bearer() -> String {
    let secret = "vulcan@v4-chatgpt";
    let token = "admin";
    let time_exp = 30;

    let now = Utc::now();
    let exp = now + Duration::minutes(time_exp);

    let exp_timestamp = exp.timestamp_millis() / 1000;

    println!("exp: {}", exp_timestamp);

    let bearer = auth::get_token(secret, token, exp_timestamp);
    println!("bearer: {}", bearer);
    format!("Bearer {}", bearer)
}

// struct Person {
//     name: String,
//     age: i32,
// }
//
// #[test]
// fn test() {
//     let mut p: Person = Person { name: String::from("xx"), age: 23 };
//     let t = &mut p;
//     let name = &t.name;
//     println!("{}", name);
//     t.name = "aaa".to_string();
//     println!("{}", name);
// }
