pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("number: {}", count);
        if count == 20 {
            break;
        }
    }
}
