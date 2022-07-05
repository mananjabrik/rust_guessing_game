// traditioan struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, las: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: las.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to topple
    fn to_tuple(&self) -> (String, String) {
        (self.first_name.clone(), self.last_name.clone())
    }
}

pub fn run() {
    // let mut c: Color = Color {
    //     red: 255,
    //     green: 10,
    //     blue: 10,
    // };

    // c.green = 20;

    // println!("color red: {} green: {} blue: {}", c.red, c.green, c.blue);

    // let mut c = Color(255, 10, 10);
    // c.1 = 20;

    // println!("color red: {} green: {} blue: {}", c.0, c.1, c.2);

    let mut p = Person::new("John", "Jabrik");

    p.set_last_name("Smith");

    println!("{}", p.full_name());
    println!("{:?}", p.to_tuple());
}
