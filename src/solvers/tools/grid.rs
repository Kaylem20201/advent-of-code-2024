use std::io;
use std::usize;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub elements: Vec<T>,
    pub height: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl<T> Grid<T> {
    pub fn get(&self, coord: &Vector) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            return Some(&self.elements[index]);
        }

        None
    }

    pub fn replace_at(&mut self, coord: &Vector, value: T) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.elements[index] = value;
            return self.get(coord);
        }

        None
    }

    pub fn coord_to_index(&self, coord: &Vector) -> Option<usize> {
        if coord.x < 0 || coord.x as usize >= self.length {
            return None;
        }
        if coord.y < 0 || coord.y as usize >= self.height {
            return None;
        }
        let y = coord.y as usize * self.length;
        let x = coord.x as usize;
        Some(x + y)
    }

    pub fn index_to_coord(&self, index: usize) -> Vector {
        let y = index / self.length;
        let x = index - (y * self.length);
        return Vector {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        };
    }
}

impl Grid<char> {
    pub fn print(&self) {
        for i in 0..self.height {
            let start_index = i * self.length;
            let end_index = start_index + self.length;
            let line = String::from_iter(self.elements[start_index..end_index].into_iter());
            println!("{}", line);
        }
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

    pub fn add(&self, _rhs: &Vector) -> Vector {
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
