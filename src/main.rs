// main.rs

extern crate sdl2;
extern crate sdl2_image;

mod action;
mod board;
mod gfx;
mod gui;
mod tile;

use action::SudokuAction;
use board::Board;
use gui::Gui;

type History = Vec<Board>;

fn main() {
    let mut gui = Gui::new();
    let mut quit = false;

    let mut h: History = Vec::new();
    let mut curr_history: usize = 0;
    h.push(Board::new());

    while !quit {
        assert!(curr_history < h.len());

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

        gui.draw_to_screen(&h[curr_history]);
    }
}
