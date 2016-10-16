// resource.rs

use std::collections::HashMap;
use std::env;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2_image::LoadTexture;

#[derive(Clone,Copy,Eq,Hash,PartialEq)]
pub enum Res {
    ToolbarSudoku,
    ToolbarUndo,
    ToolbarRedo,
    ToolbarActivePencil,
    ToolbarInactivePencil,
    ToolbarActiveCrossOut,
    ToolbarInactiveCrossOut,
    ToolbarActiveNumber(u8),
    ToolbarInactiveNumber(u8),
    TileInit(u8),
    TileGuess(u8),
    TileConflict(u8),
}

pub const TILE_NUMBER_HEIGHT: u32 = 15;
pub const TILE_NUMBER_WIDTH: u32 = 15;
pub const TOOLBAR_BUTTON_HEIGHT: u32 = 9;
pub const TOOLBAR_BUTTON_WIDTH: u32 = 53;
pub const TOOLBAR_NUMBER_WIDTH: u32 = 13;
pub const TOOLBAR_UNDO_REDO_WIDTH: u32 = 8;

pub struct GfxLib<'a> {
    pub renderer: Renderer<'a>,
    texture: Texture,
    lib: HashMap<Res, Rect>,
}

impl<'a> GfxLib<'a> {
    pub fn new(renderer: Renderer<'a>) -> GfxLib<'a> {
        let texture = match GfxLib::load_texture(&renderer) {
            None => panic!("Error loading sudoku.png"),
            Some(t) => t
        };

        let mut lib = HashMap::new();

        lib.insert(Res::ToolbarSudoku,
                Rect::new( 0,  0, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarUndo,
                Rect::new(54,  0, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarRedo,
                Rect::new(64,  0, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarActivePencil,
                Rect::new( 0, 10, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarInactivePencil,
                Rect::new( 0, 20, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarActiveCrossOut,
                Rect::new(54, 10, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarInactiveCrossOut,
                Rect::new(54, 20, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));

        for v in 1..9+1 {
            let x = ((TOOLBAR_NUMBER_WIDTH - 1) as i32) * ((v - 1) as i32);

            lib.insert(Res::ToolbarActiveNumber(v),
                    Rect::new(x, 30, TOOLBAR_NUMBER_WIDTH, TOOLBAR_BUTTON_HEIGHT));
            lib.insert(Res::ToolbarInactiveNumber(v),
                    Rect::new(x, 40, TOOLBAR_NUMBER_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        }

        for v in 1..9+1 {
            let x = (TILE_NUMBER_WIDTH * (v - 1)) as i32;

            lib.insert(Res::TileInit(v as u8),
                    Rect::new(x, 50, TILE_NUMBER_WIDTH, TILE_NUMBER_HEIGHT));
            lib.insert(Res::TileGuess(v as u8),
                    Rect::new(x, 65, TILE_NUMBER_WIDTH, TILE_NUMBER_HEIGHT));
            lib.insert(Res::TileConflict(v as u8),
                    Rect::new(x, 80, TILE_NUMBER_WIDTH, TILE_NUMBER_HEIGHT));
        }

        GfxLib {
            renderer: renderer,
            texture: texture,
            lib: lib
        }
    }

    fn load_texture(renderer: &Renderer<'a>) -> Option<Texture> {
        let bmp = Path::new("resource/sudoku.png");
        if let Ok(t) = renderer.load_texture(bmp) {
            return Some(t);
        }

        match env::current_exe() {
            Err(e) => println!("{}", e),

            Ok(mut exe_path) => {
                exe_path.set_file_name("sudoku.png");
                match renderer.load_texture(exe_path.as_path()) {
                    Err(e) => println!("{}", e),
                    Ok(t) => return Some(t)
                }
            }
        }

        None
    }

    pub fn draw(&mut self, res: Res, dst: Rect) {
        if let Some(&src) = self.lib.get(&res) {
            self.renderer.copy(&self.texture, Some(src), Some(dst));
        }
    }
}
