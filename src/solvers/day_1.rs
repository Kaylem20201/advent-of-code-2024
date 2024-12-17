pub fn solve(input: String) -> String {

    let (mut left_vals, mut right_vals) = parse_input(input);

    println!("Parsed input.");
    // println!("Left vals: {:?}", parsed.0);
    // println!("Right vals: {:?}", parsed.1);

    left_vals.sort();
    right_vals.sort();

    let zipped = Iterator::zip(left_vals.iter(), right_vals.iter());

    let answer: u64 = zipped.fold(0, |sum, pair| {
        let diff: u64 = pair.1.abs_diff(*pair.0).into();
        sum + diff
    });

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
