use trpl::{ReceiverStream, Stream, StreamExt};
fn main(){
    trpl::run(async {
        let mut messages = get_message();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_message() -> impl Stream<Item = String> {
     let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

    for message in messages{
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
//First a function called get_messages is created that returns impl Stream<Item = String>
//For its implementation, an async channel is created, loops over the first 10 characters of the english alphabet
//and sends them across the channel
//We also use a new type: ReceiverStream with converts the rx receiver from the trpl::channel into  a Stream with next method
//Back in main, we use the while let loop to print all the messages from the stream