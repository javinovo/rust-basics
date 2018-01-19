
#[allow(dead_code)] // Covers both unused variables and enum variants

// Concepts: enum, match, if let

// This could also be defined with 4 structs
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {        
        match self {  
            &Message::Write(ref msg) => println!("{}", msg),
            _ => () // default: do nothing
        }

        // When we only want a single match we can simple use 'if let'. 
        // However, exhaustive checking is not enforced anymore
        if let &Message::Write(ref msg) = self { // ref matches a value and yields a reference to it while & matches a references and yield its value
            println!("{}", msg);
        } else {
            // Do nothing (could get rid of this else)
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Just like structs, enums can be generic: Result and Option are examples
}
