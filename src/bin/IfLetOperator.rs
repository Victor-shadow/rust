fn main() {
    // create string
    let string = "c++";

    // use if-else with string comparison
    let output = if string == "scala" {
        format!("programming language topic: {}", "scala")
    } else if string == "java" {
        format!("programming language topic: {}", "java")
    } else if string == "c++" {
        format!("programming language topic: {}", "c++")
    } else {
        format!("programming language topic: {}", "not a programming language")
    };

    println!("{:?}", output);
}