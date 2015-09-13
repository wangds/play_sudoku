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

    pub fn print(&self) {
        for t in self.tiles.iter() {
            t.print();
        }
        println!("");
    }
}
