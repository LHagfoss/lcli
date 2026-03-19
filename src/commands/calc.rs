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

pub fn handle_square_root_command(a: f64) {
    if a < 0.0 {
        println!("Cannot calculate square root of a negative number.");
        return;
    }

    let result = sqrt_newton(a);

    if is_whole_number(result) {
        println!("Square root of {} is {}", a, result as u64);
    } else {
        println!("Square root of {} is {:.6}", a, result);
    }
}

fn sqrt_newton(value: f64) -> f64 {
    if value == 0.0 {
        return 0.0;
    }

    let mut x = if value >= 1.0 { value } else { 1.0 };
    let tolerance = 1e-12;
    let max_iterations = 100;

    for _ in 0..max_iterations {
        let next = 0.5 * (x + value / x);
        if (next - x).abs() < tolerance {
            return next;
        }
        x = next;
    }

    x
}

fn is_whole_number(value: f64) -> bool {
    (value - value.round()).abs() < 1e-9
}
