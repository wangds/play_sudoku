// main.rs

mod action;
mod board;
mod tile;

use action::SudokuAction;
use board::Board;

type History = Vec<Board>;

fn read_input(iter: u32) -> SudokuAction {
    if iter == 0 {
        SudokuAction::AssignValue(0,0,1)
    } else if iter == 1 {
        SudokuAction::CrossOutValue(1,1,2)
    } else {
        SudokuAction::Quit
    }
}

fn redraw_world(b: &Board) {
    b.print();
}

fn main() {
    let mut quit = false;
    let mut redraw = true;
    let mut iter = 0;

    let mut h: History = Vec::new();
    h.push(Board::new());

    while !quit {
        assert!(h.len() > 0);
        let old_len = h.len();

        match read_input(iter) {
            SudokuAction::Quit => quit = true,

            SudokuAction::AssignValue(x,y,v) =>
                if let Some(new_b) = h.last().unwrap().assign_value(x, y, v) {
                    h.push(new_b);
                },

            SudokuAction::CrossOutValue(x,y,v) =>
                if let Some(new_b) = h.last().unwrap().cross_out_value(x, y, v) {
                    h.push(new_b);
                }
        }

        if h.len() != old_len {
            redraw = true;
        }

        if redraw {
            redraw_world(h.last().unwrap());
        }

        // dodgy.
        iter = iter + 1;
    }
}
