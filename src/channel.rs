use std::collections::VecDeque;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;

fn main() {
    let mut channel_queue = Mutex::new(VecDeque::<u32>::new());

    thread::scope(|s| {
        // receiver thread
        let recv_thread = s.spawn(|| {
            loop{
                // makes sure the lock acquired wouldn't need to be held longer than for the purpuse of obtaining
                // the return value of pop_front call
                let opt_item = channel_queue.lock().expect("acquired a poisoned lock").pop_front();
                if let Some(item) = opt_item {
                    dbg!(item);
                } else {
                    thread::park();
                }
            }
        });

        // sender thread
        for i in 0.. {
            channel_queue.lock().expect("acquired a poisoned lock").push_back(i);
            recv_thread.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}