use feed_rs::parser;

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

impl std::fmt::Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.url)
    }
}

pub async fn fetch_last_articles(url: &str, count: usize) -> anyhow::Result<Vec<Article>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let feed = parser::parse(&content[..])?;

    Ok(feed
        .entries
        .iter()
        .take(count)
        .map(|entry| Article {
            title: entry
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_default(),
            url: entry
                .links
                .first()
                .map(|l| l.href.clone())
                .unwrap_or_default(),
        })
        .collect())
}
