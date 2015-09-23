// tile.rs

pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub init: bool,
    pub assignment: Option<u8>,

    // true list of potential values in this tile
    pub candidates: Vec<u8>,

    // list of values the player has crossed out
    pub eliminated: Vec<u8>
}

impl Tile {
    pub fn new(x: u8, y: u8) -> Tile {
        Tile {
            x: x,
            y: y,
            init: false,
            assignment: None,
            candidates: vec![1,2,3,4,5,6,7,8,9],
            eliminated: Vec::new()
        }
    }

    pub fn new_with_eliminated(tile: &Tile) -> Tile {
        Tile {
            x: tile.x,
            y: tile.y,
            init: false,
            assignment: None,
            candidates: vec![1,2,3,4,5,6,7,8,9],
            eliminated: tile.eliminated.clone()
        }
    }

    pub fn is_in_same_block(&self, x: u8, y: u8) -> bool {
        self.x/3 == x/3 && self.y/3 == y/3
    }

    pub fn is_valid_assign_value(&self, x: u8, y: u8, value: u8) -> bool {
        return x < 9 && y < 9 && (1 <= value && value <= 9)
            && (self.x != x
                    || self.y != y
                    || self.assignment.is_none());
    }

    pub fn assign_value(&self, x: u8, y: u8, value: u8, init: bool) -> Tile {
        assert!(x < 9 && y < 9 && (1 <= value && value <= 9));
        let mut vs = self.candidates.clone();

        if self.x == x && self.y == y {
            assert!(self.assignment.is_none());
            vs.retain(|&v| v == value);

            Tile {
                x: self.x,
                y: self.y,
                init: init,
                assignment: Some(value),
                candidates: vs,
                eliminated: self.eliminated.clone()
            }
        } else {
            if self.x == x
                || self.y == y
                || self.is_in_same_block(x, y) {
                vs.retain(|&v| v != value);
            }

            Tile {
                x: self.x,
                y: self.y,
                init: self.init,
                assignment: self.assignment,
                candidates: vs,
                eliminated: self.eliminated.clone()
            }
        }
    }

    pub fn is_valid_unassign_value(&self, x: u8, y: u8) -> bool {
        return x < 9 && y < 9
            && (self.x != x
                || self.y != y
                || (!self.init && self.assignment.is_some()));
    }

    pub fn is_valid_cross_out_value(&self, x: u8, y: u8, value: u8) -> bool {
        return x < 9 && y < 9 && (1 <= value && value <= 9)
            && (self.x != x
                || self.y != y
                || (!self.init
                    && self.assignment.is_none()
                    && self.candidates.iter().any(|&v| v == value)
                    && self.eliminated.iter().all(|&v| v != value)))
    }

    pub fn cross_out_value(&self, x: u8, y: u8, value: u8) -> Tile {
        assert!(x < 9 && y < 9 && (1 <= value && value <= 9));
        let mut vs = self.eliminated.clone();

        if self.x == x && self.y == y {
            assert!(self.assignment.is_none()
                    || self.assignment.unwrap() != value);
            vs.push(value);
        }

        Tile {
            x: self.x,
            y: self.y,
            init: self.init,
            assignment: self.assignment,
            candidates: self.candidates.clone(),
            eliminated: vs
        }
    }

    pub fn is_init(&self) -> bool {
        self.init
    }

    pub fn is_guess(&self) -> bool {
        if let Some(value) = self.assignment {
            self.candidates.iter().any(|&v| v == value)
        } else {
            false
        }
    }

    /*
    pub fn is_conflict(&self) -> bool {
        if let Some(value) = self.assignment {
            self.candidates.iter().all(|&v| v != value)
        } else {
            false
        }
    }
    */

    pub fn is_remaining_candidate(&self, value: u8) -> bool {
        let mut found = false;

        for &v in self.candidates.iter().filter(
                |&&v1| self.eliminated.iter().all(|&v2| v1 != v2)) {
            if v == value {
                found = true
            } else {
                return false
            }
        }

        found
    }

    /*
    pub fn print(&self) {
        println!("{} {}: {:?}", self.y, self.x, self.candidates);
    }
    */
}
