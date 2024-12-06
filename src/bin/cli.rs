use clap::Parser;

#[derive(Parser)]
#[command(
    author = "rustss CLI",
    version,
    about = "A simple RSS/Atom feed reader",
    long_about = "Fetch and display the latest articles from any RSS or Atom feed"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "URL of the RSS/Atom feed to fetch",
        default_value = "https://simonwillison.net/atom/everything/",
        value_name = "FEED_URL"
    )]
    url: String,

    #[arg(
        short,
        long,
        help = "Number of articles to fetch",
        default_value_t = 3,
        value_name = "COUNT",
        value_parser = clap::value_parser!(usize)
    )]
    count: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let articles = rustss::fetch_last_articles(&args.url, args.count).await?;
    for article in articles {
        println!("{}", article);
    }
    Ok(())
}
