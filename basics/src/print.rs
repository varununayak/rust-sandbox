pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{}", 1);
    println!("{} is from {}", "Varun", "Stanford");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes {2}",
        "Varun", "Stanford", "Coding"
    );

    // Named Args
    println!(
        "{name} likes to play {sport}",
        name = "Varun",
        sport = "Tennis"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10 ,10);

    // Placeholder for debug trait
    println!("{:?}",(12, true, "hello")); // Tuple
    
    // Basic math
    println!("10 + 10 = {}", 10 + 10)
}
