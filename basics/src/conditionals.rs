pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let obviously_old = true;

    if age >= 21 && check_id || obviously_old {
        println!("Bartender: What drink do you want?");
    } else if age < 21 && check_id {
        println!("Bartender: You are too young");
    } else {
        println!("Bartender: Show your ID");
    }
    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
