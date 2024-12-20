use regex::Regex;

struct ParsedInput {
    mul_pairs: Vec<(u32, u32)>
}

pub fn solve(input: String) -> (String, String) {

    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);

}

fn part_1(input: &ParsedInput) -> String {
    let iter = input.mul_pairs.iter();
    let res = iter.fold(0, |acc, pair| {
        let prod = pair.0 * pair.1;
        return acc + prod;
    });
    return res.to_string();
}

fn part_2(input: &ParsedInput) -> String {
    return String::from("Not yet implemented");
}

fn parse_input(input: &str) -> ParsedInput {

    let re = Regex::new(r"mul\((?<num_1>\d+),(?<num_2>\d+)\)").unwrap();
    let mul_pairs: Vec<(u32, u32)> = re.captures_iter(input).map(|capture| {
        let (_, [num_1, num_2]) = capture.extract();
        return (num_1.parse::<u32>().unwrap(), num_2.parse::<u32>().unwrap());
    }).collect();
    return ParsedInput { mul_pairs };

}
