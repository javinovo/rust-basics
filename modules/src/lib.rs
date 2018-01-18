// Declare top level modules as public
pub mod client;
pub mod network;

#[cfg(test)] // Run with 'cargo test'
mod tests {
    #[test]
    fn it_works() {
        // 'tests' is a submodule of 'modules' thus we need to refer to the sibling module 'client'
        ::client::connect(); // Absolute path (:: <=> root)
        super::client::connect(); // Relative path
    }
}
