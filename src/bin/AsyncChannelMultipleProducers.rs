
use std::time::Duration;
use async_std::{channel, task};
use trpl::join;

#[tokio::main]
async fn main(){
    let (tx, mut rx) = channel::unbounded();

    let tx1 = tx.clone();
    let producer1 = task::spawn(async move {
        for i in 0..3 {
            tx1.send(format!("Message {} from producer1", i)).await.unwrap();
            task::sleep(Duration::from_millis(200)).await;
        }
    });

    let producer2 = task::spawn(async move {
        for i in 0..3{
            tx.send(format!("Message {} from producer 2", i)).await.unwrap();
            task::sleep(Duration::from_millis(300)).await;
        }
    });

    let consumer = task::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("Consumed: {}", msg);

        }
    });

    join!(producer1, producer2, consumer);


;

}