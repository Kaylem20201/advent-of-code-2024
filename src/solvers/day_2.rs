struct ParsedInput<'a> {
    lines: Vec<&'a str>
}

pub fn solve(input: String) -> (String, String) {

    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);

}

fn part_1(input: &ParsedInput) -> String {
    input.lines.iter().fold(0, |sum, line| {
        if is_safe(line) { return sum + 1; }
        return sum;
    }).to_string()
}

fn part_2(input: &ParsedInput) -> String {
    return String::from("Not implemented");
}

fn parse_input(input: &str) -> ParsedInput {
    let lines: Vec<&str> = input.lines().collect();

    ParsedInput {
        lines
    }
}

fn is_safe(line: &str) -> bool {
    let nums: Vec<u32> = line.split_whitespace().map(|num_str| {
        return num_str.parse::<u32>().unwrap();
    }).collect();
    if !&nums.is_sorted_by(|a,b| a<b) && 
        !&nums.is_sorted_by(|a,b| b<a) {
        return false;
    }

    let pairs= nums.windows(2);
    for pair in pairs {
        let diff = pair[0].abs_diff(pair[1]);
        if diff < 1 || diff > 3 {return false;}
    }
    return true;
}
