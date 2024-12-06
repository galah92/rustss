use feed_rs::parser;

async fn fetch_last_articles(url: &str, count: usize) -> anyhow::Result<Vec<String>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let feed = parser::parse(&content[..])?;

    Ok(feed
        .entries
        .iter()
        .take(count)
        .map(|entry| {
            format!(
                "{}: {}",
                entry
                    .title
                    .as_ref()
                    .map(|t| t.content.as_str())
                    .unwrap_or_default(),
                entry
                    .links
                    .first()
                    .map(|l| l.href.as_str())
                    .unwrap_or_default()
            )
        })
        .collect())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let articles = fetch_last_articles("https://simonwillison.net/atom/everything/", 3).await?;
    for article in articles {
        println!("{}", article);
    }
    Ok(())
}
