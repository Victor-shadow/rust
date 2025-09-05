use std::time::Duration;
use async_std::task;
use async_std::future::timeout;

async fn potentially_long_operation() -> Result<(), ()> {
    // Simulate long operation
    task::sleep(Duration::from_secs(10)).await;
    Ok(())
}



fn main() {
    task::block_on(async {
        let future = potentially_long_operation();
        match timeout(Duration::from_secs(5), future).await {
            Ok(result) => println!("Operation Succeeded: {:?}", result),
            Err(_) => println!("Operation timed out"),
        }
    });
}

