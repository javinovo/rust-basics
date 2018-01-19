#[allow(unused_variables)]

// Concepts: struct, impl, self, field, method, associate function, unit-like struct, generics, monomorphization

#[derive(Debug)] // Allows println!("{:?}")
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { 
    // Methods: instance's function: rect1.area(); 
    // Equivalent to: (&rect1).area(); <- Automatic referencing thanks to "&self"
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }    

    fn print(&self) {
        println!("Area of {:?}: {}", self, self.area());
    }
}

impl Rectangle { // Multiple impl blocks are allowed

    fn ctor(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height } // Field init shorthand: no need for the field name if it's the same
    }

    fn square(size: u32) -> Rectangle { // Associated function (eg. constructor): Rectangle::square(3);
        Rectangle { width: size, height: size }
    }
}

fn main()
{
    // Get instance from associated function
    let rect1 = Rectangle::ctor(100, 100); 
    let rect2 = Rectangle::square(50); 

    let rect3 = Rectangle { width: 100, ..rect1 }; // Instantiate copying the other fields from rect1

    for r in [&rect1, &rect2, &rect3].iter() {
        r.print();
    }

    println!("Fits: {}", rect1.can_hold(&rect2));


    // Field names are optional. 
    // Even the fields themselves: those structs are called unit-like structs which are useful to implement traits without data

    #[derive(Debug)] // Allows println!("{:?}")
    struct Point(i32, i32); 

    let p = Point(1, 2);
    let Point { 0: x, 1: y } = p; // Supports destructuring
    println!("{:?}, 0: {}, 1: {}, x: {}, y: {}", p, p.0, p.1, x, y);


    // Generics allow to define structure and methods supporting several types.
    // To avoid impacting performance, the compiler performs monomorphization: it  generates the concrete types by analyzing usage
    
    let rect = Rect::new(2, 3);
    println!("Area of generic rect: {}", rect.area());

    let string_rect = Rect::new(String::from("2"), String::from("3"));
    //generic_rect.area(); // error[E0599]: no method named `area` found for type `Rect<std::string::String>` in the current scope
}

struct Rect<T> {
    width: T,
    height: T
}

impl<T> Rect<T> {
    fn new(width: T, height: T) -> Rect<T> {
        Rect { width, height }
    }
}

impl Rect<i32> { // Methods may be implemented only for certain type parameters
    fn area(&self) -> i32 {
        self.width * self.height
    }
}