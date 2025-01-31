use std::collections::{HashMap, HashSet};

use super::tools::grid::{Grid, Point};

struct ParsedInput {
    grid: Grid<u32>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");
    parsed_input.grid.print();

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let mut sum = 0;

    for (point, digit) in input.grid.iter() {
        if *digit != 0 {
            continue;
        }
        let mut nines: HashSet<Point> = HashSet::new();
        let mut points: Vec<Point> = vec![point];

        loop {
            // println!("{:?}", points);
            let mut new_points: Vec<Vec<Point>> = Vec::new();
            for point in points.clone().into_iter() {
                match traverse(&input.grid, &point) {
                    Some(new_paths) => {
                        new_points.push(new_paths);
                        continue;
                    }
                    None => {
                        nines.insert(point);
                    }
                }
            }
            points = new_points.concat();
            if points.is_empty() {
                break;
            }
        }
        sum += nines.len();
    }

    sum.to_string()
}

fn traverse(grid: &Grid<u32>, start: &Point) -> Option<Vec<Point>> {
    if !grid.is_in_bounds(start) {
        panic!("Point {:?} not in bounds", start);
    }
    let start_val = grid.get(&start).unwrap();
    if *start_val == 9 {
        return None;
    }
    let cardinals = grid.get_cardinals(start);
    Some(
        cardinals
            .into_iter()
            .fold(Vec::new(), |mut points, cardinal| {
                match cardinal {
                    Some((point, value)) => {
                        if *value != start_val+1 { return points; }
                        points.push(point);
                        points
                    },
                    None => points
                }
            })
    )
}

fn part_2(input: &ParsedInput) -> String {
    let mut sum = 0;

    for (point, digit) in input.grid.iter() {
        if *digit != 0 {
            continue;
        }
        let mut points: Vec<Point> = vec![point];

        loop {
            // println!("{:?}", points);
            let mut new_points: Vec<Vec<Point>> = Vec::new();
            for point in points.clone().into_iter() {
                match traverse(&input.grid, &point) {
                    Some(new_paths) => {
                        new_points.push(new_paths);
                        continue;
                    }
                    None => {
                        sum += 1;
                    }
                }
            }
            points = new_points.concat();
            if points.is_empty() {
                break;
            }
        }
    }

    sum.to_string()
}

fn parse_input(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let length = lines.get(0).unwrap().len();
    let elements: Vec<u32> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| u32::from(char::to_digit(char, 10).unwrap()))
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect::<Vec<u32>>();
    let grid = Grid {
        elements,
        height,
        length,
    };

    return ParsedInput { grid };
}
