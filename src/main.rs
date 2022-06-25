mod learn_rush;

//use library called rand to generate random number
// use rand::Rng;
// use library standart std = standard, library cmp = compare, Oradering = Ordering
// use std::cmp::Ordering;
//use librari io = input output, to print to screen
// use std::io;
fn main() {
    // print on screen
    // println!("Guess the number!");
    // // create immutable variable, random number
    // let secret_number = rand::thread_rng().gen_range(1..101);
    // //loop until user guess the number
    // loop {
    //     println!("Please input your guess.");
    //     // create mutable variable, user input
    //     let mut guess = String::new();
    //     //user input the guess
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     // convert string to integer
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     // print the guess from input
    //     println!("You guessed: {}", guess);
    //     // compare the guess with secret number
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    learn_rush::test();
}
// mutable is used to change the value of variable
// immutable is used to read the value of variable
// by default, all variable in rust is immutable
