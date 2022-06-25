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
    // by default is "i32"
    let x = 5;
    // by default is "f64"
    let y = 2.5;
    // by default is "bool"
    let z = true;
    // by default is "char"
    let c = 'a';
    // by default is "String"
    let name = String::from("Manan");
    // by default is "Tuple"
    let (my_name, my_age) = ("Manan", 25);
    // by default is "Array"
    let arr = [1, 2, 3, 4, 5];
    // by default is "Vector"
    let vec = vec![1, 2, 3, 4, 5];
    println!("{}", x);
}
