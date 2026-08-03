use std::fs::File;
use std::io::copy;
use tempfile::Builder;
use anyhow::{Context, Ok};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let temp_dir = Builder::new().prefix("example").tempdir().context("Failed to create temporary directory")?;
    let target = "https://mehulxbuilds.in/images/mehulxbuilds.png";
    let response = reqwest::get(target).await.context("failed to get file")?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = temp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };

    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
