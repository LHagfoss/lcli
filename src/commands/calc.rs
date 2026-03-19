pub fn add(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

pub fn subtract(a: i32, b: i32) {
    println!("{} - {} = {}", a, b, a - b);
}

pub fn multiply(a: i32, b: i32) {
    println!("{} * {} = {}", a, b, a * b);
}

pub fn divide(a: i32, b: i32) {
    if a | b == 0 {
        println!("Cannot divide by zero.");
        return;
    }

    println!("{} / {} = {}", a, b, a / b);
}