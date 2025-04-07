use chrono::{Local, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::thread;

fn main() {
    let expression = "0/5 * * * * *";
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    loop {
        let now = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            println!(
                "Running every 5 seconds. Current time: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
}