// main.rs

extern crate sdl2;

mod action;
mod board;
mod gui;
mod tile;

use action::SudokuAction;
use board::Board;
use gui::Gui;

type History = Vec<Board>;

fn main() {
    let mut gui = Gui::new();
    let mut quit = false;
    let mut redraw = true;

    let mut h: History = Vec::new();
    let mut curr_history: usize = 0;
    h.push(Board::new());

    while !quit {
        assert!(curr_history < h.len());
        let old_history = curr_history;

        match gui.read_input() {
            SudokuAction::NoOp => {},
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
            gui.redraw(&h[curr_history]);
            redraw = false;
        }
    }
}
