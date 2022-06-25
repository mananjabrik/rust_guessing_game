// Variable hold primitive data or references to data.
// Varible are immutable by default.
// Variable can be mutable by using mut keyword.
// Rust is a block-scoped language, so variables are not available outside of their scope.

pub fn run() {
    // immutable variable
    let name = "Manan";
    // mutable variable
    let mut age = 25;
    println!("Hello, {} the name is immutable variable", name);
    age = 26;
    println!(
        "{} is {} years old the age is mutabble and assign with 26",
        name, age
    );

    // Define constant variable
    const ID: i32 = 001;
    println!("ID :{}", ID);

    //assign multiple variable at once
    let (my_name, my_age) = ("Manan", 25);
    println!("{} is {} years old", my_name, my_age);
}
