use std::{time::Duration, thread::sleep, collections::HashMap};
use rust_twitter_bot_lib::{TwitterBot, Tweet};

fn good_bot_bot_last_tweet(bot: &TwitterBot, since_id: Option<&str>) -> Option<Tweet> {
    let mut params = HashMap::new();

    params.insert("result_type", "recent");
    params.insert("count", "1");
    if let Some(since_id) = since_id {
        params.insert("since_id", since_id);
    }

    if let Ok(last_tweet) = bot.get_tweets_query("from:GoodBotBot1 to:XKCDAltTextBot \"Good Bot.\"", Some(params)) {
        return last_tweet.into_iter().next();
    }
    eprintln!("Error: GoodGoodBotBotBot Couldn't get tweets :'(");
    None
}

fn reply_to_good_bot_bot(bot: &TwitterBot, id: &str) {
    let mut params = HashMap::new();
    params.insert("in_reply_to_status_id", id);

    if bot.tweet("@GoodBotBot1 Good Good bot bot.", Some(params)).is_err() {
        eprintln!("Error: GoodGoodBotBotBot Couldn't tweet :'(");
    }
}

fn main() {
    let bot = TwitterBot::new()
        .consumer_key(env!("CONSUMER_KEY"))
        .consumer_secret_key(env!("CONSUMER_SECRET_KEY"))
        .access_token(env!("ACCESS_TOKEN"))
        .secret_access_token(env!("SECRET_ACCESS_TOKEN"));

    println!("GoodGoodBotBotBot launched :D");
    
    let mut since_id = good_bot_bot_last_tweet(&bot, None).map(|elem| elem.id().to_string());

    if let Some(since_id) = since_id.clone() {
        println!("Find latest tweet: {}", since_id);
    }

    loop {
        sleep(Duration::from_secs(15));
        if let Some(new_tweet_id) = good_bot_bot_last_tweet(&bot, (&since_id).as_deref()).map(|elem| elem.id().to_string()) {
            println!("New Tweet !");
            reply_to_good_bot_bot(&bot, &new_tweet_id);
            since_id = Some(new_tweet_id);
        }
    }
}
