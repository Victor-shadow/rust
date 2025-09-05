use std::thread;
use std::time::Duration;
use futures::stream::Stream; // Needed for the return type
use futures::StreamExt;      // Optional: for combinators
use trpl::{channel, ReceiverStream}; // Assuming `trpl` re-exports these

fn get_intervals() -> impl Stream<Item = u32> + Unpin {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {}: {:?}", count, send_error);
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn main() {
    let mut stream = get_intervals();

    // Example usage: consume first 10 values
    let future = async {
        for _ in 0..10 {
            if let Some(val) = stream.next().await {
                println!("Interval: {}", val);
            }
        }
    };

    trpl::run(future);
}