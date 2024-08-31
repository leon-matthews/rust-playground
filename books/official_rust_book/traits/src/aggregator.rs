
use std::fmt::{self, Display};


// Traits //////////////////////////////
pub trait Summary {
    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }

    fn summarise_author(&self) -> String;
}


// NewsArticle /////////////////////////
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        self.author.clone()
    }
}


// Tweet ///////////////////////////////
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}: {}", self.username, self.content)

    }
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarise_author(&self) -> String {
        format!("@{}", self.username.clone())
    }
}
