use core::time;
use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "Hi",
            "From",
            "the",
            "thread"
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    };
}