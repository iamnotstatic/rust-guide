use chrono::prelude::*;
use humantime::format_duration;
use std::time::Duration;

fn main() {
    let val: Duration = Duration::from_secs(10000);
    println!("{}", format_duration(val));

    let utc: DateTime<Local> = Local::now();
    println!("{}", utc.format("%Y-%m-%d %H:%M:%S"));
}
