fn main(){
  define_language();//invoke the function in the main rust program
}
fn define_language(){
    let language: &str = "Rust programming language";

    println!("{}, is statically typed", language);

}