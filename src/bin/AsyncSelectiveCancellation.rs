use std::time::Duration;
use async_std::task;
use futures::future::FutureExt;

async fn potentially_long_operation() -> Result<(), ()>{
    //simulate long operation
    task::sleep(Duration::from_secs(5)).await;
    Ok(())
}
fn main() {
    task::block_on(async {
        let operation = potentially_long_operation().fuse();
        let timeout = task::sleep(Duration::from_secs(5)).fuse();

        futures::pin_mut!(operation, timeout);

        futures::select! {
        result = operation => println!("Operation completed: {:?}", result),
        _ = timeout => println!("Operation timed out"),
    }

    })


}