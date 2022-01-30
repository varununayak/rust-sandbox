// Primitive str = immutable fixed length string
// String = growable, heap allocated data struct

pub fn run() {
    let hello = "Hello"; // immutable
    let mut hello2 = String::from("Hello "); // growable

    println!("{}", hello);

    // Get length 
    println!("Length: {}", hello.len());

    // Modify
    hello2.push('W');
    hello2.push_str("orld!");

    println!("{}", hello2);

    println!("Capacity: {}", hello2.capacity());
    println!("Is Empty: {}", hello2.is_empty());
    println!("Contains 'World': {}", hello2.contains("World"));
    println!("Replaced: {}", hello2.replace("World", "There"));

    // Loop through
    for word in hello2.split_whitespace() {
        println!("{}", word)
    }

    assert_eq!(12, hello2.len());
}