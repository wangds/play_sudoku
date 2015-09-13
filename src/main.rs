// main.rs

mod board;
mod tile;

use board::Board;

fn main() {
    let b = Board::new();
    b.print();
}
