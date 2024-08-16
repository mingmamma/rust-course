#![allow(unused)]
use stream_cancel::{StreamExt, Tripwire};
use tokio;
use tokio::stream;

// use tokio_stream::StreamExt;
use futures::prelude::*;

#[tokio::main]
async fn main() {
    let (trigger, tripwire) = Tripwire::new();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let listener_stream = tokio_stream::wrappers::TcpListenerStream::new(listener);
    let mut canlable_lis_str = listener_stream.take_until_if(tripwire);

    loop {
        let str_next_res_opt = canlable_lis_str.next().await;
        match str_next_res_opt {
            // while the stream of TcpListener still yields Some (stream), spawn a new tokio task
            // to split the stream and do echo copying
            Some(next_res) => {
                let mut next_stream = next_res.unwrap();
                tokio::spawn(async move {
                    let (str_r, str_w) = next_stream.split();
                    // let bytes_copied = tokio::io::copy(&mut str_r, &mut str_w);
                });
            },
            None => { break }
        }
    }
}
