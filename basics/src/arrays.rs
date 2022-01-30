// Vector - growable arrays

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // Re assign
    numbers[2] = 20;
    println!("{:?}", numbers);
 
    // Get single value
    println!("{}", numbers[0]);

    // Arrays are stack allocated
    println!("Array at occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}