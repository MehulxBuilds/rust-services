use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await.context("failed to fetch json")?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await.context("failed to read response body")?;
    println!("Body:\n{}", body);
    Ok(())
}
