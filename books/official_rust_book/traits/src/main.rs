#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Display;

mod aggregator;
use aggregator::{NewsArticle, Summary, Tweet};


fn main() {
    let article = make_article();
    notify(&article);

    let tweet = make_tweet();
    notify2(&tweet);
    notify3(&tweet);
    notify4(&tweet);
}


/// Trait as parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise());
}


/// The above is syntax sugar for this
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news2! {}", item.summarise());
}


/// But it can start to get too verbose
fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news3! {}", item.summarise());
}


/// So we can break out the `where` syntax
fn notify4<T>(item: &T)
where
    T: Summary + Display
{
    println!("Breaking news4! {}", item.summarise());
}


fn make_article() -> NewsArticle {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are good."),
    }
}


fn make_tweet() -> Tweet {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
