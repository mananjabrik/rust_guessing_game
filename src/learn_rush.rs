pub fn test() {
    // print to console
    println!("Hello, world! from file learn_rush.rs");

    // basic formatting
    println!("{} days", 31);

    // Positional arguments
    println!("halo {0} apakah {2} yg kamu {1} ?", "manan", "suka", "rust");

    // Named arguments
    println!("{first} {last}", first = "halo", last = "manan");

    //placeholder traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
