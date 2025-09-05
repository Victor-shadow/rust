
use std::time::Duration;
use trpl::Either;

fn main(){
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally Finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {}seconds", duration.as_secs());
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration>{
    match trpl::race(future_to_try, trpl::sleep(max_time)).await{
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
//If future_to_try succeeds and we get Left(output), we return Ok(output)if the sleep timer elapses
//instead we get a Right(()) we ignore the() and with _ and return Err(max_time) instead
//with that we have as a working timeout built out of two async helpers