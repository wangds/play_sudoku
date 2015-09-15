// resource.rs

use std::collections::HashMap;
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
}

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
        let bmp = Path::new("resource/sudoku.png");
        let texture = match renderer.load_texture(bmp) {
            Err(e) => panic!("{}", e),
            Ok(t) => t
        };

        let mut lib = HashMap::new();

        lib.insert(Res::ToolbarSudoku,
                Rect::new_unwrap( 0,  0, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarUndo,
                Rect::new_unwrap(54,  0, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarRedo,
                Rect::new_unwrap(64,  0, TOOLBAR_UNDO_REDO_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarActivePencil,
                Rect::new_unwrap( 0, 10, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarInactivePencil,
                Rect::new_unwrap( 0, 20, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarActiveCrossOut,
                Rect::new_unwrap(54, 10, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        lib.insert(Res::ToolbarInactiveCrossOut,
                Rect::new_unwrap(54, 20, TOOLBAR_BUTTON_WIDTH, TOOLBAR_BUTTON_HEIGHT));

        for v in 1..9+1 {
            let x = ((TOOLBAR_NUMBER_WIDTH - 1) as i32) * ((v - 1) as i32);

            lib.insert(Res::ToolbarActiveNumber(v),
                    Rect::new_unwrap(x, 30, TOOLBAR_NUMBER_WIDTH, TOOLBAR_BUTTON_HEIGHT));
            lib.insert(Res::ToolbarInactiveNumber(v),
                    Rect::new_unwrap(x, 40, TOOLBAR_NUMBER_WIDTH, TOOLBAR_BUTTON_HEIGHT));
        }

        GfxLib {
            renderer: renderer,
            texture: texture,
            lib: lib
        }
    }

    pub fn draw(&mut self, res: Res, dst: Rect) {
        if let Some(&src) = self.lib.get(&res) {
            self.renderer.copy(&self.texture, Some(src), Some(dst));
        }
    }
}
