// gui.rs

use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::Mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2_image;
use sdl2_image::INIT_PNG;

use action::SudokuAction;
use board::Board;
use gfx::*;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

#[derive(Clone,Copy,Eq,PartialEq)]
enum Brush {
    Pencil,
    CrossOut
}

enum WidgetType {
    Undo,
    Redo,

    // ToolbarBrush(value,active,inactive)
    ToolbarBrush(Brush,Res,Res),

    // ToolbarNumber(value)
    ToolbarNumber(u8)
}

pub struct Gui<'a> {
    gfx: GfxLib<'a>,
    event_pump: EventPump,
    state: GuiState,
    widgets: Vec<Widget>,
    redraw: bool
}

struct GuiState {
    selected_brush: Brush,
    selected_value: u8
}

struct Widget {
    mode: WidgetType,
    rect: Rect,
}

impl<'a> Gui<'a> {
    pub fn new() -> Gui<'a> {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        sdl2_image::init(INIT_PNG);

        let window
            = video.window("Sudoku", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build().unwrap();

        let renderer = window.renderer().build().unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Gui {
            gfx: GfxLib::new(renderer),
            event_pump: event_pump,
            state: GuiState::new(),
            widgets: Gui::make_widgets(),
            redraw: true
        }
    }

    fn make_widgets() -> Vec<Widget> {
        let mut ws = Vec::new();
        let y = (SCREEN_HEIGHT as i32) - (TOOLBAR_BUTTON_HEIGHT as i32) - 3;
        let x_undo = (3 + TOOLBAR_BUTTON_WIDTH + 3) as i32;
        let x_redo = x_undo + (TOOLBAR_UNDO_REDO_WIDTH + 2) as i32;
        let toolbar_spacing = (TOOLBAR_NUMBER_WIDTH - 1) as i32;
        let x_1 = (SCREEN_WIDTH as i32) - toolbar_spacing * 9 - 4;
        let x_crossout = (x_redo + (TOOLBAR_UNDO_REDO_WIDTH as i32) + x_1) / 2 + 2;
        let x_pencil = x_crossout - (TOOLBAR_BUTTON_WIDTH as i32) - 2;

        // undo
        ws.push(Widget {
                mode: WidgetType::Undo,
                rect: Rect::new_unwrap(x_undo, y, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT),
                });

        // redo
        ws.push(Widget {
                mode: WidgetType::Redo,
                rect: Rect::new_unwrap(x_redo, y, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT),
                });

        // pencil
        ws.push(Widget {
                mode: WidgetType::ToolbarBrush(
                        Brush::Pencil, Res::ToolbarActivePencil, Res::ToolbarInactivePencil),
                rect: Rect::new_unwrap(x_pencil, y, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT)
                });

        // cross out
        ws.push(Widget {
                mode: WidgetType::ToolbarBrush(
                        Brush::CrossOut, Res::ToolbarActiveCrossOut, Res::ToolbarInactiveCrossOut),
                rect: Rect::new_unwrap(x_crossout, y, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT)
                });

        // toolbar
        for v in 1..9+1 {
            let x = x_1 + toolbar_spacing * (v as i32 - 1);

            ws.push(Widget {
                    mode: WidgetType::ToolbarNumber(v),
                    rect: Rect::new_unwrap(
                            x, y, TOOLBAR_NUMBER_WIDTH, TOOLBAR_BUTTON_HEIGHT)
                    });
        }

        ws
    }

    pub fn read_input(&mut self) -> SudokuAction {
        let timeout: u32 = 1000 / 60;
        if let Some(e) = self.event_pump.wait_event_timeout(timeout) {
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
        } else {
            // redraw if no events.
            self.redraw = true;
        }

        SudokuAction::NoOp
    }

    pub fn draw_to_screen(&mut self, board: &Board) {
        if !self.redraw {
            return;
        }

        let colour_white = Color::RGB(0xD0, 0xD0, 0xD0);
        let colour_light_grey = Color::RGB(0x98, 0x98, 0x98);
        let colour_dark_grey = Color::RGB(0x58, 0x58, 0x58);

        let toolbar_rect = Rect::new_unwrap(
                0,
                (SCREEN_HEIGHT as i32) - (TOOLBAR_BUTTON_HEIGHT as i32) - 6,
                SCREEN_WIDTH,
                TOOLBAR_BUTTON_HEIGHT + 6);

        let home_rect = Rect::new_unwrap(
                3,
                (SCREEN_HEIGHT as i32) - (TOOLBAR_BUTTON_HEIGHT as i32) - 3,
                TOOLBAR_BUTTON_WIDTH,
                TOOLBAR_BUTTON_HEIGHT);

        self.gfx.renderer.set_draw_color(colour_white);
        self.gfx.renderer.clear();

        // toolbar
        self.gfx.renderer.set_draw_color(colour_light_grey);
        self.gfx.renderer.fill_rect(toolbar_rect);

        self.gfx.renderer.set_draw_color(colour_dark_grey);
        self.gfx.renderer.draw_rect(toolbar_rect);

        self.gfx.draw(Res::ToolbarSudoku, home_rect);

        for w in self.widgets.iter() {
            Gui::draw_widget(&mut self.gfx, w, board, &self.state);
        }

        self.gfx.renderer.present();
        self.redraw = false;
    }

    fn draw_widget(gfx: &mut GfxLib, widget: &Widget, _: &Board, state: &GuiState) {
        let res = match widget.mode {
            WidgetType::Undo => Res::ToolbarUndo,
            WidgetType::Redo => Res::ToolbarRedo,

            WidgetType::ToolbarBrush(b,active,inactive) =>
                if state.selected_brush == b {
                    active
                } else {
                    inactive
                },

            WidgetType::ToolbarNumber(v) =>
                if state.selected_value == v {
                    Res::ToolbarActiveNumber(v)
                } else {
                    Res::ToolbarInactiveNumber(v)
                }
        };

        gfx.draw(res, widget.rect);
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
