use std::time::{Duration, Instant};

pub fn handle_spam_counter_command(content: &str, counter: i32) {
    if counter <= 0 {
        println!("Iteration count must be a positive integer.");
        return;
    }

    for _ in 0..counter {
        println!("{}", content);
    }
}

pub fn handle_spam_duration_command(content: &str, duration: i32) {
    if duration <= 0 {
        println!("Duration must be a positive integer.");
        return;
    }

    let duration_secs = Duration::from_secs(duration as u64);
    let start = Instant::now();

    while start.elapsed() < duration_secs {
        println!("{}", content);
    }
}