// gui.rs

use std::cmp::{max,min};
use sdl2;
use sdl2::EventPump;
use sdl2::TimerSubsystem;
use sdl2::event::Event;
use sdl2::event::WindowEventId;
use sdl2::keyboard::Keycode;
use sdl2::mouse::Mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use sdl2_image;
use sdl2_image::INIT_PNG;

use action::SudokuAction;
use board::Board;
use gfx::*;
use tile::Tile;

// FIXME - not sure what to import.
const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = 0x1001;

const MIN_TOOLBAR_WIDTH: u32
    = 3
    + TOOLBAR_UNDO_REDO_WIDTH + 2 // undo
    + TOOLBAR_UNDO_REDO_WIDTH + 2 // redo
    + TOOLBAR_BUTTON_WIDTH + 2 // pencil
    + TOOLBAR_BUTTON_WIDTH + 2 // erase
    + (TOOLBAR_NUMBER_WIDTH - 1) * 9 // numbers
    + 3;

const DEFAULT_SCREEN_WIDTH: u32 = 640;
const DEFAULT_SCREEN_HEIGHT: u32 = 400;
const MIN_SCREEN_WIDTH: u32 = MIN_TOOLBAR_WIDTH;
const MIN_SCREEN_HEIGHT: u32 = 200;

// (w, h, board_scale, toolbar_scale)
type ScreenSize = (u32,u32,u32,u32);

#[derive(Clone,Copy,Eq,PartialEq)]
enum Brush {
    Pencil,
    CrossOut
}

enum WidgetType {
    Label,
    Undo,
    Redo,

    // Tile(x,y)
    Tile(u8,u8),

    // ToolbarBrush(value,active,inactive)
    ToolbarBrush(Brush,Res,Res),

    // ToolbarNumber(value)
    ToolbarNumber(u8)
}

pub struct Gui<'a> {
    gfx: GfxLib<'a>,
    timer: TimerSubsystem,
    event_pump: EventPump,
    state: GuiState,
    widgets: Vec<Widget>,

    screen_size: ScreenSize,
    redraw: bool,
    last_redraw: u32,

    // Some(new screen size) if need to relayout the widgets
    resize: Option<(u32,u32)>
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

        let screen_size = Gui::calc_screen_size_and_scale(
                DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT);

        let mut window
            = video.window("Sudoku", DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT)
            .resizable()
            .position_centered()
            .opengl()
            .build().unwrap();

        window.set_minimum_size(MIN_SCREEN_WIDTH, MIN_SCREEN_HEIGHT);

        let renderer = window.renderer().build().unwrap();

        let timer = sdl.timer().unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Gui {
            gfx: GfxLib::new(renderer),
            timer: timer,
            event_pump: event_pump,
            state: GuiState::new(),
            widgets: Gui::make_widgets(screen_size),
            screen_size: screen_size,
            redraw: true,
            last_redraw: 0,
            resize: None
        }
    }

    fn calc_screen_size_and_scale(screen_w: u32, screen_h: u32) -> ScreenSize {
        let board_x_spacing = TILE_NUMBER_WIDTH + 4;
        let board_y_spacing = TILE_NUMBER_HEIGHT + 4;
        let board_w = board_x_spacing * 9 + 2;
        let board_h = board_y_spacing * 9 + 2;

        let toolbar_w = MIN_TOOLBAR_WIDTH + TOOLBAR_BUTTON_WIDTH + 3; // sudoku
        let toolbar_h = TOOLBAR_BUTTON_HEIGHT + 6;

        let toolbar_x_scale = screen_w / toolbar_w;
        let toolbar_y_scale = (screen_h - board_h) / toolbar_h;
        let toolbar_scale = max(1, min(toolbar_x_scale, toolbar_y_scale));

        let board_x_scale = screen_w / board_w;
        let board_y_scale = (screen_h - toolbar_scale * toolbar_h) / board_h;
        let board_scale = max(1, min(board_x_scale, board_y_scale));

        (screen_w, screen_h, board_scale, toolbar_scale)
    }

    fn is_sudoku_label_visible(screen_size: ScreenSize) -> bool {
        let (screen_w, _, _, toolbar_scale) = screen_size;
        let toolbar_w = MIN_TOOLBAR_WIDTH + TOOLBAR_BUTTON_WIDTH + 3; // sudoku
        screen_w >= toolbar_scale * toolbar_w
    }

    fn make_widgets(screen_size: ScreenSize) -> Vec<Widget> {
        let mut ws = Vec::new();
        let (screen_w, screen_h, board_scale, toolbar_scale) = screen_size;
        let y = (screen_h - toolbar_scale * (TOOLBAR_BUTTON_HEIGHT + 3)) as i32;

        let toolbar_spacing = TOOLBAR_NUMBER_WIDTH - 1;
        let board_x_spacing = TILE_NUMBER_WIDTH + 4;
        let board_y_spacing = TILE_NUMBER_HEIGHT + 4;

        let label_visible = Gui::is_sudoku_label_visible(screen_size);
        let x_undo =
            if label_visible {
                (toolbar_scale * (3 + TOOLBAR_BUTTON_WIDTH + 3)) as i32
            } else {
                3
            };
        let x_redo = x_undo + (toolbar_scale * (TOOLBAR_UNDO_REDO_WIDTH + 2)) as i32;
        let x_1 = (screen_w - toolbar_scale * (toolbar_spacing * 9 + 4)) as i32;
        let x_crossout = (x_redo + (toolbar_scale * (TOOLBAR_UNDO_REDO_WIDTH + 4)) as i32 + x_1) / 2;
        let x_pencil = x_crossout - (toolbar_scale * (TOOLBAR_BUTTON_WIDTH + 2)) as i32;
        let (board_x, board_y) = Gui::calc_board_xy(screen_size);

        // label
        if label_visible {
            ws.push(Widget {
                    mode: WidgetType::Label,
                    rect: Rect::new_unwrap(
                            (toolbar_scale * 3) as i32,
                            y,
                            toolbar_scale * TOOLBAR_BUTTON_WIDTH,
                            toolbar_scale * TOOLBAR_BUTTON_HEIGHT)
                    });
        }

        // undo
        ws.push(Widget {
                mode: WidgetType::Undo,
                rect: Rect::new_unwrap(x_undo, y,
                        toolbar_scale * TOOLBAR_UNDO_REDO_WIDTH,
                        toolbar_scale * TOOLBAR_BUTTON_HEIGHT),
                });

        // redo
        ws.push(Widget {
                mode: WidgetType::Redo,
                rect: Rect::new_unwrap(x_redo, y,
                        toolbar_scale * TOOLBAR_UNDO_REDO_WIDTH,
                        toolbar_scale * TOOLBAR_BUTTON_HEIGHT),
                });

        // pencil
        ws.push(Widget {
                mode: WidgetType::ToolbarBrush(
                        Brush::Pencil, Res::ToolbarActivePencil, Res::ToolbarInactivePencil),
                rect: Rect::new_unwrap(x_pencil, y,
                        toolbar_scale * TOOLBAR_BUTTON_WIDTH,
                        toolbar_scale * TOOLBAR_BUTTON_HEIGHT)
                });

        // cross out
        ws.push(Widget {
                mode: WidgetType::ToolbarBrush(
                        Brush::CrossOut, Res::ToolbarActiveCrossOut, Res::ToolbarInactiveCrossOut),
                rect: Rect::new_unwrap(x_crossout, y,
                        toolbar_scale * TOOLBAR_BUTTON_WIDTH,
                        toolbar_scale * TOOLBAR_BUTTON_HEIGHT)
                });

        // tiles
        for row in 0..9 {
            for col in 0..9 {
                let x = board_x + (board_scale * (3 + board_x_spacing * col)) as i32;
                let y = board_y + (board_scale * (3 + board_y_spacing * row)) as i32;
                ws.push(Widget {
                        mode: WidgetType::Tile(col as u8, row as u8),
                        rect: Rect::new_unwrap(x, y,
                                board_scale * TILE_NUMBER_WIDTH,
                                board_scale * TILE_NUMBER_HEIGHT)
                        })
            }
        }

        // toolbar
        for v in 1..9+1 {
            let x = x_1 + (toolbar_scale * toolbar_spacing * (v - 1) as u32) as i32;

            ws.push(Widget {
                    mode: WidgetType::ToolbarNumber(v),
                    rect: Rect::new_unwrap(x, y,
                            toolbar_scale * TOOLBAR_NUMBER_WIDTH,
                            toolbar_scale * TOOLBAR_BUTTON_HEIGHT)
                    });
        }

        ws
    }

    fn calc_board_xy(screen_size: ScreenSize) -> (i32, i32) {
        let (screen_w, screen_h, board_scale, toolbar_scale) = screen_size;
        let board_x_spacing = TILE_NUMBER_WIDTH + 4;
        let board_y_spacing = TILE_NUMBER_HEIGHT + 4;
        let x0 = (screen_w - board_scale * (board_x_spacing * 9 + 2)) / 2;
        let y0 = (screen_h
                    - toolbar_scale * (TOOLBAR_BUTTON_HEIGHT + 6)
                    - board_scale * (board_y_spacing * 9 + 2)) / 2;
        (x0 as i32, y0 as i32)
    }

    pub fn read_input(&mut self) -> SudokuAction {
        let curr_ticks = self.timer.ticks();
        if curr_ticks >= self.last_redraw + 1000 / 60 {
            self.redraw = true;
            return SudokuAction::NoOp;
        }

        let timeout = self.last_redraw + 1000 / 60 - curr_ticks;
        if let Some(e) = self.event_pump.wait_event_timeout(timeout) {
            match e {
                Event::Quit {..} =>
                    return SudokuAction::Quit,

                Event::Window { win_event_id: WindowEventId::Resized, data1, data2, .. } =>
                    self.resize = Some((data1 as u32, data2 as u32)),

                Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                    let mut window = self.gfx.renderer.window_mut().unwrap();

                    if window.window_flags() & SDL_WINDOW_FULLSCREEN_DESKTOP != 0 {
                        window.set_fullscreen(FullscreenType::Off).unwrap();
                    } else {
                        window.set_fullscreen(FullscreenType::Desktop).unwrap();
                    }

                    return SudokuAction::NoOp
                },

                Event::KeyDown { keycode: Some(k), .. } =>
                    match self.state.on_key_down(k) {
                        SudokuAction::NoOp => {},
                        a => return a
                    },

                Event::MouseButtonDown { mouse_btn: Mouse::Left, x, y, .. } =>
                    if let Some(w) = Gui::find_widget(&self.widgets, x, y) {
                        match self.state.on_lmb(&w) {
                            SudokuAction::NoOp => return SudokuAction::NoOp,
                            a => return a
                        }
                    },

                Event::MouseButtonDown { mouse_btn: Mouse::Right, x, y, .. } =>
                    if let Some(w) = Gui::find_widget(&self.widgets, x, y) {
                        match self.state.on_rmb(&w) {
                            SudokuAction::NoOp => {},
                            a => return a
                        }
                    },

                Event::MouseButtonDown { mouse_btn: Mouse::Unknown(8), .. } =>
                    return SudokuAction::Undo,

                Event::MouseButtonDown { mouse_btn: Mouse::Unknown(9), .. } =>
                    return SudokuAction::Redo,

                Event::MouseWheel { y, .. } =>
                    self.state.on_wheel(y),

                Event::DropFile { filename, .. } =>
                    return SudokuAction::New(Some(filename)),

                _ => {}
            }
        }

        SudokuAction::NoOp
    }

    fn find_widget(widgets: &Vec<Widget>, x: i32, y: i32) -> Option<&Widget> {
        widgets.iter().find(|w| {
                let r = &w.rect;
                r.x() <= x && x <= r.x() + (r.width() as i32)
                && r.y() <= y && y <= r.y() + (r.height() as i32) })
    }

    pub fn draw_to_screen(&mut self, board: &Board) {
        if !self.redraw {
            return;
        }

        if let Some((new_w, new_h)) = self.resize {
            self.screen_size = Gui::calc_screen_size_and_scale(new_w, new_h);
            self.widgets = Gui::make_widgets(self.screen_size);
            self.resize = None;
        }

        let (screen_w, screen_h, board_scale, toolbar_scale) = self.screen_size;
        let colour_white = Color::RGB(0xD0, 0xD0, 0xD0);
        let colour_light_grey = Color::RGB(0x98, 0x98, 0x98);
        let colour_dark_grey = Color::RGB(0x58, 0x58, 0x58);

        let toolbar_rect = Rect::new_unwrap(
                0,
                (screen_h - toolbar_scale * (TOOLBAR_BUTTON_HEIGHT + 6)) as i32,
                screen_w,
                toolbar_scale * (TOOLBAR_BUTTON_HEIGHT + 6));

        self.gfx.renderer.set_draw_color(colour_white);
        self.gfx.renderer.clear();

        // board
        self.gfx.renderer.set_draw_color(colour_light_grey);
        for &y in [1,2,4,5,7,8].iter() {
            Gui::draw_board_hline(&mut self.gfx, self.screen_size, y);
        }
        for &x in [1,2,4,5,7,8].iter() {
            Gui::draw_board_vline(&mut self.gfx, self.screen_size, x);
        }

        self.gfx.renderer.set_draw_color(colour_dark_grey);
        for &y in [0,3,6,9].iter() {
            Gui::draw_board_hline(&mut self.gfx, self.screen_size, y);
        }
        for &x in [0,3,6,9].iter() {
            Gui::draw_board_vline(&mut self.gfx, self.screen_size, x);
        }

        // toolbar
        self.gfx.renderer.set_draw_color(colour_light_grey);
        self.gfx.renderer.fill_rect(toolbar_rect);

        self.gfx.renderer.set_draw_color(colour_dark_grey);
        self.gfx.renderer.draw_rect(toolbar_rect);

        // widgets
        for w in self.widgets.iter() {
            Gui::draw_widget(&mut self.gfx, board_scale, w, board, &self.state);
        }

        self.gfx.renderer.present();
        self.redraw = false;
        self.last_redraw = self.timer.ticks();
    }

    fn draw_board_hline(gfx: &mut GfxLib, screen_size: ScreenSize, y: u32) {
        let (_, _, scale, _) = screen_size;
        let board_x_spacing = TILE_NUMBER_WIDTH + 4;
        let board_y_spacing = TILE_NUMBER_HEIGHT + 4;
        let (board_x, board_y) = Gui::calc_board_xy(screen_size);

        let hline = Rect::new_unwrap(
                board_x,
                board_y + (scale * board_y_spacing * y) as i32,
                scale * (2 + board_x_spacing * 9),
                scale * 2);

        gfx.renderer.fill_rect(hline);
    }

    fn draw_board_vline(gfx: &mut GfxLib, screen_size: ScreenSize, x: u32) {
        let (_, _, scale, _) = screen_size;
        let board_x_spacing = TILE_NUMBER_WIDTH + 4;
        let board_y_spacing = TILE_NUMBER_HEIGHT + 4;
        let (board_x, board_y) = Gui::calc_board_xy(screen_size);

        let vline = Rect::new_unwrap(
                board_x + (scale * board_x_spacing * x) as i32,
                board_y,
                scale * 2,
                scale * (2 + board_y_spacing * 9));

        gfx.renderer.fill_rect(vline);
    }

    fn draw_widget(gfx: &mut GfxLib, scale: u32,
            widget: &Widget, board: &Board, state: &GuiState) {
        let res = match widget.mode {
            WidgetType::Label => Res::ToolbarSudoku,
            WidgetType::Undo => Res::ToolbarUndo,
            WidgetType::Redo => Res::ToolbarRedo,

            WidgetType::Tile(x,y) => {
                if let Some(t) = board.get(x,y) {
                    Gui::draw_tile(gfx, scale, t, widget.rect);
                }
                return;
            },

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

    fn draw_tile(gfx: &mut GfxLib, scale: u32, tile: &Tile, dst: Rect) {
        // chequer pattern
        if (tile.x + tile.y) % 2 != 0 {
            let colour_rose = Color::RGB(0xC2, 0xBC, 0xBC);
            gfx.renderer.set_draw_color(colour_rose);
            gfx.renderer.fill_rect(Rect::new_unwrap(
                    dst.x() - (scale * 1) as i32,
                    dst.y() - (scale * 1) as i32,
                    dst.width() + scale * 2,
                    dst.height() + scale * 2));
        }

        if let Some(v) = tile.assignment {
            let res =
                if tile.is_init() {
                    Res::TileInit(v)
                } else if tile.is_guess() {
                    Res::TileGuess(v)
                } else {
                    Res::TileConflict(v)
                };
            gfx.draw(res, dst);
        } else {
            let x_spacing: u32 = 3;
            let y_spacing: u32 = 3;
            let x0 = dst.x() + (dst.width() / 2 - scale * x_spacing) as i32;
            let y0 = dst.y() + (dst.height() / 2 - scale * y_spacing) as i32;
            let colour_dark_grey = Color::RGB(0x58, 0x58, 0x58);
            gfx.renderer.set_draw_color(colour_dark_grey);

            for &v in tile.candidates.iter().filter(
                    |&&v1| tile.eliminated.iter().all(|&v2| v1 != v2)) {
                if 1 <= v && v <= 9 {
                    let x = (v - 1) % 3;
                    let y = 2 - (v - 1) / 3;

                    gfx.renderer.fill_rect(Rect::new_unwrap(
                            x0 + (scale * x_spacing * x as u32) as i32,
                            y0 + (scale * y_spacing * y as u32) as i32,
                            scale * 1,
                            scale * 1));
                }
            }
        }
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

    fn on_lmb(&mut self, widget: &Widget) -> SudokuAction {
        match widget.mode {
            WidgetType::Label => {},
            WidgetType::Undo => return SudokuAction::Undo,
            WidgetType::Redo => return SudokuAction::Redo,

            WidgetType::Tile(x,y) =>
                match self.selected_brush {
                    Brush::Pencil => return SudokuAction::AssignValue(
                            x, y, self.selected_value),
                    Brush::CrossOut => return SudokuAction::CrossOutValue(
                            x, y, self.selected_value)
                },

            WidgetType::ToolbarBrush(brush,_,_) =>
                self.selected_brush = brush,

            WidgetType::ToolbarNumber(value) =>
                self.selected_value = value
        }

        SudokuAction::NoOp
    }

    fn on_rmb(&mut self, widget: &Widget) -> SudokuAction {
        match widget.mode {
            WidgetType::Tile(x,y) => SudokuAction::UnassignValue(x,y),
            _ => SudokuAction::NoOp
        }
    }

    fn on_wheel(&mut self, delta: i32) {
        self.selected_value = max(1, min(self.selected_value as i32 + delta, 9)) as u8;
    }
}
