use std::str::FromStr;
use clap::{Parser};
use reqwest::{Method};
use crate::args::Args;
use crate::http_client::request::RequestBody;
use crate::http_client::response::Output;

mod http_client;
mod args;
mod test;

fn main() {
    // get_args();

    // 使用clap获取参数
    let args = Args::parse();
    println!("args: {:?}", args);

    let url = args.url;
    let method = Method::from_str(&args.method).unwrap();

    let request_body = RequestBody::new(args.query, args.form, args.json);

    let response = http_client::send_request(url, method, request_body);
    // println!("{:#?}", response);
    // render_raw_content(RefCell::new(response.unwrap())).unwrap();
    response.unwrap().output().unwrap();
}


fn get_args() {
    // 获取参数
    let args: std::env::Args = std::env::args();
    println!("获取参数: {:?}", args);
}
