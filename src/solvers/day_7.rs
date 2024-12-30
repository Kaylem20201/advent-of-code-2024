use std::collections::HashMap;

struct ParsedInput {
    equations: HashMap<usize, Vec<usize>>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let mut res = 0;
    for equation in input.equations.clone() {
        let (target, mut nums) = equation;
        let mut num_iter = nums.iter_mut();
        let mut set = Vec::new();
        set.push(*num_iter.next().unwrap());
        for num in num_iter {
            set = test_operators(set, *num);
        }
        println!("Tested equations on {:?}, ending set: {:?}", nums, set);
        if set.contains(&target) {
            res += target;
        }
    }

    res.to_string()
}

fn test_operators(set: Vec<usize>, next_val: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for prev_val in set.into_iter() {
        res.push(prev_val + next_val);
        res.push(prev_val * next_val);
    }
    return res;
}

fn part_2(input: &ParsedInput) -> String {
    let mut res = 0;
    for equation in input.equations.clone() {
        let (target, mut nums) = equation;
        let mut num_iter = nums.iter_mut();
        let mut set = Vec::new();
        set.push(*num_iter.next().unwrap());
        for num in num_iter {
            set = test_operators_2(set, *num);
        }
        println!("Tested equations on {:?}, ending set: {:?}", nums, set);
        if set.contains(&target) {
            res += target;
        }
    }

    res.to_string()
}

fn test_operators_2(set: Vec<usize>, next_val: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for prev_val in set.into_iter() {
        res.push(prev_val + next_val);
        res.push(prev_val * next_val);
        res.push(
            (prev_val.to_string() + &next_val.to_string())
                .parse::<usize>()
                .unwrap(),
        )
    }
    return res;
}

fn parse_input(input: &str) -> ParsedInput {
    let lines = input.lines();
    let mut equations = HashMap::new();
    for line in lines {
        let mut nums: Vec<&str> = line
            .split(|c: char| c == ':' || c.is_ascii_whitespace())
            .filter(|p| !p.is_empty())
            .collect();
        // println!("nums: {:?}", nums);
        let mut nums_iter = nums.iter_mut();
        let lhs = nums_iter.next().unwrap().parse::<usize>().unwrap();
        // println!("lhs: {:?}", lhs);
        let rhs = nums_iter
            .map(|str| {
                return str.parse::<usize>().unwrap();
            })
            .collect::<Vec<usize>>();
        equations.insert(lhs, rhs);
    }
    return ParsedInput { equations };
}
