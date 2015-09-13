// tile.rs

pub struct Tile {
    x: u8,
    y: u8,
    candidates: Vec<u8>
}

impl Tile {
    pub fn new(x: u8, y: u8) -> Tile {
        Tile {
            x: x,
            y: y,
            candidates: vec![1,2,3,4,5,6,7,8,9]
        }
    }

    pub fn print(&self) {
        println!("{} {}: {:?}", self.y, self.x, self.candidates);
    }
}
