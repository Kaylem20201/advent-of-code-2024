use std::error::Error;
use std::fs;
use std::env;
use std::path::Path;

use reqwest::header;

pub struct Config {
    day: u8,
    client: reqwest::blocking::Client,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let day = match args[1].parse::<u8>() {
            Ok(day) => day,
            Err(_e) => return Err("provide day number as first argument"),
        };
        let client = Config::build_client()?;

        Ok(Config { day, client })
    }

    fn build_client() -> Result<reqwest::blocking::Client, &'static str> {
        let mut headers = header::HeaderMap::new();
        let token = env::var("AOC_SESSION").unwrap();
        let mut cookie = header::HeaderValue::from_str(&format!("session={}", token)).unwrap();
        cookie.set_sensitive(true);
        headers.insert(header::COOKIE, cookie);
        println!("Headers: {:?}", headers);
        let client = match reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build() {
            Ok(client) => client,
            Err(_e) => return Err("session cookie invalid."),
        };
        
        return Ok(client);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let day = config.day;
    println!("Solving for day {day}...");
    let raw_input: String = get_input(config)?;
    println!("Input ready.");

    Ok(())
}

fn get_input(config: Config) -> Result<String, Box<dyn Error>> {
    let mut filestring = env::var("AOC_DIR")?;
    filestring.push_str(&format!("day_{}/input.txt", config.day));
    let filepath = Path::new(&filestring);
    let input = fs::read_to_string(&filepath).unwrap_or_else(|_| {
        println!("Existing input not found.");
        return cache_input(config, &filepath).unwrap_or_else(|err| {
            panic!("Could not fetch input: {err:?}");
        });
    });
    return Ok(input);
}

fn cache_input(config: Config, filepath: &Path) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2024/day/{}/input", config.day);
    let request = config.client.get(url);
    println!("Headers: {:?}", request);
    let res = request.send()?.text()?;
    println!("Input fetched! Writing to {}", filepath.display());
    fs::create_dir_all(filepath.parent().unwrap())?;
    fs::write(filepath, &res)?;
    println!("Input written to file.");
    return Ok(res);
}