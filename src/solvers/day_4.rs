use core::panic;
use std::usize;

struct ParsedInput {
    grid: Grid<char>,
}

struct Grid<T> {
    elements: Vec<T>,
    height: usize,
    length: usize,
}

#[derive(Debug)]
struct Vector {
    x: isize,
    y: isize,
}

impl<T> Grid<T> {
    fn get(&self, coord: &Vector) -> &T {
        if coord.x < 0 {
            panic!("X value must be 0 or positive! Given x: {:?}", coord.x);
        }
        if coord.y < 0 {
            panic!("Y value must be 0 or positive! Given y: {:?}", coord.y);
        }
        let y = coord.y as usize * self.length;
        let x = coord.x as usize;
        let index: usize = x + y;
        return &self.elements[index];
    }

    fn index_to_coord(&self, index: usize) -> Vector {
        let y = index / self.length;
        let x = index - (y * self.length);
        return Vector {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        };
    }
}

impl Vector {
    pub const NW: Vector = Vector { x: -1, y: -1 };
    pub const N: Vector = Vector { x: 0, y: -1 };
    pub const NE: Vector = Vector { x: 1, y: -1 };
    pub const E: Vector = Vector { x: 1, y: 0 };
    pub const SE: Vector = Vector { x: 1, y: 1 };
    pub const S: Vector = Vector { x: 0, y: 1 };
    pub const SW: Vector = Vector { x: -1, y: 1 };
    pub const W: Vector = Vector { x: -1, y: 0 };

    fn add(&self, _rhs: &Vector) -> Vector {
        // println!("Adding.");
        // println!("lhs: {:?}", &self);
        // println!("rhs: {:?}", _rhs);
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }

    pub fn get_directions() -> Vec<Vector> {
        vec![
            Vector::NW,
            Vector::N,
            Vector::NE,
            Vector::E,
            Vector::SE,
            Vector::S,
            Vector::SW,
            Vector::W,
        ]
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
    let pattern_chars = "XMAS".chars().collect::<Vec<char>>();
    let grid = &input.grid;
    let sum = grid
        .elements
        .iter()
        .enumerate()
        .fold(0, |sum, (index, grid_char)| {
            if grid_char != pattern_chars.get(0).unwrap() {
                return sum;
            }
            let starting_coord = grid.index_to_coord(index);
            let res = Vector::get_directions()
                .iter()
                .fold(0, |inner_sum, direction| {
                    match check_pattern(&grid, &pattern_chars, &starting_coord, direction) {
                        true => inner_sum + 1,
                        false => inner_sum,
                    }
                });
            println!("{res} matches found at {:?}", starting_coord);
            return res + sum;
        });
    return sum.to_string();
}

fn part_2(input: &ParsedInput) -> String {
    let pattern_chars = "MAS".chars().collect::<Vec<char>>();
    let grid = &input.grid;
    let sum = grid
        .elements
        .iter()
        .enumerate()
        .fold(0, |sum, (index, grid_char)| {
            if grid_char != pattern_chars.get(1).unwrap() {
                return sum;
            }
            let middle_coord = grid.index_to_coord(index);
            if middle_coord.x < 1
                || middle_coord.y < 1
                || middle_coord.x >= grid.length.try_into().unwrap()
                || middle_coord.y >= grid.height.try_into().unwrap()
            {
                return sum;
            }
            let coords_and_dirs: Vec<(Vector, Vector)> = vec![
                (middle_coord.add(&Vector::NW), Vector::SE),
                (middle_coord.add(&Vector::NE), Vector::SW),
                (middle_coord.add(&Vector::SE), Vector::NW),
                (middle_coord.add(&Vector::SW), Vector::NE),
            ];
            let res = coords_and_dirs
                .iter()
                .fold(0, |inner_sum, coord_and_dir| {
                    match check_pattern(&grid, &pattern_chars, &coord_and_dir.0, &coord_and_dir.1) {
                        true => inner_sum + 1,
                        false => inner_sum,
                    }
                });
            if res > 2 { panic!("This is impossible!"); }
            if res == 2 {
                println!("X-MAS found at {:?}", middle_coord);
                return sum + 1;
            }
            return sum;
        });
    return sum.to_string();
}

fn parse_input(input: &str) -> ParsedInput {
    let lines = input
        .lines()
        .map(|line_str| {
            let chars = line_str.chars();
            return chars.collect::<Vec<char>>();
        })
        .collect::<Vec<Vec<char>>>();
    let height = lines.len();
    // println!("Lines.length: {:?}", lines.len());
    // println!("Height: {:?}", height);
    // println!("lines.get(0): {:?}", lines.get(0).unwrap().len());
    let length = lines.get(0).unwrap().len();
    let chars = lines
        .iter()
        .flatten()
        .map(|char| char.clone())
        .collect::<Vec<char>>();
    ParsedInput {
        grid: Grid {
            elements: chars,
            height,
            length,
        },
    }
}

fn check_pattern(
    grid: &Grid<char>,
    pattern: &Vec<char>,
    starting_coord: &Vector,
    direction: &Vector,
) -> bool {
    let mut current_coord: Vector = Vector {
        x: starting_coord.x,
        y: starting_coord.y,
    };
    for check_char in pattern {
        if current_coord.x < 0
            || current_coord.y < 0
            || current_coord.x >= grid.length.try_into().unwrap()
            || current_coord.y >= grid.height.try_into().unwrap()
        {
            return false;
        }
        let grid_char = grid.get(&current_coord);
        if grid_char != check_char {
            // println!("{:?} did not match {:?}", grid_char, check_char);
            return false;
        }
        current_coord = current_coord.add(direction);
    }
    println!(
        "Pattern found at: {:?}, Direction: {:?}",
        starting_coord, direction
    );
    return true;
}
