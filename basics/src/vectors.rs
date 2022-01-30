// Vector - growable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    // Re assign
    numbers[2] = 20;
    println!("{:?}", numbers);
    
    // Add
    numbers.push(10);
    println!("{:?}", numbers);
    
    // Get single value
    println!("{}", numbers[0]);

    // Vectors are stack allocated
    println!("vector at occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Vec Slice: {:?}", slice);

    // Loop
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and Mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);

}