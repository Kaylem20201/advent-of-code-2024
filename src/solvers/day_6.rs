use std::collections::HashSet;

use crate::solvers::tools::grid;
use grid::Grid;
use grid::Vector;

struct ParsedInput {
    grid: Grid<char>,
}

struct State {
    position: Vector,
    direction: Vector,
}

struct PathIterator<'a, T> {
    curr: State,
    grid: &'a Grid<T>,
}

fn rotate_right(direction: &Vector) -> Vector {
    match direction {
        &Vector::N => Vector::E,
        &Vector::E => Vector::S,
        &Vector::S => Vector::W,
        &Vector::W => Vector::N,
        _ => panic!("Needs cardinal directions"),
    }
}

impl Iterator for PathIterator<'_, char> {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        let mut direction = self.curr.direction.clone();
        loop {
            let possible_next = self.curr.position.add(&direction);
            let char;
            match self.grid.get(&possible_next) {
                None => {
                    println!("Reached edge: {:?}", possible_next);
                    return None;
                },
                Some(ch) => char = ch,
            }
            if *char != '#' {
                self.curr.position = possible_next.clone();
                self.curr.direction = direction;
                return Some(self.curr.position.clone());
            }
            println!("Turning at: {:?}", self.curr.position);
            println!("Previous direction: {:?}", direction);
            direction = rotate_right(&direction);
            println!("Turned. Position now: {:?}", self.curr.position);
            println!("Direction now: {:?}", direction);
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    // println!("Grid: ");
    // println!("Elements: {:?}", parsed_input.grid.elements);
    // println!("Height: {:?}", parsed_input.grid.height);
    // println!("Length: {:?}", parsed_input.grid.length);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let start = find_start(&input.grid);
    let mut visited = HashSet::new();
    visited.insert(start.position.clone());
    let path = PathIterator { curr: start, grid: &input.grid };
    for coord in path {
        visited.insert(coord);
    }
    return visited.len().to_string();
}

fn part_2(input: &ParsedInput) -> String {
    return String::from("Not implemented yet.");
}

fn parse_input(input: &str) -> ParsedInput {
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let length = lines.get(0).unwrap().len();
    let elements: Vec<char> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();
    return ParsedInput {
        grid: Grid {
            elements,
            height,
            length,
        },
    };
}

fn find_start(grid: &Grid<char>) -> State {
    for (index, char) in grid.elements.iter().enumerate() {
        match char {
            '^' => {
                return State {
                    position: grid.index_to_coord(index),
                    direction: Vector::N,
                };
            }
            '>' => {
                return State {
                    position: grid.index_to_coord(index),
                    direction: Vector::E,
                };
            }
            '<' => {
                return State {
                    position: grid.index_to_coord(index),
                    direction: Vector::W,
                };
            }
            'v' => {
                return State {
                    position: grid.index_to_coord(index),
                    direction: Vector::S,
                };
            }
            _ => continue,
        }
    }
    panic!("Starting position not found");
}
