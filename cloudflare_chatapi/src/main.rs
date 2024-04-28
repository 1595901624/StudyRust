use axum::{
    async_trait,
    body::Body,
    extract::{ContentLengthLimit, Json, RawBody},
    http::Request,
    response::Response,
    routing::post,
    Router, Server,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::extract::{FromRequest, RequestParts};
use http::HeaderValue;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/proxy", post(proxy_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn proxy_handler(
    ContentLengthLimit(Json(payload)): ContentLengthLimit<Json<ProxyRequest>, { 1024 * 10 }>,
) -> Response<Body> {
    // 转发请求到外部API
    let client = Client::new();
    let res = client.post("https://playground.ai.cloudflare.com/api/inference")
        .header("Cookie", HeaderValue::from_static("__cf_bm=I9wbf2jkhEyUL_uNiTaAOTRyNtYMsWAH96hRy0Iyofk-1714189957-1.0.1.1-xbY2FEeTNU1ZY3n0MQ5XD0ijHqtmCbs0LyLczAhOh8ku6HPd1z2dAP70dcVL24v3BXKxWCoRK60t3eeTh9tAzA; path=/; expires=Sat, 27-Apr-24 04:22:37 GMT; domain=.ai.cloudflare.com; HttpOnly; Secure"))
        .json(&payload)
        .send()
        .await;
    dbg!(&res);

    // 根据外部API响应处理结果
    match res {
        Ok(external_response) => {
            let status = external_response.status();
            let headers = external_response.headers().clone();
            let body = external_response.bytes().await.unwrap();

            let mut response = Response::new(Body::from(body));
            *response.status_mut() = status;
            *response.headers_mut() = headers;
            response
        }
        Err(_) => {
            let response = Response::builder()
                .status(502)
                .body(Body::from("Bad Gateway"))
                .unwrap();
            response
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ProxyRequest {
    messages: Vec<Message>,
    model: String,
    max_tokens: u32,
    stream: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

// #[async_trait]
// impl<B> FromRequest<B> for ProxyRequest
//     where
//         B: Send, // required by `async_trait`
// {
//     type Rejection = (http::StatusCode, String);
//
//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let bytes = RawBody::from_request(req).await?.0;
//         serde_json::from_slice(&bytes).map_err(|_| {
//             (
//                 http::StatusCode::BAD_REQUEST,
//                 "Invalid JSON".to_string(),
//             )
//         })
//     }
// }