//a generic structure named Data
//the <T> type indicates data type
struct Data<T>{
    value: T,
}
fn main(){
    //two instances
    //generic type of i32
    let t:Data<i32> = Data{value: 100};
    println!("Value is :{}", t.value);
    //generic type of String
    let t2:Data<String> = Data{value: "Rust".to_string()};
    println!("Value is :{}",t2.value);

}