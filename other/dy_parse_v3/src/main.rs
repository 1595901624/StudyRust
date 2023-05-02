use std::error::Error;
use std::ffi::OsStr;

use headless_chrome::{Browser, LaunchOptions, LaunchOptionsBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://v.douyin.com/DbnFSdG/";
    // let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.64";

    let launch_options = LaunchOptionsBuilder::default().headless(true).build()?;
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;

    tab.wait_for_element("video")?;
    let result = tab.get_content()?;

    // // 解析
    let document = scraper::Html::parse_document(result.as_str());
    let video_selector = scraper::Selector::parse("video > source").unwrap();
    let videos = document.select(&video_selector).map(|x| {
        let element = x.value();
        let attrs = element.attrs();
        for (key, value) in attrs {
            if key == "src" {
                return value;
            }
        }
        return "";
    });
    let mut index = 1;
    for video_url in videos {
        if video_url.starts_with("//") {
            println!("下载地址 {index} -> https:{video_url}");
        } else {
            println!("下载地址 {index} -> {video_url}");
        }
        index += 1;
    }
    Ok(())
}