#[allow(dead_code)]

fn main() {
    // Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    // They capture their environment and thus can access variables from the scope in which they are defined.
    
    // Type annotations are optional and the syntax can be more compact than regular function definitions:
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // We must use v3 and v4 or the compiler won't be able to infer their types
    add_one_v3(3);
    add_one_v4(3);

    // All closures implement one of the traits Fn, FnMut, or FnOnce, depending on how they capture values.
    // -FnOnce: takes ownership and thus can only be called once
    // -Fn: borrows
    // -FnMut: borrows mutably so it can change the environment
    // Normally, the compiler infers which one based on how variables are used inside the closure.
    // Ownership can be forced by using the 'move' keyword
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    //  You may know this pattern as memoization or lazy evaluation.
    use std::thread;
    use std::time::Duration;

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    expensive_result.value(10);


    // Custom iterator example

    let sum: u32 = Counter::new(5).zip(Counter::new(5).skip(1))
                                  .map(|(a, b)| a * b)
                                  .filter(|x| x % 3 == 0)
                                  .sum();
    assert_eq!(18, sum);    


    // Functions as arguments and return types

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    println!("{}", do_twice(add_one, 2)); // pass the function declared above as argument
    println!("{}", do_twice(returns_add_function(), 2)); // pass the function returned from the function as argument
    //println!("{}", do_twice(returns_add_closure(), 2));  // error[E0308]: mismatched types: expected fn pointer, found struct `std::boxed::Box`
    println!("{}", do_twice(returns_add_closure_as_function(), 2));
}

// We can use trait bounds on generics for closures.
// As a matter of fact, regular functions implement those traits too so we can use them in the same way.
// (However, functions can not capture values from the environment)
struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}



struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < self.max {
            Some(self.count)
        } else {
            None
        }
    }
}


// fn in the arguments is a function pointer. It is a type (unlike closures which are traits: Fn, FnMut, FnOnce)
// Function pointers implement those 3 closure traits so they can be passed to functions expecting a closure. 
// However, closures may not be passed to functions expecting a function pointer. 
// Thus, it is more general to use closures and should be preferred
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { 
    f(arg) + f(arg)
}

fn returns_add_function() -> fn(i32) -> i32 {
    fn add(x: i32) -> i32 {
        x + 1
    }

    add
}

// Closures may be returned but since they are represented by traits they must be boxed in order to have a known size 
fn returns_add_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Now we are returning a function pointer to a closure: easier to read
fn returns_add_closure_as_function() -> fn(i32) -> i32 {
    |x| x +1 
}