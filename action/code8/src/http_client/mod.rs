pub(crate) mod request;
pub(crate) mod response;

use std::time::Duration;
use reqwest::{Error, Method};
use crate::http_client::request::{RequestBody, RequestBodyBuilder};

/// 发送请求
pub fn send_request(url: String, method: Method, request_body: RequestBody) -> Result<reqwest::blocking::Response, Error> {
    let client_builder = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15));
    let client = client_builder.build()?;
    let response = client.request(method, url).add_body(&request_body).send()?;
    Ok(response)
}
