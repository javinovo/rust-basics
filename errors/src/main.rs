#[allow(dead_code)]

// Concepts: panic, abort, ? operator

fn main() {
    // There are no exceptions, only recoverable or unrecoverable errors (panics)
    // It is a good practice to propagate errors as much as possible, thus deferring the consideration of unrecoverability
    // However, an API may consider an improper (eg. invalid parameters) use by some external caller unrecoverable to enhace robustness
    
    // panic! prints a failure message, unwinds and cleans up the stack, and then quits
    // Abort mode: unwinding and cleaning can be disabled and delegated to the operative system

    // Result provides unwrap and expect mthods returning the value in the Ok or panicking if Err
    let username = read_username_from_file("hello.txt")
        .expect("Couldn't read the username"); // expect allows to specify a panic message
}

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// More compact version with the ? operator 
// It only works on types implementing the Try trait like Result
// It uses the From trait to convert between error types
fn read_username_from_file_with_shortcut(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?; // Assigns the value of Ok or returns Err
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // Assigns the value of Ok or returns Err
    Ok(s) // ? yields the value inside Ok so we can't return that: we must wrap it in a Result
}

// We can chain it
fn read_username_from_file_even_shorter(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}