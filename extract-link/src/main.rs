use select::document::Document;
use select::predicate::Name;
use anyhow::{Context, Ok};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res= reqwest::get("https://mehulxbuilds.in")
        .await?
        .text()
        .await
        .context("Failed to fetch the URL")?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| println!("{}", link));

    Ok(())
}
