use std::{sync, thread};

fn main() {
    let (tx, rx) = sync::mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    print!("Received: {}", received);
}