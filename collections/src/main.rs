#[allow(unused_variables)]

// Concepts: vector, deference operator, string, UTF8 encoding, hash map

fn main() {
    let v1: Vec<i32> = Vec::new(); // Type can't be inferred because we don't store anything

    let mut v2 = vec![1, 2, 3]; // Vec<i32> inferred
    v2.push(4);

    {
        let third: &i32 = &v2[2]; // Unsafe access
        let non_existent: Option<&i32> = v2.get(100); // Safe access
        if let None = non_existent {
            println!("Out of bounds");
        }

        // v2.push(5); error[E0502]: cannot borrow `v2` as mutable because it is also borrowed as immutable
        // Adding might require allocating new memory and copying the old elements to the new space. 
        // In that case, the reference to the element would be invalid (deallocated memory)
    } // mutable borrow ends

    v2.push(5);

    for el in &mut v2 { // Borrow mutably but not move so that we can use it afterwards
        *el += 10; // Dereference operator: accesses the value in el
    }

    v2.push(16);

    for el in &v2 { // Borrow v2, otherwise we wouldn't be able to add afterwards
        println!("{}", el);
    }

    // Vectors can only store values of the same type but we can use enums or traits to get around this
    
    // Strings is a wrapper over a Vec<u8> (a vector of bytes) storing UTF8 encoded chars.
    // UTF8 encoding is an Unicode standard in which chars are encoded using one to four bytes.
    // It is designed to be backward-compatible with ASCII: 
    // the first 128 chars of Unicode correspond one-to-one with ASCII and are encoded in a single byte with the same binary value.
    // Thus, strings have three persectives: bytes, scalar values (chars) and grapheme clusters (letters, ie. visual representation) 
    let hindi = "नमस्ते";
    println!("{} has {} chars in {} bytes", hindi, hindi.chars().count(), hindi.bytes().count());

    // Indexing is not supported:
    // - Each char may or may not equal to a byte and thus indexing can not refer to bytes
    // - Also if we were to index by chars, indexing could not be O(1) (constant time) 
    // Instead, slices may be used: they are a view over contiguous bytes of the string. 
    println!("{}", &hindi[..3]); // First char (each char occupies 3 bytes)

    // The starting and ending indices refer to bytes which must correspond to a char boundary, otherwise a run-time error will occur
    // let slice = &hindi[0..2]; // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside 'न' (bytes 0..3) of `नमस्ते`', src\libcore\str\mod.rs:2235:4


    use std::collections::HashMap; // Not in the prelude

    let mut scores = HashMap::new(); // No macro helper

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);    

    // We can zip two vectors into a HashMap:
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip creates a vector of tuples and collect turns it into a HashMap
    // Since collect can output several structures, we must declare the desired one by annotation the result variable
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); 

    let first_team = &teams[0];
    println!("{}", first_team); 

    if let Some(value) = scores.get(first_team) {
        println!("{}", value);
    }
    
    scores.insert(first_team, &20); // Overwrite
    scores.entry(first_team).or_insert(&50); // Insert if doesn't have a value already: in this case does nothing

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Types with the Copy trait are copied into the hash map
    // Owned value are moved. If we insert references to values, they must be valid for at least as long as the hasp map is valid
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");    
    
    // map.insert(&field_name, &field_value); // error[E0597]: `field_value` does not live long enough    
    
    map.insert(field_name, field_value); // We are moving the values
    // println!("{}", field_name); // error[E0382]: use of moved value: `field_name`
}