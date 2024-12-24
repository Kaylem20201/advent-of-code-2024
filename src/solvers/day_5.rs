use std::collections::HashMap;

struct ParsedInput {
    rules: HashMap<u32, Vec<u32>>,
    patterns: Vec<Vec<u32>>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    println!("Rules: {:?}", parsed_input.rules);
    println!("Patterns: {:?}", parsed_input.patterns);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let res: usize = input.patterns.iter().fold(0, |res, pattern| {
        let middle_index: usize = pattern.len() / 2;
        let middle_number: usize =
            usize::try_from(pattern.get(middle_index).unwrap().to_owned()).unwrap();
        let mut iter = pattern.iter();
        let mut excludes = Vec::new();
        while let Some(i) = iter.next() {
            if let Some(rule_vec) = input.rules.get(&i) {
                for rule in rule_vec {
                    if excludes.contains(rule) {
                        println!("Pattern {:?} violates rule ({:?},{:?})", pattern, i, rule);
                        return res;
                    }
                }
            }
            excludes.push(*i);
        }
        return res + middle_number;
    });
    return res.to_string();
}

fn part_2(input: &ParsedInput) -> String {
    return String::from("Not yet implemented.");
}

fn parse_input(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<&str>>();
    let rules_lines = lines.iter().take_while(|line| line.contains("|"));
    let patterns_lines = lines
        .iter()
        .skip_while(|line| line.is_empty() || line.contains("|"));
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in rules_lines {
        let mut iter = line.split("|");
        let ele1 = iter.next().unwrap().parse::<u32>().unwrap();
        let ele2 = iter.next().unwrap().parse::<u32>().unwrap();
        if let Some(existing_vec) = rules.get_mut(&ele1) {
            existing_vec.push(ele2);
            continue;
        }
        rules.insert(ele1, vec![ele2]);
    }
    let patterns = patterns_lines
        .map(|line| {
            let iter = line.split(",");
            iter.map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    return ParsedInput { rules, patterns };
}
