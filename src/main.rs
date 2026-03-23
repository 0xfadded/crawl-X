use agent_twitter_client::scraper::Scraper;
use std::env;
use std::fs;
use std::io::BufWriter;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_id = env::args().nth(1).expect("Please provide a target account ID");
    println!("Crawling tweets and replies for account ID: {}", account_id);

    let twitter_username = env::var("TWITTER_USERNAME").expect("TWITTER_USERNAME is not set");
    let twitter_password = env::var("TWITTER_PASSWORD").expect("TWITTER_PASSWORD is not set");
    let twitter_email = env::var("TWITTER_EMAIL").ok();
    let twitter_2fa_secret = env::var("TWITTER_2FA_SECRET").ok();

    let mut scraper = Scraper::new().await?;

    // Attempt to load cookies
    let has_cookies = if fs::metadata("cookies.json").is_ok() {
        if let Ok(cookies) = fs::read_to_string("cookies.json") {
            if let Err(e) = scraper.set_cookies(&cookies).await {
                println!("Failed to set cookies: {:?}", e);
                false
            } else {
                println!("Loaded cookies from cookies.json");
                true
            }
        } else {
            false
        }
    } else {
        false
    };

    if !has_cookies {
        println!("Logging in...");
        match scraper.login(twitter_username.clone(), twitter_password.clone(), twitter_email.clone(), twitter_2fa_secret.clone()).await {
            Ok(_) => println!("Successfully logged in."),
            Err(e) => println!("Error logging in: {:?}", e),
        }
    } else {
         println!("Skipping login due to existing cookies. (If you still get 403, delete cookies.json and retry)");
    }

    // Save cookies
    match scraper.save_cookies("cookies.json").await {
        Ok(_) => println!("Saved cookies to cookies.json"),
        Err(e) => println!("Failed to save cookies: {:?}", e),
    }

    let mut cursor: Option<String> = None;
    let mut all_tweets = Vec::new();

    loop {
        println!("Fetching tweets... (cursor: {:?})", cursor);
        let response = match scraper.fetch_tweets_and_replies_by_user_id(&account_id, 200, cursor.as_deref()).await {
            Ok(res) => res,
            Err(e) => {
                println!("Error fetching tweets: {:?}", e);
                break;
            }
        };

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
