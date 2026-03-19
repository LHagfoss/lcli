pub fn handle_add_command(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

pub fn handle_subtract_command(a: i32, b: i32) {
    println!("{} - {} = {}", a, b, a - b);
}

pub fn handle_multiply_command(a: i32, b: i32) {
    println!("{} * {} = {}", a, b, a * b);
}

pub fn handle_divide_command(a: i32, b: i32) {
    if b == 0 {
        println!("Cannot divide by zero.");
        return;
    }

    println!("{} / {} = {}", a, b, a / b);
}

pub fn handle_square_root_command(a: u64) {
    let result = integer_sqrt(a);

    println!("Square root of {} is {}", a, result);
}

fn integer_sqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let n128 = n as u128;
    let mut x0 = n128;
    let mut x1 = (x0 + n128 / x0) / 2;

    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + n128 / x0) / 2;
    }

    x0 as u64
}
