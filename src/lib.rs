use std::error::Error;
use std::fs;
use std::env;
use std::path::Path;

pub struct Config {
    day: u8,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let day = match args[1].parse::<u8>() {
            Ok(day) => day,
            Err(_e) => return Err("provide day number as first argument"),
        };

        Ok(Config { day })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let day = config.day;
    println!("Solving for day {day}...");
    let raw_input: String = get_input(config.day)?;
    println!("{raw_input}");


    Ok(())
}

fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
    let mut filestring = env::var("AOC_DIR")?;
    filestring.push_str(&format!("day_{day}/input.txt"));
    let filepath = Path::new(&filestring);
    let input = fs::read_to_string(&filepath).unwrap_or_else(|_| {
        println!("Existing input not found.");
        return cache_input(day, &filepath).unwrap_or_else(|err| {
            panic!("Could not fetch input: {err:?}");
        });
    });
    return Ok(input);
}

fn cache_input(day: u8, filepath: &Path) -> Result<String, Box<dyn Error>> {
    let token = env::var("AOC_SESSION")?;
    let url = format!("https://adventofcode.com/2024/day/{day}/input?session={token}");
    let res = reqwest::blocking::get(url)?.text()?;
    println!("Input fetched! Writing to {}", filepath.display());
    fs::create_dir_all(filepath.parent().unwrap())?;
    fs::write(filepath, &res)?;
    println!("Input written to file.");
    return Ok(res);
}
