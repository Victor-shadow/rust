use std::time::Duration;
use trpl::run;
use trpl::channel;
fn main(){
    run (async {
        let (tx, mut rx) = channel();
        let vals = vec![
            String::from("rust"),
            String::from("from"),
            String::from("The"),
            String::from("Future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received: {value}")
        }
    });
}
//In addition to sending the messages we need to receive them. In this case, because we know how many messages
//are coming in that can be done manually by calling rx.recv().await four times
//We use a for loop to process all the items received from the synchronous channel. Rust does not yet have a way  to write a for loop
//over the asynchronous series of items
//We need to use the while let conditional loop(loop version of the if let construct)
//the loop will continue executing as long as the pattern it specifies continues to match the value
//The rx.recv() call produces a future which we await. The runtime will pause the future until it is ready
//Once a message arrives the future will resolve to Some(message) as many times a message arrives
//When the channel closes, regardless of whether any messages have arrived, the future will instead resolve to None
//to indicate that there are no more values and thus stop polling - awaiting
//The While let loop pulls all of this together. If the result of calling rx.recv().await is Some(message)
//we get access to the message and, we can use it in the loop body
//If the result is None the loop ends
//Every time the loop completes, it hits the await point again, so the runtime pauses it again until another message arrives
//Within a given async block, the order in which the await keyword appears in code, is also the order in which they are executed
//when the program runs
//There is only one async block so everything in it runs linearly . There is still no concurrency. All the tx.send calls happen, interspersed
//with all the trpl::sleep calls and their associated wait points. Only then, does the while let loop get to go through any of the await point on the recv calls
//To prevent this, the tx and rx operations should be put in their own async block, then the runtime can execute each of them separately using trpl::join

