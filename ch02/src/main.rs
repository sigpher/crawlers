use std::{collections::HashMap, error, time::Duration};

use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // let url = "https://www.httpbin.org/post";
    let url = "https://ssr3.scrape.center/";
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0";
    let mut params = HashMap::new();
    params.insert("name", "germey");
    let client = ClientBuilder::new()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    let resp = client
        .post(url)
        .basic_auth("admin", Some("admin"))
        .form(&params)
        .send()
        .await?;
    let body = resp.text_with_charset("utf-8").await?;
    println!("{}", body);
    Ok(())
}
