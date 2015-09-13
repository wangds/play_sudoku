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
    } else if iter == 2 {
        SudokuAction::Undo
    } else if iter == 3 {
        SudokuAction::Undo
    } else if iter == 4 {
        SudokuAction::Redo
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
    let mut curr_history: usize = 0;
    h.push(Board::new());

    while !quit {
        assert!(curr_history < h.len());
        let old_history = curr_history;

        match read_input(iter) {
            SudokuAction::Quit => quit = true,

            SudokuAction::Undo =>
                if curr_history > 0 {
                    curr_history = curr_history - 1
                },

            SudokuAction::Redo =>
                if curr_history + 1 < h.len() {
                    curr_history = curr_history + 1
                },

            SudokuAction::AssignValue(x,y,v) =>
                if let Some(new_b) = h[curr_history].assign_value(x, y, v) {
                    while h.len() > curr_history + 1 {
                        h.pop();
                    }
                    h.push(new_b);
                    curr_history = h.len() - 1;
                },

            SudokuAction::CrossOutValue(x,y,v) =>
                if let Some(new_b) = h[curr_history].cross_out_value(x, y, v) {
                    while h.len() > curr_history + 1 {
                        h.pop();
                    }
                    h.push(new_b);
                    curr_history = h.len() - 1;
                }
        }

        if curr_history != old_history {
            redraw = true;
        }

        if redraw {
            redraw_world(&h[curr_history]);
            redraw = false;
        }

        // dodgy.
        iter = iter + 1;
    }
}
