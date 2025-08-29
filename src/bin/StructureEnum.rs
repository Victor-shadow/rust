#[derive(Debug)]
enum GenderCategory {
    Male,
    Female
}
#[derive(Debug)]
struct Person {
    name: String,
    gender: GenderCategory
}

fn main(){
    let p1 = Person {
        name:String::from("Del"),
        gender:GenderCategory::Male
    };

    let p2 = Person {
        name:String::from("James"),
        gender:GenderCategory::Female
    };

    println!("{:?}", p1.name);
    println!("{:?}", p2.name);
    println!("{:?}", p1.gender);
    println!("{:?}", p2.gender);

}