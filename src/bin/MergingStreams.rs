use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // Timeout for messages stream
        let messages = get_display().timeout(Duration::from_millis(200));

        // Transform intervals into String, throttle, and apply timeout
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        // Merge both streams and limit to 20 items
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        // Iterate over the merged stream
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("Problem: {reason:?}"),
            }
        }
    });
}


fn get_display() -> impl Stream<Item = String>{
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h","i", "j"];
        for(index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 {100} else {300};
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")){
                eprintln!("Cannot send message '{message}' : send_error: '{send_error}'");
                break;
            } else {
                tx.send(format!("Message: '{message}'")).unwrap();
            }

        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32>{
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move{
        let mut count = 0;
        loop{
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count){
                eprintln!("Could not send interval {count}: {send_error}")
            }
        }
    });
    ReceiverStream::new(rx)
}
//we start by defining a count in the task, Then we create an infinite loop. Each iteration  of the loop
//asynchronously sleeps for one millisecond, increments the count, and then sends it over the channel
//Because, this is all wrapped in the task created by spawn_task, all of it including the infinite loop, will get cleaned up with the runtime
//This kind of infinite loop, which ends only when the runtime gets torn down, is fairly common in async rust; many programs keep needing to run indefinitely
//With async, this does not block anything else, as long as there is at least one await point in each iteration through the loop
//In the main fn, one can attempt to merge the messages and intervals streams
//We start by calling get_intervals, Then we merge the messages and intervals streams with the merge method which
//combines multiple streams into one stream that produces items from any of the source streams as soon as the items are available
//without imposing any particular ordering, Finally, we loop over that combined stream instead of over messages
//At this point, neither messages or intervals needs to be pinned or mutable, because both will be contained in the
//single merged stream. However, the call to merge doesn't compile(neither does the next call in the while let loop) this is because
//the two streams have different types
//The messages stream has the type Timeout<impl Stream<Item = String>>,  where Timeout is the the type that implements Stream for a timeout call
//The intervals stream has the type impl Stream<Item = u32> To merge these streams one needs to be transformed
// to match the other
//First we can use the map helper method to transform the intervals into a string. Second we need to match the Timeout from Messages
//Because actually  a timeout  for intervals is not what is wanted. We create a 10 second timeout with Duration::from_sec(10)
//Finally one needs to make stream mutable, so the while let loop's next calls  can iterate through the stream, and pin it so it is safe to do so
//First we use the throttle method, on the intervals stream, so that it does not overwhelm the messages stream. Throttling is a way of limiting
//the rate at which a function will be called or how often a stream will be polled
//To limit the number of messages/items we accept from a stream, we apply the take method to the merged stream
//because the final output is to be limited, not just one stream or the other
//The throttle call produces a new stream that wraps the original stream so the original stream
//gets polled only at the throttle rate, not its own native rate. We do not have a bunch of of unhandled interval messages we choose to ignore
//instead, those interval messages are never produced in the first place. This is an inherent laziness of Rust futures