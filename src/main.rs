// main.rs

mod board;
mod tile;

use board::Board;

fn main() {
    let b = Board::new();
    let b2 = b.assign_value(0,0,1).unwrap();
    b2.print();
}
