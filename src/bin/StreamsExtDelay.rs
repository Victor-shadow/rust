use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main(){
    trpl::run(async {
        let mut messages = pin!(get_output().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await{
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("Problem: {reason:?}"),
            }
        }
    })


}

fn get_output() -> impl Stream<Item = String>{
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h","i", "j"];
        for(index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 {100} else {300};
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
//In get_messages , use the enumerate iterator method with the message array to get the index of each
//item that is sent along with the item itself. Then we apply a 100-millisecond delays to even indexed items and a 300-millisecond delay
//to odd-indexed items to simulate the delays we might see from a stream of messages
//To sleep between messages in the get messages fn without blocking we need to use async
//However we cannot make get_output itself into an async function, because then we would return Future<Output = Stream<Item = String>>
//instead of Stream<Item = String>>. The caller would have to await get_messages itself to get access to the stream
//NOTE: Everything in a given future happens linearly; concurrency happens between futures
//Awaiting get_messages would require it to send all its messages, including the sleep delay  between each message
//before returning the receiver stream, As a result timeout would be useless, There would be no delays in the stream itself; they would
//all happen even before the stream is available
//Instead we leave get_messages as a regular function that returns a stream, and  we spawn a task to handle the
//async sleep calls
//Calling spawn task in this case works because the runtime has already been set; had we not a panic is triggered
//The timeout does not prevent the messages from arriving in the end. We still get all the original messages; because the channel is unbounded
//it can hold as many messages as we can fit in memory. If the message does not arrive before the timeout, the stream handler accounts for that
//but when it polls the stream  again, the message may have now arrived