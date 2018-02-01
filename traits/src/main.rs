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

    notify(&tweet);
    notify2(&tweet);
}

use std::fmt::Display;

// Any time you want to use behavior defined by a trait on a generic, 
// you need to specify that trait in the generic type parameter’s type bounds.

pub fn notify<T: Summarizable + Display>(item: &T) { // Trait bound restricting the possible types of T
    println!("Breaking news from {}: {}", item.author_summary(), item);
}

pub fn notify2<T>(item: &T) where T : Summarizable + Display { // We can also define bounds with 'where' for clarity
    println!("Breaking news from {}: {}", item.author_summary(), item);
}

// Trait bounds can also we used to filter generics on method implementations

struct Point<T> {
    x: T,
    y: T
}

impl<T: std::fmt::Display> Point<T> { // Trait bound restricting the print method to Points of some type implementing the Display trait
    fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

// We can also conditionally implement a trait for any type that implements a trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations, and are extensively used in the Rust standard library.

trait SomeTrait {
    fn hi(&self);
}

impl<T: std::fmt::Display> SomeTrait for T { // Implement the SomeTrait trait for all types implementing the trait Display
    fn hi(&self) {
        println!("{}", self);
    }
}

