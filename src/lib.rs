use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use reqwest::header;

#[rustfmt::skip]
mod solvers {
    mod tools {
        pub mod grid;
    }

    mod day_1;
    mod day_2;
    mod day_3;
    mod day_4;
    mod day_5;
    mod day_6;
    mod day_7;
    mod day_8;
    mod day_9;
    mod day_10;
    mod day_11;
    mod day_12;
    mod day_13;
    mod day_14;
    // mod day_15;
    // mod day_16;
    // mod day_17;
    // mod day_18;
    // mod day_19;
    // mod day_20;
    // mod day_21;
    // mod day_22;
    // mod day_23;
    // mod day_24;
    // mod day_25;

    pub fn solve(day: u8, input: String) -> (String, String) {
        return match day {
            1 => day_1::solve(input),
            2 => day_2::solve(input),
            3 => day_3::solve(input),
            4 => day_4::solve(input),
            5 => day_5::solve(input),
            6 => day_6::solve(input),
            7 => day_7::solve(input),
            8 => day_8::solve(input),
            9 => day_9::solve(input),
            10 => day_10::solve(input),
            11 => day_11::solve(input),
            12 => day_12::solve(input),
            13 => day_13::solve(input),
            14 => day_14::solve(input),
            // 15 => day_15::solve(input),
            // 16 => day_16::solve(input),
            // 17 => day_17::solve(input),
            // 18 => day_18::solve(input),
            // 19 => day_19::solve(input),
            // 20 => day_20::solve(input),
            // 21 => day_21::solve(input),
            // 22 => day_22::solve(input),
            // 23 => day_23::solve(input),
            // 24 => day_24::solve(input),
            // 25 => day_25::solve(input),
            _ => panic!("Solution not implemented for day {}", day),
        };
    }
}

pub struct Config {
    day: u8,
    client: reqwest::blocking::Client,
    use_example: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let day = match args[1].parse::<u8>() {
            Ok(day) => day,
            Err(_e) => return Err("provide day number as first argument"),
        };
        let client = Config::build_client()?;
        let use_example =
            args.contains(&String::from("-e")) || args.contains(&String::from("--example"));

        Ok(Config {
            day,
            client,
            use_example,
        })
    }

    fn build_client() -> Result<reqwest::blocking::Client, &'static str> {
        let mut headers = header::HeaderMap::new();
        let token = env::var("AOC_SESSION").unwrap();
        let mut cookie = header::HeaderValue::from_str(&format!("session={}", token)).unwrap();
        cookie.set_sensitive(true);
        headers.insert(header::COOKIE, cookie);
        // println!("Headers: {:?}", headers);
        let client = match reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
        {
            Ok(client) => client,
            Err(_e) => return Err("session cookie invalid."),
        };

        return Ok(client);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let day = config.day;
    println!("Getting input for day {day}...");
    let raw_input: String = get_input(config)?;
    println!("Input ready.");

    println!("Solving...");
    let (part1_answer, part2_answer) = solvers::solve(day, raw_input);
    println!("Part 1 Answer: {:?}", part1_answer);
    println!("Part 2 Answer: {:?}", part2_answer);

    Ok(())
}

fn get_input(config: Config) -> Result<String, Box<dyn Error>> {
    let mut filestring = env::var("AOC_DIR")?;
    let base_path = match config.use_example {
        true => "examples",
        false => "inputs",
    };
    filestring.push_str(&format!("{base_path}/day_{}.txt", config.day));
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
