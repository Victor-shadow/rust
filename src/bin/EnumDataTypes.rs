#[derive(Debug)]
enum GenderType {
    Name(String),
    UsrId (i32)
}

fn main() {
    let p1 = GenderType::UsrId(100);
    let p2 = GenderType::Name(String::from("Haskel"));

    println!("{:?}", p1);
    println!("{:?}", p2);

    match p1 {
        GenderType::Name(val) => {
            println!("{}", val);
        }

        GenderType::UsrId(val ) => {
            println!("{}", val);
        }
    }

}