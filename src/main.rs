// main.rs

extern crate sdl2;
extern crate sdl2_image;

mod action;
mod board;
mod gfx;
mod gui;
mod tile;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use action::SudokuAction;
use board::Board;
use gui::Gui;

type History = Vec<Board>;

fn main() {
    let mut gui = Gui::new();
    let mut h: History = Vec::new();
    let mut curr_history: usize = 0;
    let mut quit = false;

    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        if let Some(b) = load_puzzle(&filename) {
            h.push(b);
        }
    }

    if h.is_empty() {
        h.push(Board::new());
    }

    while !quit {
        assert!(curr_history < h.len());

        match gui.read_input() {
            SudokuAction::NoOp => {},
            SudokuAction::Quit => quit = true,

            SudokuAction::New(Some(filename)) =>
                if let Some(b) = load_puzzle(&filename) {
                    h.clear();
                    h.push(b);
                    curr_history = h.len() - 1;
                },

            SudokuAction::New(None) => {
                    h.clear();
                    h.push(Board::new());
                    curr_history = h.len() - 1;
                },

            SudokuAction::Undo =>
                if curr_history > 0 {
                    curr_history = curr_history - 1
                },

            SudokuAction::Redo =>
                if curr_history + 1 < h.len() {
                    curr_history = curr_history + 1
                },

            SudokuAction::AssignValue(x,y,v) =>
                if let Some(new_b) = h[curr_history].assign_value(x, y, v, false) {
                    while h.len() > curr_history + 1 {
                        h.pop();
                    }
                    h.push(new_b);
                    curr_history = h.len() - 1;
                },

            SudokuAction::UnassignValue(x,y) =>
                if let Some(new_b) = h[curr_history].unassign_value(x, y) {
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

fn load_puzzle(filename: &String) -> Option<Board> {
    let path = Path::new(filename);
    match File::open(path) {
        Ok(mut f) => load_board(&mut f),

        Err(e) => {
            println!("{}: {}", filename, e);
            None
        }
    }
}

fn load_board(file: &mut File) -> Option<Board> {
    let mut board = Board::new();
    let mut x: u8 = 0;
    let mut y: u8 = 0;

    for b in file.bytes() {
        let c = b.unwrap() as char;
        let mut next_col = false;
        let mut next_row = false;

        match c {
            '0' | '.' => {
                next_col = true;
            },

            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let v = c.to_digit(10).unwrap() as u8;
                if let Some(new_b) = board.assign_value(x, y, v, true) {
                    board = new_b;
                    next_col = true;
                } else {
                    break;
                }
            },

            '|' | '-' | ' ' | '\n' | '\t' => {
                // ignored characters.
            },

            _ => {
                // not a valid puzzle.
                break;
            }
        }

        if next_col {
            x = x + 1;
            if x >= 9 {
                next_row = true;
            }
        }
        if next_row {
            x = 0;
            y = y + 1;
            if y >= 9 {
                return Some(board)
            }
        }
    }

    None
}
