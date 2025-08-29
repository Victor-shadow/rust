fn main(){
      let mut count = 0; //initialize a mutable counter variable
     'count_up: loop{ //outer loop label
         println!("count =  {count}");
         let mut remainder = 10;

         loop{ //inner loop
             println!("remaining = {remainder}");

             if remainder == 9{
                 break;
             }
             if count == 2{
                 break 'count_up;
             }
             remainder = remainder - 1;
         }
         count = count  + 1;
    }
    println!("End count = {count}");
}
//the outer loop has the label 'count_up' and it will count from '0 - 2'
//the inner loop without a label counts down from 10 to 9
//the first break that does not specify the label will exit the inner loop
//the break 'count_up' statement will exit the outer loop