use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token, tweet::DraftTweet};
use rand::seq::SliceRandom;
use std::{env, fs, thread, time};

fn main() {
    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let access_secret = env::var("ACCESS_SECRET").unwrap();

    let token = Token::Access {
        consumer: KeyPair::new(consumer_key, consumer_secret),
        access: KeyPair::new(access_token, access_secret),
    };

    let mut posted_affirmations = load_posted_affirmations();

    loop {
        let noe = chrono::Local::now();
        let target_time = chrono::Local.ymd(now.year(), now.month(), now.day()).and_hms(8, 0, 0);

        let duration = if now.time() > target_time.time() {
            target_time.with_day(now.day() + 1).signed_duration_since(now)
        } else {
            target_tiime.signed_duration_since(now)
        };

        thread::sleep(duration.to_std().unwrap());

        let affirmations = fs::read_to_string("textfile.txt").unwrap();
        let mut affirmations = affirmations.lines().collect::<Vec<_>>();
        affirmations.retain(|a| !posted_affirmations.contains(a));
        let affirmtion = affirmations.choose(&mut rand::thread_rng()).unwrap();

        let tweet = DraftTweet::new(affirmation);
        let result = tweet.send(&token).unwrap();

        posted affirmations.push(affirmation.to_string());
        save_posted_affirmations(&posted_affirmations);
        }
    }

    fn load_posted_affirmations() -> Vec<String>> {
        if let Ok(content) = fs::read_to_string("posted_texts.txt") {
            content.lines().map(|line| line.to_string()).collect()
        } else {
            Vec::new()
        }
        
    }
}

fn save_posted_affirmations(posted ffirmations: &[String]) {
    let content = posted_affirmations.join("\n");
    fs::write("posted_texts.txt", content).unwrap();

}
