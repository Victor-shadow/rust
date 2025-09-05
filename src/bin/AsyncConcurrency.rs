use async_std::task;
use std::time::Duration;
use futures::{select, FutureExt, pin_mut};

async fn perform_work() {
    task::sleep(Duration::from_millis(200)).await;
}

fn main() {
    task::block_on(async {
        let timeout = task::sleep(Duration::from_secs(10)).fuse();
        pin_mut!(timeout);

        loop {
            let tick = task::sleep(Duration::from_secs(1)).fuse();
            pin_mut!(tick);

            select! {
                _ = tick => {
                    println!("Performing periodic work!");
                    perform_work().await;
                }

                _ = timeout => {
                    println!("Timeout reached");
                    break;
                }
            }
        }
    });
}