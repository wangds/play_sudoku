// board.rs

use tile::Tile;

pub struct Board {
    tiles: Vec<Tile>
}

impl Board {
    pub fn new() -> Board {
        let mut ts: Vec<Tile> = Vec::new();

        for y in 0..9 {
            for x in 0..9 {
                let t = Tile::new(x, y);
                ts.push(t);
            }
        }

        Board {
            tiles: ts
        }
    }

    fn is_valid_assign_value(&self, x: u8, y: u8, v: u8) -> bool {
        self.tiles.iter().all(|t| t.is_valid_assign_value(x, y, v))
    }

    pub fn assign_value(&self, x: u8, y: u8, v: u8, init: bool) -> Option<Board> {
        if self.is_valid_assign_value(x, y, v) {
            let mut ts: Vec<Tile> = Vec::new();

            for old_t in self.tiles.iter() {
                let new_t = old_t.assign_value(x, y, v, init);
                ts.push(new_t);
            }

            Some(Board {
                tiles: ts
            })
        } else {
            None
        }
    }

    fn is_valid_unassign_value(&self, x: u8, y: u8) -> bool {
        self.tiles.iter().all(|t| t.is_valid_unassign_value(x, y))
    }

    pub fn unassign_value(&self, x: u8, y: u8) -> Option<Board> {
        if self.is_valid_unassign_value(x, y) {
            let mut ts = Vec::new();

            for t in self.tiles.iter() {
                ts.push(Tile::new_with_eliminated(t))
            }

            let mut b = Board {
                tiles: ts
            };

            for t in self.tiles.iter() {
                if !(t.x == x && t.y == y) {
                    if let Some(v) = t.assignment {
                        b = b.assign_value(t.x, t.y, v, t.is_init()).unwrap();
                    }
                }
            }

            Some(b)
        } else {
            None
        }
    }

    fn is_valid_cross_out_value(&self, x: u8, y: u8, v: u8) -> bool {
        self.tiles.iter().all(|t| t.is_valid_cross_out_value(x, y, v))
    }

    pub fn cross_out_value(&self, x: u8, y: u8, v: u8) -> Option<Board> {
        if self.is_valid_cross_out_value(x, y, v) {
            let mut ts: Vec<Tile> = Vec::new();

            for old_t in self.tiles.iter() {
                let new_t = old_t.cross_out_value(x, y, v);
                ts.push(new_t);
            }

            Some(Board {
                tiles: ts
            })
        } else {
            None
        }
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.x == x && t.y == y)
    }

    pub fn is_unique(&self, tile: &Tile, value: u8) -> bool {
        tile.is_remaining_candidate(value)
            || self.is_unique_in_row(tile.x, tile.y, value)
            || self.is_unique_in_col(tile.x, tile.y, value)
            || self.is_unique_in_block(tile.x, tile.y, value)
    }

    fn is_unique_in_row(&self, x: u8, y: u8, value: u8) -> bool {
        for t in self.tiles.iter() {
            if t.y == y
                && t.x != x
                && t.candidates.iter().any(|&v| v == value)
                && !t.eliminated.iter().any(|&v| v == value) {
                return false
            }
        }

        true
    }

    fn is_unique_in_col(&self, x: u8, y: u8, value: u8) -> bool {
        for t in self.tiles.iter() {
            if t.x == x
                && t.y != y
                && t.candidates.iter().any(|&v| v == value)
                && !t.eliminated.iter().any(|&v| v == value) {
                return false
            }
        }

        true
    }

    fn is_unique_in_block(&self, x: u8, y: u8, value: u8) -> bool {
        for t in self.tiles.iter() {
            if !(t.x == x && t.y == y)
                && t.is_in_same_block(x, y)
                && t.candidates.iter().any(|&v| v == value)
                && !t.eliminated.iter().any(|&v| v == value) {
                return false
            }
        }

        true
    }

    pub fn autofill(&self) -> Option<Board> {
        type AutoFillXYV = (u8,u8,u8);
        let mut unique_list: Vec<AutoFillXYV> = Vec::new();

        for t in self.tiles.iter() {
            if t.assignment.is_some() {
                continue
            }

            for &v in t.candidates.iter().filter(
                    |&&v1| t.eliminated.iter().all(|&v2| v1 != v2)) {
                if self.is_unique(t, v) {
                    unique_list.push((t.x, t.y, v));
                    break;
                }
            }
        }

        unique_list.iter().fold(None, |board, &(x,y,v)|
                match board {
                    None => self.assign_value(x, y, v, false),
                    Some(b) => b.assign_value(x, y, v, false)
                })
    }

    /*
    pub fn print(&self) {
        for t in self.tiles.iter() {
            t.print();
        }
        println!("");
    }
    */
}
