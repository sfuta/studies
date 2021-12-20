use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}\nin file {}", config.query, config.filename);

    let mut contents = String::new();
    let mut f = File::open(config.filename).expect("file not found");
    f.read_to_string(&mut contents)
        .expect("something went wrong reaing the file");

    println!("With text: \n{}", contents);

}

struct Config { query: String, filename: String }

/**
 * args parse 
 * 
 * @param &[String] args
 * @return Config
 */
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
