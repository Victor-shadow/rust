enum MyOption<T>{
    Some(T),
    None,
}

impl<T> MyOption<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            MyOption::Some(x ) => x,
            MyOption::None => f(),
        }
    }
}
fn main() {
  let opt : MyOption<String> = MyOption::None;
    let val = opt.unwrap_or_else(|| "fallback".to_string());
    println!("{val:#?}");

}
//Recall that T is a generic type representing the type of values in Some variant of an Option
//That type T is also the return type of unwrap_or_else function: code that calls unwrap_or_else on an Option<String>, will get a String
//unwrap_or_else function has the additional generic Type parameter F. The F type is the Type of the parameter named f, which is the closure we provide when calling unwrap_or_else
//The trait bound specified on th generic type F is FnOnce() -> T, which means that F must be able to called once, take no arguments and return a T
//Using FnOnce, in the trait bound expresses the constraint that unwrap_or_else is only going to call f at most one time
//In the body of unwrap_or_else, we can see that if the Option is Some, f won't be called If the Option is None, f will be called once
//Because all closures implement FnOnce , unwrap_or_else accepts all three kinds of closures and is as flexible as it can be