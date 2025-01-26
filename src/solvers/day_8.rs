use std::collections::HashMap;
use std::collections::HashSet;

use crate::solvers::tools::grid;
use grid::Grid;
use grid::Point;

struct ParsedInput {
    grid: Grid<char>,
    antennas: HashMap<char, HashSet<Point>>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");

    parsed_input.grid.print();
    println!("{:?}", parsed_input.antennas);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let mut antinode_grid : Grid<char> = Grid {
        elements: input.grid.elements.clone(),
        length: input.grid.length,
        height: input.grid.height
    };
    input.antennas.iter().for_each(|(_, antenna_set)| {
        let mut seen_antennas: HashSet<&Point> = HashSet::new();
        antenna_set.into_iter().for_each(|antenna| {
            seen_antennas.iter().for_each(|seen_antenna| {
                let diff = antenna.sub(seen_antenna);
                println!("Diff: {:?}", diff);
                let antinode_1 = seen_antenna.sub(&diff);
                let antinode_2 = antenna.add(&diff);
                /* println!(
                    "Antinodes for antennas {:?} and {:?}: {:?}",
                    seen_antenna,
                    antenna,
                    (&antinode_1, &antinode_2)
                ); */
                if input.grid.is_in_bounds(&antinode_1) { antinode_grid.replace_at(&antinode_1, '#'); }
                if input.grid.is_in_bounds(&antinode_2) { antinode_grid.replace_at(&antinode_2, '#'); }
            });
            seen_antennas.insert(antenna);
        });
    });

    antinode_grid.print();

    antinode_grid.elements.into_iter().filter(|element| {
        *element == '#'
    }).collect::<Vec<_>>().len().to_string()

}

fn part_2(input: &ParsedInput) -> String {
    let mut antinode_grid : Grid<char> = Grid {
        elements: input.grid.elements.clone(),
        length: input.grid.length,
        height: input.grid.height
    };
    input.antennas.iter().for_each(|(_, antenna_set)| {
        let mut seen_antennas: HashSet<&Point> = HashSet::new();
        antenna_set.into_iter().for_each(|antenna| {
            seen_antennas.iter().for_each(|seen_antenna| {
                antinode_grid.replace_at(&antenna, '#');
                antinode_grid.replace_at(&seen_antenna, '#');
                let diff = antenna.sub(seen_antenna);
                println!("Diff: {:?}", diff);
                let mut antinode = seen_antenna.sub(&diff);
                loop {
                    println!("Antinode: {:?}", antinode);
                    if !input.grid.is_in_bounds(&antinode) { break; }
                    antinode_grid.replace_at(&antinode, '#');
                    antinode = antinode.sub(&diff);
                }
                antinode = antenna.add(&diff);
                loop {
                    println!("Antinode: {:?}", antinode);
                    if !input.grid.is_in_bounds(&antinode) { break; }
                    antinode_grid.replace_at(&antinode, '#');
                    antinode = antinode.add(&diff);
                }
            });
            seen_antennas.insert(antenna);
        });
    });

    antinode_grid.print();

    antinode_grid.elements.into_iter().filter(|element| {
        *element == '#'
    }).collect::<Vec<_>>().len().to_string()

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
    let grid = Grid {
        elements,
        height,
        length,
    };
    let antennas = parse_antennas(&grid);
    return ParsedInput { grid, antennas };
}

fn parse_antennas(grid: &Grid<char>) -> HashMap<char, HashSet<Point>> {
    let mut antennas: HashMap<char, HashSet<Point>> = HashMap::new();
    for (i, char) in grid.elements.iter().enumerate() {
        if *char == '.' {continue}
        let antenna = grid.index_to_coord(i);
        if let Some(set) = antennas.get_mut(char) {
            set.insert(antenna);
            continue;
        }
        antennas.insert(*char, HashSet::new());
        antennas.get_mut(char).unwrap().insert(antenna);
    }

    antennas
}
