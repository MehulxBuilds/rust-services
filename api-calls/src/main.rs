use std::format;
use anyhow::Context;
use serde::Deserialize;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct Owner {
    login: String,
}

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    owner: Owner,
    stargazers_count: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}",
        owner = "mehulxbuilds",
        repo = "x8"
    );

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "reqwest")
        .send()
        .await.context("failed to fetch response")?;

    let user: User = response
        .json()
        .await
        .context("failed to parse response as JSON")?;

    println!("{:?} {:?} {:?} {:?}", user.id, user.name, user.owner.login, user.stargazers_count);

    Ok(())
}
