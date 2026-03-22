# Twitter Crawler

A Rust tool for crawling posts and replies from an X (formerly Twitter) account.

## Usage

You must provide Twitter credentials via environment variables to avoid the 403 Forbidden ban when fetching tweets:

```bash
export TWITTER_USERNAME=your_username
export TWITTER_PASSWORD=your_password
export TWITTER_EMAIL=your_email@example.com

cargo run -- elonmusk
```

Or pass via command line:
```bash
cargo run -- elonmusk
```

It uses `agent-twitter-client` which is designed to emulate Twitter Web Client behavior (using cookies and the `UserAuth` flow) to minimize banning.
