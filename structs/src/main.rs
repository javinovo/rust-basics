
// Concepts: struct, impl, self, field, method, associate function, 

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
}