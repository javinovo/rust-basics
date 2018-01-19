#[allow(dead_code)]

// Concepts: trait, trait bound, blanket implementation

// Trait definitions are a way to group method signatures together 
// in order to define a set of behaviors necessary to accomplish some purpose.

mod lib; // Declares a module in either lib.rs or lib\mod.rs
use lib::*; // Glob operator: import everything into scope

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    notify(tweet);

    // Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations, and are extensively used in the Rust standard library.
}

use std::fmt::Display;

// Any time you want to use behavior defined by a trait on a generic, 
// you need to specify that trait in the generic type parameter’s type bounds.

pub fn notify<T: Summarizable + Display>(item: T) { // Trait bound restricting the possible types of T
    println!("Breaking news from {}: {}", item.author_summary(), item);
}

pub fn notify2<T>(item: T) 
    where T : Summarizable + Display{ // We can define bounds with 'where' for clarity
    println!("Breaking news from {}: {}", item.author_summary(), item);
}