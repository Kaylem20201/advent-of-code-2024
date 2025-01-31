use std::ops::{Add, Sub};
use std::usize;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub elements: Vec<T>,
    pub height: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl<T> Grid<T> {
    pub fn get(&self, coord: &Point) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            return Some(&self.elements[index]);
        }

        None
    }

    pub fn replace_at(&mut self, coord: &Point, value: T) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.elements[index] = value;
            return self.get(coord);
        }

        None
    }

    pub fn is_in_bounds(&self, coord: &Point) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && coord.x < self.length.try_into().unwrap()
            && coord.y < self.height.try_into().unwrap()
    }

    pub fn coord_to_index(&self, coord: &Point) -> Option<usize> {
        if !self.is_in_bounds(coord) {
            return None;
        }

        let y = coord.y as usize * self.length;
        let x = coord.x as usize;

        Some(x + y)
    }

    pub fn index_to_coord(&self, index: usize) -> Point {
        if index >= self.elements.len() {
            panic!("Out of grid bounds");
        }
        let y = index / self.length;
        let x = index - (y * self.length);
        return Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        };
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: &self,
            curr_index: 0,
        }
    }

    pub fn get_cardinals(&self, start: &Point) -> Vec<Option<(Point, &T)>> {
        let cardinals =
            [Point::N, Point::E, Point::S, Point::W].map(|direction| start.add(&direction));
        cardinals
            .into_iter()
            .map(|point| {
                if let Some(value) = self.get(&point) {
                    return Some((point, value));
                }
                None
            })
            .collect()
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    curr_index: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<(Point, &'a T)> {
        if self.curr_index >= self.grid.elements.len() {
            return None;
        }
        let ele = &self.grid.elements[self.curr_index];
        let point = self.grid.index_to_coord(self.curr_index);
        self.curr_index += 1;
        return Some((point, ele));
    }
}

impl<T: ToString> Grid<T> {
    pub fn print(&self) {
        for i in 0..self.height {
            let start_index = i * self.length;
            let end_index = start_index + self.length;
            let line = String::from_iter(self.elements[start_index..end_index].into_iter().map(
                |element| {
                    return element.to_string();
                },
            ));
            println!("{line}");
        }
    }
}

impl Point {
    pub const NW: Point = Point { x: -1, y: -1 };
    pub const N: Point = Point { x: 0, y: -1 };
    pub const NE: Point = Point { x: 1, y: -1 };
    pub const E: Point = Point { x: 1, y: 0 };
    pub const SE: Point = Point { x: 1, y: 1 };
    pub const S: Point = Point { x: 0, y: 1 };
    pub const SW: Point = Point { x: -1, y: 1 };
    pub const W: Point = Point { x: -1, y: 0 };

    pub fn add(&self, _rhs: &Point) -> Point {
        // println!("Adding.");
        // println!("lhs: {:?}", &self);
        // println!("rhs: {:?}", _rhs);
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }

    pub fn sub(&self, _rhs: &Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }

    pub fn get_directions() -> Vec<Point> {
        vec![
            Point::NW,
            Point::N,
            Point::NE,
            Point::E,
            Point::SE,
            Point::S,
            Point::SW,
            Point::W,
        ]
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
