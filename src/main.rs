use std::thread;
use std::thread::Thread;
use std::time::Duration;

fn main() {
    for i in 0..10 {
        println!("Hello, world! with times:{}",i);
        thread::sleep(Duration::from_secs(1));
    }
}
