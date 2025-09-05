use async_std::task;

fn main(){
    //CPU-Intensive work in thread pool
    let thread_result = std::thread::spawn(|| {
        compute_intensive_work()
    }).join().unwrap();

    println!("Thread Result: {:?}", thread_result);

    //Async I-bound operations
    async_std::task::block_on(async {
        let network_data = fetch_data_async().await;
        process_async(network_data).await
    });
}

async fn process_async(data: String) {
    println!("Processing async data; {}", data);
}

async fn fetch_data_async() -> String {
    task::sleep(std::time::Duration::from_millis(500)).await;
    "fetched_data".to_string()
}

fn compute_intensive_work() -> i32 {
    //Simulate CPU Work
    let mut sum = 0;
    for i in 0..1_000_000{
        sum += i;
    }
    sum

}