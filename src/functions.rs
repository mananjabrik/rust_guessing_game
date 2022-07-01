// function - used to store block of code for reuse

pub fn run() {
    greeeting("hai", "manan");

    // Bind funtion value to variale
    let get_sum = add(5, 4);
    println!("sum: {}", get_sum);

    // CLosure
    let n3: i32 = 10;
    let add_n3 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("add_n3: {}", add_n3(3, 5));
}

fn greeeting(greeting: &str, name: &str) {
    println!("{} {}, nice to meet you", greeting, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
