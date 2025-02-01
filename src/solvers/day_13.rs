const A_COST: u32 = 3;
const B_COST: u32 = 1;

struct ParsedInput {
    problems: Vec<Problem>,
}

#[derive(Debug)]
struct Problem {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");
    println!("{:?}", parsed_input.problems);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    String::from("Not yet implemented")
}

fn part_2(input: &ParsedInput) -> String {
    String::from("Not yet implemented")
}

fn parse_input(input: &str) -> ParsedInput {
    let mut problems: Vec<Problem> = Vec::new();

    let mut lines = input.lines().filter(|line| !line.is_empty());

    loop {
        let raw_problem = lines.by_ref().take(3);
        let raw_lines = raw_problem.collect::<Vec<&str>>();
        if let [a_line, b_line, prize_line] = &raw_lines[..] {
            let (a_x, a_y) = a_line
                .trim_start_matches("Button A: X+")
                .split_once(", Y+")
                .expect("Couldn't split");
            let (b_x, b_y) = b_line
                .trim_start_matches("Button B: X+")
                .split_once(", Y+")
                .expect("Couldn't split");
            let (p_x, p_y) = prize_line
                .trim_start_matches("Prize: X=")
                .split_once(", Y=")
                .expect("Couldn't split");
            let problem = Problem {
                a: (a_x.parse().unwrap(), a_y.parse().unwrap()),
                b: (b_x.parse().unwrap(), b_y.parse().unwrap()),
                prize: (p_x.parse().unwrap(), p_y.parse().unwrap()),
            };
            problems.push(problem);
        } else {
            break;
        }
    }

    ParsedInput { problems }
}
