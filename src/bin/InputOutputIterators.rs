use std::env;
use std::process;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str>{
        args.next(); //go to next program

        let query = match args.next(){
            Some(arg ) => arg,
            None => return Err("Did not get a Query String"),
        };

        let file_path = match args.next(){
            Some(arg ) => arg,
            None => return Err("Did not get a File Path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{
            file_path,
            query,
            ignore_case,
        })

    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
fn main(){
     let config = Config::build(env::args()).unwrap_or_else(|err| {
         eprintln!("Problem parsing arguments: {err}");
         process::exit(1);
     });

    let contents = std::fs::read_to_string(&config.file_path).expect("Should have been able to read a file");

    let results = search(&config.query, &contents);

    for line in results {
        println!("{line}");
    }
}//mutating args by iterating over it, we can add the mut keyword into the specification of the args parameter to make it mutable
