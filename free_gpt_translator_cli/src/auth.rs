use base64::{encode_config, URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::str;

fn encode(json: &str) -> String {
    let bytes = json.as_bytes();
    encode_config(bytes, URL_SAFE_NO_PAD)
}

fn encode_string(json: String) -> String {
    encode_config(json.as_bytes(), URL_SAFE_NO_PAD)
}

fn hmac_sha256(data: &str, key: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize().into_bytes();
    let bytes = &result[..];
    encode_config(bytes, URL_SAFE_NO_PAD)
}

pub fn get_token(secret: &str, token: &str, exp: i64) -> String {
    let algo_json = encode(r#"{"alg":"HS256","typ":"JWT"}"#);
    let json = format!(r#"{{"token":"{}","exp":{}}}"#, token, exp);
    let hmac_sha256 = hmac_sha256(&(algo_json.clone() + "." + &encode(&json)), secret);
    format!("{}.{}.{}", algo_json, encode(&json), hmac_sha256)
}

// fn main() {
//     let secret = "your_secret";
//     let token = "your_token";
//     let exp = 1625464800; // example expiration timestamp
//
//     let result = get_token(secret, token, exp);
//     println!("{}", result);
// }
