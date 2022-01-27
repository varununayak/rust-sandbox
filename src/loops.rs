pub fn run() {
    let mut count = 0;
    
    // Infinite loop
    loop {
        count += 1;
        println!("Count at {}", count);
        if count == 20 {
            break;
        }
    }

    // While loop (Fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // For range
    for count in 0..100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
    }
}