// primitive str = Immutable fixed-length string somewhere in memory
// Strinng = growable, heap-allocated buffer of bytes

pub fn run() {
    let mut hello = String::from("Hello");

    //get length
    println!("Length: {}", hello.len());

    //push char
    hello.push('W'); // push this for add char to end of string

    // push string
    hello.push_str(", world!"); // push_str for adding a string to the end of a string

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("Is empty: {}", hello.is_empty());

    //contains
    println!("Contains 'world'?: {}", hello.contains("world"));

    //replace
    println!("Replace: {}", hello.replace("world", "everyone"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2, s.len());
    println!("{}", s);

    println!("{}", hello);
}
