use std::fs::File;

fn main(){
    let f = File::open("Cargo.toml").expect("File not able to open!");
    //file does not exist
    println!("End of main");
}