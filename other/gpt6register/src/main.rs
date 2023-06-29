use std::collections::HashMap;
use reqwest::Error;

use serde::Serialize;

const CODE_URL: &str = "https://chatgpt6.com/rest/user/send_code";
const REGISTER_URL: &str = "https://chatgpt6.com/rest/user/register";

#[tokio::main]
async fn main() {
    let fetch_msg_url = "https://linshiyou.com/#/fekken@linshiyou.com";
    get(fetch_msg_url).await;
    // match get("https://www.baidu.com").await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         dbg!(e);
    //     }
    // };
}

fn create_http_client() -> reqwest::Client {
    let proxy = reqwest::Proxy::all("http://114.233.70.231:9000").unwrap();
    reqwest::Client::builder().build().unwrap()
}

async fn get(url: &str) -> Result<(), reqwest::Error> {
//     https://youxiang.dev/api/messages/99999@youxiang.dev/bHnBNUX2RJwqd8zscm3O
    let res = create_http_client().get(url).send().await?;
    let msg = res.text().await?;
    println!("[GET][{url}]\n{msg}");
    Ok(())
}

async fn post<T>(url: &str, form_body: HashMap<&str, T>) -> Result<(), reqwest::Error> where T: Serialize + Sized {
    let res = create_http_client().post(url).form(&form_body).send().await?;
    let msg = res.text().await?;
    println!("[POST][{url}]\n{msg}");
    Ok(())
}

/// 发送验证码
async fn send_code() -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("type", "register");
    params.insert("username", "mizjerafi@snapmail.cc");
    post(CODE_URL, params).await?;
    Ok(())
}

async fn register() -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("code", "9492");
    params.insert("username", "886@linshiyou.com");
    params.insert("password", "1099bbb2eb8d1f2d91225eed9bb26b2b");
    params.insert("extra_params", r#"{"activity_code":"e10adc3949ba59abbe56e057f20f883e","inviter":"432728752128"}"#);
    post(REGISTER_URL, params).await?;
    Ok(())
}
