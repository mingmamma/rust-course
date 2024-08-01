// #![feature(noop_waker)]
#![allow(dead_code, unused)]
use std::process;
use std::future::Future;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{self, Context, Poll, Waker};
use std::ops::DerefMut;
use std::pin::Pin;

use futures::{self, FutureExt};
// use std::time::Duration;

// Arc<Mutex<dyn Future<Output = ()> + Unpin>> is actually Arc<Mutex<dyn Future<Output = ()> + Unpin + 'static>> ?!
// type TaskAkaRefCntPtrToFuture = Arc<Mutex<dyn Future<Output = ()> + Unpin>>;

type TaskAkaRefCntPtrToFuture = Pin<Box<dyn Future<Output = ()>>>;

struct MyMiniTokio {
    task_queue_tx: Sender<TaskAkaRefCntPtrToFuture>,
    task_queue_rx: Receiver<TaskAkaRefCntPtrToFuture>,
}

impl MyMiniTokio {
    pub fn new() -> Self {
        let (task_queue_tx, task_queue_rx) = mpsc::channel::<TaskAkaRefCntPtrToFuture>();
        Self {
            task_queue_tx,
            task_queue_rx,
        }
    }

    pub fn run(&self) -> () {
        loop {
            match self.task_queue_rx.recv() {
                Ok(mut a_task) => {
                    
                    // a_task: Arc<Mutex<dyn Future<Output = ()> + Unpin>>
                    // {
                    //     let mut cx: Context = Context::from_waker(Waker::noop());
                    //     let _ = a_task.lock().expect("acquired a poisoned lock").deref_mut().poll_unpin(&mut cx);
                    // }
                    
                    // a_task: Pin<Box<dyn Future<Output = ()>>
                    {
                        let waker: Waker = futures::task::noop_waker();
                        let mut cx: Context = Context::from_waker(&waker);

                        if a_task.as_mut().poll(&mut cx).is_pending() {
                            self.task_queue_tx.send(a_task);
                        }
                    }
                },
                Err(_) => {
                    break
                }
            }
        }
        // about to return from the current method ONLY IF having broken out of
        // the previous loop 
        ()
    }

    // alternative styple of trait bound declaration
    // pub fn spawn(&self, fut: impl Future<Output = ()> + Unpin + 'static) {
    pub fn spawn<F>(&self, fut: F) -> () 
    where F: Future<Output = ()> + Unpin + 'static {
        // type TaskAkaRefCntPtrToFuture = Arc<Mutex<dyn Future<Output = ()> + Unpin>>;
        // let task_as_arc_wrapped_fut: TaskAkaRefCntPtrToFuture = Arc::new(Mutex::new(fut));

        let task_as_arc_wrapped_fut: TaskAkaRefCntPtrToFuture = Box::pin(fut);
        self.task_queue_tx
            .send(task_as_arc_wrapped_fut)
            .expect("failed to submit input future as task to runtime for execution");
    }
}

// struct RuntimeEntry {
//     runtime_entry: Option<u32>,
// }

fn main() {
    // thread_local! {}
    // static RUNTIMEENTRY: RuntimeEntry = RuntimeEntry {
    //     runtime_entry: None,
    // };

    let my_mini_tokio_rt = MyMiniTokio::new();

    // my_mini_tokio_rt.spawn(async {
        
    //     // spawn(async {
    //     //     delay_text(Duration::from_millis(666)).await;
    //     //     println!("what is the question?");
    //     // });

    //     // let text = delay_text(Duration::from_millis(66)).await;
    //     // println!("{}", text);

        // temp workaround for shutdown
        process::exit(0);
    // });

    my_mini_tokio_rt.run();
}