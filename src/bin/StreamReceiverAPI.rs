use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main(){
    //Adding a timeout that applies to every item in the stream, and a delay on the items emitted
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })

}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

    for message in messages{
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}

//A timeout to the stream is added using the timeout method, which comes from the StreamExt trait
//The body of the while let loop is updated, because the stream now returns a result, The Ok variant indicates
//that the message reached on time; The Err variant indicates that the timeout elapsed before any message arrived
//We match on that result and either print the message when it is received successfully or print a notice about the timeout
//The messages are pinned after applying timeout to them, because the timeout helper produces a stream that
//needs to be pinned to be polled
//However because there are no delays between messages, the timeout does not change the behaviour of the program
