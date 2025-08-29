fn main(){
    let mut value = 1;

    loop{
        println!("{}", value * 2);
        value = value + 1;
        if value > 10 {
            break;
        }
    }
}