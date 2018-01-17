#[allow(unused_variables)]

// Concepts: shadowing, mutability, copy, clone, move, ownership, borrowing, reference, slice
fn main() {
    let a = String::from("a value");
    let a = String::from("another value"); // Shadows the previous one
    println!("{}", a);

    let mut a = a; // Create 
    a.push_str(" edited");
    println!("{}", a);

    // Scalar and compound types composed only by scalar types have a known size an can go to the stack.
    // They can have the Copy trait.
    // - Copy: copies stack data from one variable to another.
    let x = 5;
    let y = x;
    println!("{}", x);

    // For types with unknown size, memory is allocated from the heap (eg. String, not string literals).
    // They can have to Drop trait.
    // - Move: the stack data is copied but the heap data is not: a pointer is used instead. 
    //      This ressembles a shallow copy. It renders the origin variable invalid to avoid double free errors:  
    //      the destination variable is now the owner of the heap data. A move operation is an ownership transfer.
    let b = a;
    // println!("{}", a); // error[E0382]: use of moved value: `a`

    // - Clone: the heap data is copied too, thus both variables are valid. This ressembles a deep copy.
    let c = b.clone();
    println!("{}", c);

    // Heap data is freed when the variable goes out of scope.
    // Passing a variable as a function parameter is a move operation. Thus, it transfers the ownership: 
    // the variable is no longer valid in the original scope and it will be freed when the function's scope ends. 
    // Returning it will transfer the ownership back.
    let (len, d) = calculate_length(c);
    println!("{}", d); // Ownership is back into this scope but a new variable has it (ie. c is invalid)

    // Ownership transfer can be avoided by using references with &. This is called borrowing: 
    // - The data won't be dropped after leaving the callee scope.
    // - The variable in the caller scope is still valid.
    println!("{} is the length of '{}'", length(&d), d); // Borrow it and use it right after

    // It is possible to alter borrowed data since references can be mutable using &mut.
    // There may be either several immutable references or one mutable reference at the same time.
    let mut e = String::from("text with length: "); // Must be mutable
    append_length(&mut e); // Mutably borrow it
    println!("{}", e); // Ownership is back

    // Ownership rules:
    // 1) Each value in Rust has a variable that’s called its owner.
    // 2) There can only be one owner at a time.
    // 3) When the owner goes out of scope, the value will be dropped.

    // Slices are data types that do not have ownership. They are views into a block of memory: a pointer and a length. 
    // They point to a data subset and prevent it from being dropped even if the owner variable goes out of scope.
    {
        let slice = &e[5..]; // May specify starting and/or ending index 
        println!("{}", slice);
    } // slice goes out of scope here, thus allowing the borrow e mutably afterwards

    // If we didn't use the previous scope: error[E0502]: cannot borrow `e` as mutable because it is also borrowed as immutable
    let mutable_slice = &mut e[..];
    println!("{}", mutable_slice);
}

fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s) // The order matters: inverting it would yield error[E0382]: use of moved value: `s`
}

fn length(s: &String) -> usize {
    s.len()
}

fn append_length(s: &mut String) {
    let length_string = s.len().to_string();
    s.push_str(&length_string); // push_str expects a borrowed string, thus the reference
}