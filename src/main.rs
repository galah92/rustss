use askama::Template;
use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index));

    let ip = std::env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("{}:{}", ip, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn index(Query(params): Query<IndexParams>) -> axum::response::Result<IndexTemplate> {
    let url = params
        .url
        .unwrap_or_else(|| "https://simonwillison.net/atom/everything/".to_string());
    let count = params.count.unwrap_or(3);

    let articles = rustss::fetch_last_articles(&url, count)
        .await
        .unwrap_or(vec![]);

    let template = IndexTemplate {
        url,
        count,
        articles,
    };
    Ok(template)
}

#[derive(Deserialize, Debug)]
struct IndexParams {
    url: Option<String>,
    count: Option<usize>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    url: String,
    count: usize,
    articles: Vec<rustss::Article>,
}
