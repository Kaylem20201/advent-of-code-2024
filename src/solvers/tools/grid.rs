use std::usize;

pub struct Grid<T> {
    pub elements: Vec<T>,
    pub height: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl<T> Grid<T> {
    pub fn get(&self, coord: &Vector) -> &T {
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

    pub fn index_to_coord(&self, index: usize) -> Vector {
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
