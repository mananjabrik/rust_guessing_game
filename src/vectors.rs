// vectors - Resizable array

pub fn run() {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", arr);

    // Reassigne value in array
    arr[0] = 10;

    arr.push(8);
    arr.push(9);

    println!("{:?}", arr);

    // Get singgle value
    println!("{}", arr[0]);

    // Get array length
    println!("arr length: {}", arr.len());

    // Arrays are stack allocated
    println!("arr occupies {} bytes", std::mem::size_of_val(&arr));

    // Get slice
    let slice: &[i32] = &arr[0..2];
    println!("{:?}", slice);

    // loop through vector
    for x in &arr {
        println!("number {}", x);
    }

    // loop & mutate vector
    for x in &mut arr {
        *x *= 2;
    }
    println!("{:?}", arr);

    // filter vector min and max number on array
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut sum: i32 = 0;
    for (index, x) in arr.iter().enumerate() {
        sum += *x;
        if *x > max {
            max = *x;
        }
        // min mustbe greater than 0
        if index == 0 || *x < min {
            min = *x;
        }
    }
    println!("max: {}, min: {}, sum {}", max, min, sum);
}
