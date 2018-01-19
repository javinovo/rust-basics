
pub trait Summarizable {
    fn author_summary(&self) -> String; 

    fn summary(&self) -> String { // Provides a default implementation which can be optionally overriden
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle { // No dependency on the trait
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String { // Compulsory
        format!("Author: {}", self.author)
    }

     // We keep the default summary implementation
}

pub struct Tweet { // No dependency on the trait
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet { // Implement the trait for the Tweet type
    fn author_summary(&self) -> String { // Compulsory
        format!("User: {}", self.username)
    }

    fn summary(&self) -> String { // Override the default implementation
        format!("{}: {}", self.username, self.content)
    }
}

// One restriction to note with trait implementations: 
// we may implement a trait on a type as long as either the trait or the type are local to our crate.
// In other words, we aren’t allowed to implement external traits on external types.

use std::fmt;

impl fmt::Display for Tweet {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Tweet from {}: {}", self.username, self.content)
    }
}