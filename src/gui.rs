// gui.rs

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::Mouse;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

use action::SudokuAction;
use board::Board;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

enum Brush {
    Pencil,
    CrossOut
}

pub struct Gui<'a> {
    renderer: Renderer<'a>,
    event_pump: EventPump,
    state: GuiState
}

struct GuiState {
    selected_brush: Brush,
    selected_value: u8
}

impl<'a> Gui<'a> {
    pub fn new() -> Gui<'a> {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window
            = video.window("Sudoku", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build().unwrap();

        let renderer = window.renderer().build().unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Gui {
            renderer: renderer,
            event_pump: event_pump,
            state: GuiState::new()
        }
    }

    pub fn read_input(&mut self) -> SudokuAction {
        for e in self.event_pump.poll_iter() {
            match e {
                Event::Quit {..} =>
                    return SudokuAction::Quit,

                Event::KeyDown { keycode: Some(k), .. } =>
                    match self.state.on_key_down(k) {
                        SudokuAction::NoOp => {},
                        a => return a
                    },

                Event::MouseButtonDown { mouse_btn: Mouse::Left, .. } =>
                    match self.state.on_lmb() {
                        SudokuAction::NoOp => {},
                        a => return a
                    },

                Event::MouseButtonDown { mouse_btn: Mouse::Right, .. } =>
                    match self.state.on_rmb() {
                        SudokuAction::NoOp => {},
                        a => return a
                    },

                _ => {}
            }
        }

        SudokuAction::NoOp
    }

    pub fn redraw(&mut self, board: &Board) {
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        self.renderer.clear();
        board.print();
        self.renderer.present();
    }
}

impl GuiState {
    fn new() -> GuiState {
        GuiState {
            selected_brush: Brush::Pencil,
            selected_value: 1
        }
    }

    fn on_key_down(&mut self, keycode: Keycode) -> SudokuAction {
        match keycode {
            Keycode::Z => return SudokuAction::Undo,
            Keycode::X => return SudokuAction::Redo,

            Keycode::C => self.selected_brush = Brush::Pencil,
            Keycode::V => self.selected_brush = Brush::CrossOut,

            Keycode::Num1 => self.selected_value = 1,
            Keycode::Num2 => self.selected_value = 2,
            Keycode::Num3 => self.selected_value = 3,
            Keycode::Num4 => self.selected_value = 4,
            Keycode::Num5 => self.selected_value = 5,
            Keycode::Num6 => self.selected_value = 6,
            Keycode::Num7 => self.selected_value = 7,
            Keycode::Num8 => self.selected_value = 8,
            Keycode::Num9 => self.selected_value = 9,

            _ => {}
        }

        SudokuAction::NoOp
    }

    fn on_lmb(&mut self) -> SudokuAction {
        let x = 0;
        let y = 0;
        return SudokuAction::AssignValue(x, y, self.selected_value);
    }

    fn on_rmb(&mut self) -> SudokuAction {
        let x = 0;
        let y = 0;
        return SudokuAction::CrossOutValue(x, y, self.selected_value);
    }
}
