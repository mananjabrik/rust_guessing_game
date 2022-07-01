// Conditional - used to check the condition of something act

pub fn run() {
    let age: i32 = 18;
    let check_id: bool = true;
    let know_person_of_age: bool = true;
    // if age >= 21 {
    //     println!("Bartender: What would you like to drink?");
    // }
    // println!("halo bossku")
    if age >= 21 && check_id || know_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("you need more age")
    } else {
        println!("I need to see your id")
    }

    //short hand IF
    let is_off: bool = if age >= 21 { true } else { false };
    println!("is_off: {}", is_off);
}
