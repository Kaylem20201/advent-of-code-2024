pub fn solve(input: String) -> String {

    let parsed = parse_input(input);

    println!("Parsed input.");
    // println!("Left vals: {:?}", parsed.0);
    // println!("Right vals: {:?}", parsed.1);
    
    return String::from("");
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
