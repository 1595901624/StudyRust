
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use reqwest::header::HeaderMap;

use crate::dy_model::{DouYinResult, DoYinTauriModel};

mod dy_model;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dy_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_dy_info(input: &str) -> DoYinTauriModel{
    let rt = tokio::runtime::Runtime::new().expect("create tokio erro!");
    let result = rt.block_on(parse_dy(input));
    if result.is_none() {
        return DoYinTauriModel {
            name:String::from("解析失败"),
            url:String::from("解析失败"),
        }
    }
    return result.unwrap();
}

async fn parse_dy(input: &str) -> Option<DoYinTauriModel> {
    // https://v.douyin.com/h1YcUow

    // 1. 请求一次短链获取真实 id
    let result = create_default_client().get(input).send().await;
    if result.is_err() {
        // 请求错误
        println!("请求错误");
        return None;
    }
    let result = result.unwrap();
    // dbg!(&result);
    let path = result.url().path();
    let id = path.replace("/video/", "");

    // 2. 请求拼接请求地址
    let url = "https://www.iesdouyin.com/web/api/v2/aweme/iteminfo/?item_ids=";
    let url = format!("{}{}", url, id);

    // 3. 请求地址并解析结果
    let result = create_default_client().get(url.as_str()).send().await;
    if result.is_err() {
        println!("request url error!");
        return None;
    }
    // 解析成 json 地址
    let result = result.unwrap().json::<DouYinResult>().await;
    if result.is_err() {
        println!("parse json error!");
        return None;
    }

    // 4. 解析数据，输出地址
    let result = parse_result_data(result.unwrap()).await;
    return result;
}

/// 解析结果
async fn parse_result_data(result: DouYinResult) -> Option<DoYinTauriModel> {
    let dou_yin_items = result.item_list;
    if dou_yin_items.is_none() {
        println!("result is not exist!");
        return None;
    }

    let dou_yin_items = dou_yin_items.unwrap();

    let dou_yin_item = dou_yin_items.into_iter().nth(0);
    if dou_yin_item.is_none() {
        println!("data is not exist!");
        return None;
    }
    let dou_yin_item = dou_yin_item.unwrap();
    // 视频名称
    let video_name = dou_yin_item.desc.unwrap_or_default();

    let dou_yin_video = dou_yin_item.video;
    if dou_yin_video.is_none() {
        println!("video is not exist!");
        return None;
    }
    let dou_yin_video = dou_yin_video.unwrap();
    let play_address = dou_yin_video.play_addr;

    // 播放地址
    if play_address.is_none() {
        println!("play_address is not exist!");
        return None;
    }
    let play_address = play_address.unwrap();
    let play_list = play_address.url_list.unwrap_or_default();
    if play_list.is_empty() {
        println!("play list is not exist!");
        return None;
    }

    let video_url = play_list.into_iter().nth(0).unwrap_or_default();

    // 现在获取的地址直接下载会存在水印，我们需要把地址中的 playwm 替换为 play
    // https://aweme.snssdk.com/aweme/v1/playwm/?video_id=v0200fg10000cdieeejc77ucgndnburg&ratio=720p&line=0
    // https://aweme.snssdk.com/aweme/v1/play/?video_id=v0200fg10000cdieeejc77ucgndnburg&ratio=720p&line=0
    let video_url = video_url.replace("playwm", "play");
    // dbg!(&video_url);
    if video_url.is_empty() {
        println!("video url is not exist!");
        return None;
    }
    let video_response = create_default_client().get(video_url.as_str()).send().await;
    if video_response.is_err() {
        println!("video url response error!");
        return None;
    }
    let video_response = video_response.unwrap();
    let real_video_url = video_response.url().to_string();

    println!("视频名称：{}", video_name);
    println!("视频播放地址：{}", real_video_url);
    return Some(DoYinTauriModel {
        name:video_name,
        url:real_video_url
    });
}


/// 创建一个客户端
fn create_default_client() -> reqwest::Client {
    let headers = HeaderMap::new();
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36 Edg/107.0.1418.62";
    return reqwest::Client::builder().default_headers(headers).connect_timeout(Duration::from_secs(10)).user_agent(agent).build().unwrap();
}
