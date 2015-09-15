// tile.rs

pub struct Tile {
    x: u8,
    y: u8,
    assignment: Option<u8>,
    candidates: Vec<u8>
}

impl Tile {
    pub fn new(x: u8, y: u8) -> Tile {
        Tile {
            x: x,
            y: y,
            assignment: None,
            candidates: vec![1,2,3,4,5,6,7,8,9]
        }
    }

    pub fn is_valid_assign_value(&self, x: u8, y: u8, value: u8) -> bool {
        return if x < 9 && y < 9 && (1 <= value && value <= 9) {
            if self.x == x && self.y == y {
                self.assignment.is_none()
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn assign_value(&self, x: u8, y: u8, value: u8) -> Tile {
        assert!(x < 9 && y < 9 && (1 <= value && value <= 9));
        let mut vs = self.candidates.clone();

        if self.x == x && self.y == y {
            assert!(self.assignment.is_none());
            vs.retain(|&v| v == value);

            Tile {
                x: self.x,
                y: self.y,
                assignment: Some(value),
                candidates: vs
            }
        } else {
            if self.x == x
                || self.y == y
                || (self.x/3 == x/3 && self.y/3 == y/3) {
                vs.retain(|&v| v != value);
            }

            Tile {
                x: self.x,
                y: self.y,
                assignment: self.assignment,
                candidates: vs
            }
        }
    }

    pub fn is_valid_cross_out_value(&self, x: u8, y: u8, value: u8) -> bool {
        return if x < 9 && y < 9 && (1 <= value && value <= 9) {
            if self.x == x && self.y == y {
                self.assignment.is_none()
                && self.candidates.iter().any(|&v| v == value)
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn cross_out_value(&self, x: u8, y: u8, value: u8) -> Tile {
        assert!(x < 9 && y < 9 && (1 <= value && value <= 9));
        let mut vs = self.candidates.clone();

        if self.x == x && self.y == y {
            assert!(self.assignment.is_none()
                    || self.assignment.unwrap() != value);
            vs.retain(|&v| v != value);
        }

        Tile {
            x: self.x,
            y: self.y,
            assignment: self.assignment,
            candidates: vs
        }
    }

    /*
    pub fn print(&self) {
        println!("{} {}: {:?}", self.y, self.x, self.candidates);
    }
    */
}
