use askama::Template;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    url: String,
    count: usize,
    articles: Vec<String>,
}

#[derive(Deserialize)]
pub struct Params {
    url: Option<String>,
    count: Option<usize>,
}

async fn index(Query(params): Query<Params>) -> Html<String> {
    let url = params
        .url
        .unwrap_or_else(|| "https://simonwillison.net/atom/everything/".to_string());
    let count = params.count.unwrap_or(3);

    let articles = match rustss::fetch_last_articles(&url, count).await {
        Ok(articles) => articles.iter().map(|a| a.to_string()).collect(),
        Err(_) => vec!["Error fetching feed".to_string()],
    };

    let template = IndexTemplate {
        url,
        count,
        articles,
    };
    Html(
        template
            .render()
            .unwrap_or_else(|_| "Template error".to_string()),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
