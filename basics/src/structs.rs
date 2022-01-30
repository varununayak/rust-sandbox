
// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

/// Tuple Struct
struct Color_(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}

pub fn run() {
   let mut col = Color{red:255,green:0,blue:0};
   col.red = 200;
   println!("Color: {} {} {}", col.red, col.green, col.blue);

   let mut col_ = Color_(255,0,0);
   col_.0 = 200;
   println!("Color: {} {} {}", col_.0, col_.1, col_.2);

   let mut p = Person::new("Varun", "Nayak");
   println!("Name: {} {}", p.first_name, p.last_name);
   println!("Name: {}", p.full_name());

   p.set_last_name("Federer");
   println!("Name: {}", p.full_name());
   
   println!("Tuple: {:?}", p.to_tuple());
}