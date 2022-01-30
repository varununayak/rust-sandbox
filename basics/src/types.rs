/*
Primitive types
Integers: u8, i8, .... u128, i128
Floats: f32, f64
Boolean
Characters
Tuples
Arrays
*/

// Rust is a statically typed language, however the compiler can infer type

pub fn run() {
    // Default is i32
    let x = 1;
    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max f32: {}", std::f32::MAX);

    let is_active = true;
    println!("{:?}", (x, y, is_active));
}
