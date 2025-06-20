use chrono::Local;
use owo_colors::OwoColorize;
use serde::Serialize;

#[derive(Serialize)]
struct TimeOutput {
    local_time: String,
    local_timezone: String,
}

pub fn handle_time_command(is_json: bool) {
    let now = Local::now();
    if is_json {
        let output = TimeOutput {
            local_time: now.to_rfc3339(),
            local_timezone: now.format("%Z").to_string(),
        };
        println!("{}", serde_json::to_string(&output).unwrap());
    } else {
        println!(
            "Current Time: {}",
            now.format("%Y-%m-%d %H:%M:%S %Z").bright_blue()
        );
    }
}