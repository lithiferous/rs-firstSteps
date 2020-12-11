mod lib;
use crate::lib::{Summary, notify};

fn main()
{
    let tweet = lib::Tweet {
        username: String::from("horse_shit"),
        content:  String::from("I dunno, honey"),
        reply: false,
        retweet: false,
    };
    let art = lib::NewsArt {
        author: String::from("my_ass"),
        content:  String::from("I dunno, prob 15 mm"),
        headline: String::from("warn!!"),
        location: String::from("New guinea"),
    };

    println!("new msg: {}", tweet.sumup_author(),);
    notify(tweet, art);
}
