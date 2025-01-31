const TICS: u8 = 25;

struct ParsedInput {
    stones: Vec<usize>
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {

    println!("Initial arrangement:");
    println!("{:?}", input.stones);

    let mut stones = input.stones.clone();

    for i in 1..TICS+1 {
        stones = stones.into_iter().flat_map(|stone| {
            step_stone(stone)
        }).collect();

        println!("After {i} blinks:");
        println!("{:?}", stones);
    } 

    stones.len().to_string()
}

fn step_stone(stone: usize) -> Vec<usize> {
    if stone == 0 { return vec![1]; }
    let str = stone.to_string();
    if str.len() % 2 == 0 {
        let left_stone = str[0..str.len()/2].parse().unwrap();
        let right_stone = str[str.len()/2..str.len()].parse().unwrap();
        return vec![left_stone, right_stone];
    }
    vec![stone * 2024]
}

fn part_2(input: &ParsedInput) -> String {
    String::from("Not yet implemented")
}

fn parse_input(input: &str) -> ParsedInput {
    let stones = input.split_whitespace().map(|slice| {
        slice.parse::<usize>().unwrap()
    }).collect();

    ParsedInput { stones }
}
