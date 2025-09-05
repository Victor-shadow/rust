//The trpl crate supplies a spawn_task function that is similar to the thread::spawn API and a
//sleep function that is async version of thread::sleep
use std::time::Duration;

fn main(){
    trpl::run(async {
         trpl::spawn_task(async {
            for i in 1..10{
                println!("the first iteration {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..10{
            println!("The second iteration {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

      
    });

}
//The top-level function is asynchronous since the main function has been set with trpl::run
//There are two loops within the block, each containing a trpl::sleep call which waits for half a second before sending the
//next message. We put one loop in the body of the trpl::spawn_task and the other in a top level for loop, we also add
//await after the sleep calls
//This version stops as soon as the for loop in the body of the main async finishes, because the task spawned by spawn_task is shut down
//when the main function ends
//If one wants to run all the way to the task completion, you need to use a join handle to wait for the first task to complete
//With threads, use the join method to block until the main thread was done running