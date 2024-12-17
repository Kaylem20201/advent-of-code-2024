use dotenv;
use std::env;
use std::process;

use advent_of_code_2024::Config;

fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = advent_of_code_2024::run(config) {
        println!("Error while solving: {e}");
        process::exit(1);
    }

}
