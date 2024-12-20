use regex::Regex;

struct ParsedInput {
    mul_pairs: Vec<(u32, u32)>
}

pub fn solve(input: String) -> (String, String) {

    let valid_sections = validate_sections(&input);
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(valid_sections);

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

fn part_2(valid_sections: Vec<String>) -> String {
    let mul_pairs: Vec<(u32, u32)> = valid_sections.iter().flat_map(|sec| {
        parse_input(&sec).mul_pairs
    }).collect();
    let input = ParsedInput { mul_pairs };
    part_1(&input)
}

fn parse_input(input: &str) -> ParsedInput {

    let re = Regex::new(r"mul\((?<num_1>\d+),(?<num_2>\d+)\)").unwrap();
    let mul_pairs: Vec<(u32, u32)> = re.captures_iter(input).map(|capture| {
        let (_, [num_1, num_2]) = capture.extract();
        return (num_1.parse::<u32>().unwrap(), num_2.parse::<u32>().unwrap());
    }).collect();
    return ParsedInput { mul_pairs };

}

fn validate_sections(input: &str) -> Vec<String> {
    let do_pat = r"do()";
    let dont_pat = r"don't()";
    let mut valid_sections: Vec<String> = Vec::new();
    let mut state_valid = true;
    let mut current_buffer = String::new();
    let input_chars = input.chars();
    for char in input_chars {
        current_buffer.push(char);
        if state_valid {
            if current_buffer.ends_with(dont_pat) {
                state_valid = false;
                valid_sections.push(current_buffer);
                current_buffer = String::new();
            }
            continue;
        }
        if current_buffer.ends_with(do_pat) {
            state_valid = true;
            current_buffer = String::new();
        }
    }
    if state_valid {
        valid_sections.push(current_buffer);
    }
    // println!("Valid Sections: {:?}", valid_sections);
    return valid_sections;
}
