use trpl::channel;
use trpl::run;

fn main() {
    run(async {
        let (tx,mut rx) = channel();

        let val = String::from("rust");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Received: {}", received);
    })
}
//We use trpl::channel, an async version of the multiple-producer, single-consumer channel API
//The async version uses a mutable rather than an immutable receiver rx, and its recv method produces a future
//that  we need to await rather than  producing the value directly. We need to await rx.recv call
//