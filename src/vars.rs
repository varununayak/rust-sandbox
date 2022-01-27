// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Varun";
    println!("My name is {}", name);
    let age = 37;
    // let mut age = 37; // mutable 
    println!("My age is {}", age);

    // Define constant
    const ID: i32 = 001; // explicit type
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Varun", 25);
    println!("{} is {}", my_name, my_age);
}