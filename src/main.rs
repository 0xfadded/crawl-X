use agent_twitter_client::scraper::Scraper;
use std::env;
use std::fs::File;
use std::io::BufWriter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_id = env::args().nth(1).expect("Please provide a target account ID");
    println!("Crawling tweets and replies for account ID: {}", account_id);

    let twitter_username = env::var("TWITTER_USERNAME").expect("TWITTER_USERNAME is not set");
    let twitter_password = env::var("TWITTER_PASSWORD").expect("TWITTER_PASSWORD is not set");
    let twitter_email = env::var("TWITTER_EMAIL").ok();
    let twitter_2fa_secret = env::var("TWITTER_2FA_SECRET").ok();

    let mut scraper = Scraper::new().await?;

    println!("Logging in...");
    scraper.login(twitter_username, twitter_password, twitter_email, twitter_2fa_secret).await?;
    println!("Successfully logged in.");

    let mut cursor: Option<String> = None;
    let mut all_tweets = Vec::new();

    loop {
        println!("Fetching tweets... (cursor: {:?})", cursor);
        let response = scraper.fetch_tweets_and_replies_by_user_id(&account_id, 200, cursor.as_deref()).await?;

        let batch_size = response.tweets.len();
        println!("Fetched {} tweets.", batch_size);

        all_tweets.extend(response.tweets);

        if let Some(next_cursor) = response.next {
            cursor = Some(next_cursor);
        } else {
            break;
        }

        if batch_size == 0 {
            break;
        }
    }

    println!("Total tweets fetched: {}", all_tweets.len());

    let output_file = format!("tweets_{}.json", account_id);
    let file = File::create(&output_file)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &all_tweets)?;
    println!("Successfully saved {} tweets to {}", all_tweets.len(), output_file);

    Ok(())
}
