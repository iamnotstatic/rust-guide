use std::time::Duration;
use std::{thread, thread::sleep};

fn msg_hello() -> &'static str {
    sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let thread_a = thread::spawn(move || msg_hello());
    let thread_b = thread::spawn(move || msg_thread());
    let thread_c = thread::spawn(move || msg_excited());

    let thread_a = thread_a.join().expect("Failed to join thread a");
    let thread_b = thread_b.join().expect("Failed to join thread b");
    let thread_c = thread_c.join().expect("Failed to join thread c");

    println!("{}{}{}", thread_a, thread_b, thread_c);
}
