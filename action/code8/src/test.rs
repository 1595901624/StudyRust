#[tokio::test]
async fn test() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.baidu.com")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}