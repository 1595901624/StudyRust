use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DouYinResult {
    pub status_code: i32,
    pub item_list: Option<Vec<DouYinItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DouYinItem {
    // id
    pub aweme_id: Option<String>,
    // 视频名称
    pub desc: Option<String>,
    // 创建时间
    pub create_time: Option<i64>,
    // 视频信息
    pub video: Option<DouYinVideo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DouYinVideo {
    pub height: Option<i32>,
    pub width: Option<i32>,
    // 视频播放地址
    pub play_addr: Option<DouYinVideoPlayAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DouYinVideoPlayAddress {
    pub uri: Option<String>,
    // 视频播放地址
    pub url_list: Option<Vec<String>>,
}


/// 返回给前端的信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DoYinTauriModel {
    pub name: String,
    pub url:String
}