use std::collections::{HashMap, HashSet};

use super::tools::grid::{Grid, Point};

struct ParsedInput {
    grid: Grid<char>,
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
    let mut regions: Vec<(char, HashSet<Point>)> = Vec::new();
    let mut traversed_points: HashSet<Point> = HashSet::new();

    for (point, c) in input.grid.iter() {
        if traversed_points.contains(&point) {
            continue;
        }
        let new_region = traverse(&input.grid, &point);
        for new_point in new_region.iter() {
            traversed_points.insert(new_point.clone());
        }
        regions.push((*c, new_region));
    }

    let mut sum = 0;
    for (_c, region) in regions.into_iter() {
        let price = calc_price_1(&input.grid, &region);

        /* println!(
            "Region: {:?}\nPrice:{:?}\nArea:{:?}",
            (_c, &region),
            price,
            region.len()
        ); */

        sum += price;
    }

    sum.to_string()
}

fn traverse(grid: &Grid<char>, start: &Point) -> HashSet<Point> {
    let c = grid.get(&start).unwrap();
    let mut wip = HashSet::from([start.clone()]);
    let mut traversed = HashSet::new();

    while !wip.is_empty() {
        let mut new_points: HashSet<Point> = HashSet::new();
        for point in wip {
            if traversed.contains(&point) {
                continue;
            }
            traversed.insert(point.clone());
            grid.get_cardinals(&point)
                .into_iter()
                .filter_map(|op| {
                    if let Some((point, val)) = op {
                        if val == c {
                            return Some(point);
                        }
                    }
                    None
                })
                .for_each(|point| {
                    new_points.insert(point);
                });
        }
        wip = HashSet::from(new_points);
    }

    return traversed;
}

fn calc_price_1(grid: &Grid<char>, region: &HashSet<Point>) -> usize {
    let area = region.len();
    let perimeter = region.iter().fold(0, |perimeter, point| {
        grid.get_cardinals(point)
            .into_iter()
            .fold(0, |acc, cardinal| {
                if let Some((point, _)) = cardinal {
                    if region.contains(&point) {
                        return acc;
                    }
                    return acc + 1;
                }
                acc + 1
            })
            + perimeter
    });

    area * perimeter
}

fn calc_price_2(grid: &Grid<char>, region: &HashSet<Point>) -> usize {
    let area = region.len();
    let corners = region
        .iter()
        .fold(0, |corners, point| corners + count_corners(grid, point));

    area * corners as usize
}

fn count_corners(grid: &Grid<char>, point: &Point) -> u8 {
    let c = grid.get(point).unwrap();
    let [north, east, south, west] = grid.get_cardinals(point).map(|op| {
        if let Some((point, card_c)) = op {
            if card_c == c {
                return Some(point);
            }
        }
        None
    });
    let mut corners = 0;

    let corner_directions = [
        (north, east, Point::NE),
        (east, south, Point::SE),
        (south, west, Point::SW),
        (west, north, Point::NW),
    ];

    for (a, b, direction) in corner_directions {
        if let (Some(_), Some(_)) = (a, b) {
            if let Some(opposite) = grid.get(&point.add(&direction)) {
                if opposite != c {
                    //inner corner
                    corners += 1;
                }
            }
        } else if (None, None) == (a, b) {
            //outer corner
            corners += 1;
        }
    }

    corners
}

fn part_2(input: &ParsedInput) -> String {
    let mut regions: Vec<(char, HashSet<Point>)> = Vec::new();
    let mut traversed_points: HashSet<Point> = HashSet::new();

    for (point, c) in input.grid.iter() {
        if traversed_points.contains(&point) {
            continue;
        }
        let new_region = traverse(&input.grid, &point);
        for new_point in new_region.iter() {
            traversed_points.insert(new_point.clone());
        }
        regions.push((*c, new_region));
    }

    let mut sum = 0;
    for (_c, region) in regions.into_iter() {
        let price = calc_price_2(&input.grid, &region);

        println!(
            "Region: {:?}\nPrice:{:?}\nArea:{:?}",
            (_c, &region),
            price,
            region.len()
        );

        sum += price;
    }

    sum.to_string()
}

fn parse_input(input: &str) -> ParsedInput {
    ParsedInput {
        grid: Grid::from(input),
    }
}
