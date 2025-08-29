///The split() string method returns an iterator over the substrings of a string slice, separated by
/// characters matched by a pattern
/// The limitation of the split() method is that the result cannot be stored for later use
/// The collect method can be used to store the result returned by the split as a vector
fn main(){
  let full_name = "Dennis, Richie, C";
    for token in full_name.split(","){
      println!("token is {}", token);
    }

  //store in a vector
  println!("\n");
  let tokens:Vec<&str> = full_name.split(",").collect();
  println!("first name is {}", tokens[0]);
  println!("last name is {}", tokens[1]);
  println!("language is {}", tokens[2])

}
