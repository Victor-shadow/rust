use std::thread;
use std::time::Duration;

fn main(){
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("Message: {message}");
        }
    })
}
//We create an async channel, then spawn a thread that takes ownership of the sender side  of the channel
//Within the thread, we send the numbers 1 through 10 sleeping for a second each
//Finally we run a future created by an  async block passed to trpl::run
//In that future we await messages