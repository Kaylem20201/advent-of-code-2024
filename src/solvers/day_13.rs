const A_TOKENS: usize = 3;
const B_TOKENS: usize = 1;
const CONVERT: usize = 10000000000000;

struct ParsedInput {
    problems: Vec<Problem>,
}

#[derive(Debug)]
struct Problem {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
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
    let mut final_tokens = 0;

    for Problem { a, b, prize } in input.problems.iter() {
        let mut min_cost = (100 * A_TOKENS) + (100 * B_TOKENS);
        for b_presses in 0..=100 {
            for a_presses in 0..=100 {
                let cost = (a_presses * A_TOKENS) + (b_presses * B_TOKENS);
                let [x_pos, y_pos] = [
                    (a_presses * a.0) + (b_presses * b.0),
                    (a_presses * a.1) + (b_presses * b.1),
                ];
                if x_pos == prize.0 && y_pos == prize.1 {
                    min_cost = usize::min(min_cost, cost);
                    continue;
                }
            }
        }
        if min_cost == (100 * A_TOKENS) + (100 * B_TOKENS) {
            // println!("No solution for {:?}", (a, b, prize));
            continue;
        }
        // println!("Cost for {:?}: {:?}", (a, b, prize), min_cost);
        final_tokens += min_cost;
    }

    final_tokens.to_string()
}

fn part_2(input: &ParsedInput) -> String {
    fn solve_buttons(problem: &Problem) -> Option<(usize, usize)> {
        //System of linear equations means that there are 0 solutions, 1 solutions, or infininitely
        //  many solutions
        //Infinites in this case would need negative values
        //Problem is constrained so that infinite is not a concern
        // (a.x * x) + (b.x * b) = p.x
        // (a.y * y) + (b.y * y) = p.y
        // AX = C
        // | a.x b.x || a | = | p.x |
        // | a.y b.y || b | = | p.y |
        //Cramer's rule. x = Det(x)/Det(A), y = Det(y)/Det(A)
        //Det(A) = (a_x * b_y) - (b_x * a_y)
        //a = ((p.x * b.y) - (p.y * b.x)) / Det(A)
        //b = ((a.x * p.y) - (p.x * a.y)) / Det(A)

        let ((a_x, a_y), (b_x, b_y), (p_x, p_y)) = (
            (problem.a.0 as isize, problem.a.1 as isize),
            (problem.b.0 as isize, problem.b.1 as isize),
            (problem.prize.0 as isize, problem.prize.1 as isize),
        );
        let det = (a_x * b_y) - (b_x * a_y);
        let a_num = (p_x * b_y) - (p_y * b_x);
        let b_num = (a_x * p_y) - (p_x * a_y);

        if (a_num % det != 0) || (b_num % det != 0) {
            println!("No solution for {:?}", problem);
            dbg!(det, a_num, b_num);
            return None;
        }

        let a_raw = a_num / det;
        let b_raw = b_num / det;

        if a_raw < 0 || b_raw < 0 {
            println!("No solution for {:?}", problem);
            dbg!(det, a_raw, b_raw);
            return None;
        }

        let (a, b) = (a_raw as usize, b_raw as usize);
        Some((a, b))
    }

    let mut final_tokens = 0;

    for raw_problem in &input.problems {
        let problem: Problem = Problem {
            prize: (raw_problem.prize.0 + CONVERT, raw_problem.prize.1 + CONVERT),
            ..*raw_problem
        };
        if let Some((a, b)) = solve_buttons(&problem) {
            let cost = 3 * a + b;
            final_tokens += cost;
        }
    }

    final_tokens.to_string()
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
