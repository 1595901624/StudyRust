#[tokio::test]
async fn test() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.baidu.com")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}

#[tokio::test]
async fn test_db() {
    // 请求豆瓣网址
    let url = "https://movie.douban.com/top250";
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.64";
    let result = reqwest::ClientBuilder::new().user_agent(ua).build().unwrap().get(url).send().await.unwrap().text().await.unwrap();

    // 解析
    let document = scraper::Html::parse_document(result.as_str());
    let li_selector = scraper::Selector::parse(".grid_view > li > div > div > div > a > .title:first-child").unwrap();
    let movies = document.select(&li_selector).map(|x| x.inner_html());
    let mut index = 1;
    for title in movies {
        println!("{index}、{title}");
        index += 1;
    }
}