
extern crate modules;

use modules as m; // Alias
use m::client; // Brings server into the namespace (not its content)

// Although 'crate new modules' was used to generate this project, which implies a library, 
// by adding main.rs 'cargo run' is enabled:  it is possible to have both a binary and library crates in the same project 
// and the library one may be used from other projects
fn main() {    
    
    { // New scope 
        use modules::network::*; // Glob operator: brings all the items from the namespace into scope
        connect(); // modules::network::
        server::connect(); // modules::network::
    }

    // connect(); //error[E0425]: cannot find function `connect` in this scope
    client::connect(); // Since we imported 'client'
    ::client::connect(); // Root
}