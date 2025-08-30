use std::fs::File;

fn main() {
    let f = File::open("cargo.toml");
    match f {
        Ok(f) => {

        },
        Err(e) => {
            println!("Error opening file \n{:?}", e);
        }
    }
    //this file does not exist
    println!("End of main");
}