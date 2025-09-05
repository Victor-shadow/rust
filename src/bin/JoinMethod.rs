use std::time::Duration;

fn main(){
    trpl::run(async {
        let fut_1 = async {
            for i in 1..10 {
                print!("The first iteration {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut_2 = async {
            for i in 1..20{
                println!("The second iteration {i} after the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut_1, fut_2).await;

    })

}
//we use trpl::join to wait for both fut_1 and fut_2 to finish
//we do not await fut_1 and fut_2 but instead the new future produced by trpl::join