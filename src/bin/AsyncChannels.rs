use std::time::Duration;
use async_std::{channel, task};
use futures::join;

fn main() {
    task::block_on(async {
        let (tx, mut rx) = channel::unbounded();

        //spawn sender task
        let sender = task::spawn({
            let tx = tx.clone();
            async move {
                for i in 0..5 {
                    tx.send(i).await.unwrap();
                    task::sleep(Duration::from_millis(100)).await;
                }
            }
        });

        //spawn receiver task
        let receiver = task::spawn(async move {
            let mut rx = rx; //move rx into async block
            while let Ok(msg) = rx.recv().await {
                println!("Received: {}", msg);
            }
        });


        //wait for both tasks to finish
        join!(sender, receiver);
    });
}