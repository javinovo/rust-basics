#[allow(dead_code)]

// Conceptos: borrow checker, lifetime annotation, static lifetime, lifetime elision

// Every reference has a lifetime. Some are either implicit or inferred but some times we have to explicitly tell the compiler. 
// The main aim of lifetimes is to prevent dangling references, which will cause a program to reference data other than the data 
// we’re intending to reference.
// The part of the compiler called the borrow checker compares scopes to determine that all borrows are valid.

// Ultimately, lifetime syntax is about connecting the lifetimes of various arguments and return values of functions. 
// Once they’re connected, Rust has enough information to allow memory-safe operations 
// and disallow operations that would create dangling pointers or otherwise violate memory safety.

// Lifetime annotations don’t change how long any of the references involved live.
// What lifetime annotations do is relate the lifetimes of multiple references to each other.
// We are saying that any values that do not adhere to this contract should be rejected by the borrow checker.

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result = longest_with_an_announcement(&string1, string2, "some announcement");
    println!("The longest string is {}", result);

     // The 'static lifetime is the entire duration of the program. 
     // All string literals have the 'static lifetime, which we can choose to annotate as follows:
     let s: &'static str = "I have a static lifetime.";
}

// fn longest(x: &str, y: &str) -> &str { error[E0106]: missing lifetime specifier
// help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // we declare that all references must have the same lifetime
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This struct holds a reference so its definition needs a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision rules are patterns known by the compiler that allows us to omit otherwise required lifetime annotations

impl<'a> ImportantExcerpt<'a> { // We must keep specifying the lifetime annotations
    // By elision rules, each parameter gets its own lifetime and the return the same as self
    fn announce_and_return_part(&self, announcement: &str) -> &str { 
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

// Because lifetimes are a type of generic, the declarations of both the lifetime parameter 'a and the generic type parameter T 
// go in the same list within the angle brackets after the function name.
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}