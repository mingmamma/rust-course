use std::{thread, time};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("spawned thread: {} ", i);
            thread::sleep(time::Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("Main thread: {} ", i);
        thread::sleep(time::Duration::from_millis(1));
    }

    handle.join().unwrap();
}