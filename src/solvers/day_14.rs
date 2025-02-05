use super::tools::grid::{Grid, Point};

#[derive(Debug)]
struct ParsedInput {
    robots: Vec<Robot>,
    height: u16,
    width: u16,
}

#[derive(Debug)]
struct Robot {
    start: Point,
    vel: Point,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");
    dbg!(&parsed_input);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let starting_robots = &input.robots;
    const TICS: u32 = 100;
    // const TICS: u32 = 2;

    let mut grid = Grid {
        elements: vec![0u8; (input.height * input.width) as usize],
        height: input.height as usize,
        width: input.width as usize,
    };

    for robot in starting_robots {
        let [final_x, final_y] = [
            robot.start.x + (robot.vel.x * TICS as isize),
            robot.start.y + (robot.vel.y * TICS as isize),
        ];
        let [wrapped_x, wrapped_y]: [isize; 2] = [
            (final_x.rem_euclid(grid.width as isize)),
            (final_y.rem_euclid(grid.height as isize)),
        ];
        let point = Point {
            x: wrapped_x,
            y: wrapped_y,
        };
        if !grid.is_in_bounds(&point) {
            dbg!(final_x, final_y, wrapped_x, wrapped_y);
            panic!("Out of bounds.");
        }
        grid.replace_at(&point, grid.get(&point).unwrap() + 1);
    }
    grid.print();

    count_quadrants::<u8>(&grid).to_string()

}

fn count_quadrants<U8>(grid: &Grid<u8>) -> usize {
    let [left_x, right_x] = [0..grid.width / 2, grid.width / 2 + 1..grid.width];
    let [upper_y, lower_y] = [0..grid.height / 2, grid.height / 2 + 1..grid.height];

    let quad_1: usize = left_x
        .clone()
        .into_iter()
        .map(|x| {
            upper_y
                .clone()
                .into_iter()
                .map(|y| {
                    return grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize,
                        })
                        .unwrap().to_owned() as usize;
                }).sum::<usize>()
        })
        .sum();
    let quad_2: usize = right_x
        .clone()
        .into_iter()
        .map(|x| {
            upper_y
                .clone()
                .into_iter()
                .map(|y| {
                    return grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize,
                        })
                        .unwrap().to_owned() as usize;
                }).sum::<usize>()
        })
        .sum();
    let quad_3: usize = right_x
        .clone()
        .into_iter()
        .map(|x| {
            lower_y
                .clone()
                .into_iter()
                .map(|y| {
                    return grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize,
                        })
                        .unwrap().to_owned() as usize;
                }).sum::<usize>()
        })
        .sum();
    let quad_4: usize = left_x
        .clone()
        .into_iter()
        .map(|x| {
            lower_y
                .clone()
                .into_iter()
                .map(|y| {
                    return grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize,
                        })
                        .unwrap().to_owned() as usize;
                }).sum::<usize>()
        })
        .sum();

    quad_1 * quad_2 * quad_3 * quad_4
}

fn part_2(input: &ParsedInput) -> String {
    String::from("Not implemented yet")
}

fn parse_input(input: &str) -> ParsedInput {
    let lines: Vec<_> = input.lines().collect();
    let mut robots: Vec<Robot> = Vec::new();
    for line in lines.iter() {
        let (p_str, v_str) = line.split_once(' ').unwrap();
        let (p_x_str, p_y_str) = p_str.trim_start_matches("p=").split_once(',').unwrap();
        let [p_x, p_y] = [p_x_str, p_y_str]
            .into_iter()
            .map(|str| str.parse::<isize>().unwrap())
            .collect::<Vec<_>>()[..]
        else {
            panic!();
        };
        let (v_x_str, v_y_str) = v_str.trim_start_matches("v=").split_once(',').unwrap();
        let [v_x, v_y] = [v_x_str, v_y_str]
            .into_iter()
            .map(|str| str.parse::<isize>().unwrap())
            .collect::<Vec<_>>()[..]
        else {
            panic!();
        };
        robots.push(Robot {
            start: Point { x: p_x, y: p_y },
            vel: Point { x: v_x, y: v_y },
        });
    }

    if lines.len() <= 15 {
        //Short example input
        return ParsedInput {
            robots,
            width: 11,
            height: 7,
        };
    }

    ParsedInput {
        robots,
        width: 101,
        height: 103,
    }
}
