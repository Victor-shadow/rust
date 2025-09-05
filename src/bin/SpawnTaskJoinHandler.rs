use std::time::Duration;

fn main(){
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10{
                println!("the first iteration {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..10{
            println!("The second iteration {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });

}