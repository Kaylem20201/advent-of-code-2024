use std::collections::HashMap;

pub fn solve(input: String) -> (String, String) {

    let (left_vals, right_vals) = parse_input(input);

    println!("Parsed input.");

    let part1_answer = part_1(&left_vals, &right_vals);
    let part2_answer = part_2(left_vals, right_vals);

    return (part1_answer, part2_answer);

}

fn part_1(left_vals: &Vec<u32>, right_vals: &Vec<u32>) -> String {
    let mut owned_left_vals = left_vals.clone();
    let mut owned_right_vals = right_vals.clone();
    owned_left_vals.sort();
    owned_right_vals.sort();

    let zipped = Iterator::zip(owned_left_vals.iter(), owned_right_vals.iter());

    let answer: u64 = zipped.fold(0, |sum, pair| {
        let diff: u64 = pair.1.abs_diff(*pair.0).into();
        sum + diff
    });

    return answer.to_string();
}

fn part_2(left_vals: Vec<u32>, right_vals: Vec<u32>) -> String {
    let mut right_hash: HashMap<u32, u32> = HashMap::new();
    for key in right_vals {
        if let Some(val) = right_hash.get_mut(&key) {
            *val += 1;
        }
        else { right_hash.insert(key, 1); }
    }

    let mut answer = 0;
    for key in left_vals {
        if let Some(val) = right_hash.get(&key) {
            answer += key * val;
        }
    }

    return answer.to_string();
}

pub fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let raw_nums = input.split_whitespace();

    let nums: Vec<u32> = raw_nums.map(|raw_num| 
        raw_num.parse::<u32>().expect("Could not parse.")
    ).collect();

    let mut left_nums: Vec<u32> = Vec::new();
    let mut right_nums: Vec<u32> = Vec::new();

    for pair in nums.chunks(2) {
        left_nums.push(pair[0]);
        right_nums.push(pair[1]);
    }

    (left_nums, right_nums)

}
