use std::time::Duration;
use trpl::{run, channel, sleep, join3};

fn main() {
    run(async {
        let (tx, mut rx) = channel();
        let tx1 = tx.clone();

        let tx_fut = async move {
            let vals = vec![
                "Rust",
                "from",
                "the",
                "future",
            ];

            for val in vals {
                tx.send(val.to_string()).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        let tx1_fut = async move {
            let vals = vec![
                "more",
                "messages",
                "from",
                "the",
                "future",
            ];

            for val in vals {
                tx1.send(val.to_string()).unwrap();
                sleep(Duration::from_millis(1500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{}'", value);
            }
        };

        join3(tx_fut, tx1_fut, rx_fut).await;
    });
}
//The program still never exists, because of the way the while let loop interacts with trpl::join
//->The future returned from trpl::join completes only once both futures passed to it have
//completed
//-> the tx future completes once it finishes sleeping after sending the last message in vals
//-> The rx future won't complete until the while let loop completes
//-> The while let loop won't end until awaiting  rx.recv produces None
//->Awaiting  rx.recv will return None only Once the other end of the channel is closed
//->The channel will close only if we call rx.close or when sender side tx is dropped
//->We don't call rx.close anywhere, and tx won't be dropped until the outermost async block passed to trpl::run ends
//->The block can't end because it is blocked on trpl::join completing,
//The async block where we send messages only borrows tx because sending a message does not require ownership, but if one can move tx into that async block
//it would be dropped once the block ends
//we often need to move data into closures when working with threads
//this async channel is also a multiple producer channel so clone can be called on tx if messages are to be sent to multiple futures
//first we clone tx creating tx1 outside the first async block. We move tx1 into that block
//then later we move the original tx into a new async block, where we send more messages on a slight delay
//Both of the async blocks for sending messages need to be async move blocks so that both tx and tx1  get dropped when the blocks finish