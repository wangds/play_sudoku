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

    pub fn assign_value(&self, x: u8, y: u8, v: u8) -> Option<Board> {
        if self.is_valid_assign_value(x, y, v) {
            let mut ts: Vec<Tile> = Vec::new();

            for old_t in self.tiles.iter() {
                let new_t = old_t.assign_value(x, y, v);
                ts.push(new_t);
            }

            Some(Board {
                tiles: ts
            })
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

    /*
    pub fn print(&self) {
        for t in self.tiles.iter() {
            t.print();
        }
        println!("");
    }
    */
}
