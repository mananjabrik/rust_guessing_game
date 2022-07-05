// traditioan struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct

struct Color(u8, u8, u8);

pub fn run() {
    // let mut c: Color = Color {
    //     red: 255,
    //     green: 10,
    //     blue: 10,
    // };

    // c.green = 20;

    // println!("color red: {} green: {} blue: {}", c.red, c.green, c.blue);

    let mut c = Color(255, 10, 10);
    c.1 = 20;

    println!("color red: {} green: {} blue: {}", c.0, c.1, c.2);
}
