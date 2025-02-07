use std::collections::HashMap;

use super::tools::grid::{Grid, Point};

struct ParsedInput {
    grid: Grid<char>,
    moves: Box<str>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");
    parsed_input.grid.print();
    dbg!(&parsed_input.moves);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let mut grid = input.grid.clone();
    let mut pos = grid.find_first(&'@').unwrap();

    for m in input.moves.chars() {
        println!("Move: {:?}", m);
        pos = nav(&mut grid, &pos, m);
        grid.print();
    }

    grid.find_all(&'O').unwrap().iter().fold(0, |sum, point| {
        sum + (100 * point.y + point.x)
    }).to_string()
}

//Mutates in place
//Returns updated start position '@'
fn nav(grid: &mut Grid<char>, pos: &Point, c: char) -> Point {
    let direction: Point = match c {
        '^' => Point::N,
        '>' => Point::E,
        'v' => Point::S,
        '<' => Point::W,
        _ => panic!(),
    };

    let mut moving: Vec<(Point, char)> = vec![(pos.clone(), '@')];

    loop {
        let next_pos = moving.last().unwrap().0 + direction;
        let Some(next_char) = grid.get(&next_pos) else {
            panic!();
        };
        match *next_char {
            '#' => return moving.first().unwrap().0,
            'O' => {
                moving.push((next_pos, *next_char));
            }
            '.' => break,
            _ => panic!(),
        }
    }
    
    grid.replace_at(&moving.first().unwrap().0, '.');
    let new_positions: Vec<(Point, char)> = moving
        .iter()
        .map(|(old_pos, c)| (*old_pos + direction, *c))
        .collect();
    new_positions.iter().for_each(|(pos, c)| {
        grid.replace_at(pos, *c);
    });

    new_positions[0].0
}

fn part_2(input: &ParsedInput) -> String {
    String::from("Not yet implemented")
}

fn parse_input(input: &str) -> ParsedInput {
    let (grid_str, moves_str): (&str, &str) = input.split_once("\n\n").unwrap();

    ParsedInput {
        grid: Grid::from(grid_str.trim()),
        moves: Box::from(moves_str.replace("\n", "")),
    }
}
