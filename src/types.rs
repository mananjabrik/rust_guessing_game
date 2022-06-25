/*
Primitive Types---
Unsign Integers: u8, u16, u32, u64, u128, usize (number of bits they take in memory)
Signed Integers: i8, i16, i32, i64, i128, isize (number of bits they take in memory)
Floats: f32, f64
Booleans: bool
Characters: char
Tuples: Tuple structs can be used to group together multiple values.
Arrays: fixed size and fixed type.
Vectors: growable and mutable.
*/

// Rust is a block-scoped language, so variables are not available outside of their scope.
// Rust is a compile-time language, so we can't use variable before we declare it.
// Rust is a dynamically typed language, so we can't use variable before we assign it.
// Rust is a type-safe language, so we can't use variable before we assign it with correct type.
// Rust is a statically typed language, so we can't use variable before we assign it with correct type.

pub fn run() {
    //Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45378789787;

    // Find max size of type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Character
    let a: char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a, face));
}
