// resource.rs

use std::collections::HashMap;
use std::env;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2::render::{Renderer,Texture};

#[cfg(feature = "flic")]
use flic;
#[cfg(feature = "flic")]
use sdl2::pixels::PixelFormatEnum;
#[cfg(feature = "flic")]
use sdl2::render::BlendMode;

#[cfg(feature = "png")]
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
            None => panic!("Error loading sudoku.flc or sudoku.png"),
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
        if let Some(t) = try_load_flic(renderer) {
            return Some(t);
        }
        if let Some(t) = try_load_png(renderer) {
            return Some(t);
        }

        None
    }

    pub fn draw(&mut self, res: Res, dst: Rect) {
        if let Some(&src) = self.lib.get(&res) {
            let _ = self.renderer.copy(&self.texture, Some(src), Some(dst));
        }
    }
}

/*--------------------------------------------------------------*/

#[cfg(not(feature = "flic"))]
fn try_load_flic(_: &Renderer) -> Option<Texture> {
    None
}

#[cfg(feature = "flic")]
fn try_load_flic(renderer: &Renderer) -> Option<Texture> {
    let path = Path::new("resource/sudoku.flc");
    if let Some(t) = try_load_flic2(renderer, &path) {
        return Some(t);
    }

    if let Ok(mut path) = env::current_exe() {
        path.set_file_name("sudoku.flc");
        if let Some(t) = try_load_flic2(renderer, &path) {
            return Some(t);
        }
    }

    None
}

#[cfg(feature = "flic")]
fn try_load_flic2(renderer: &Renderer, path: &Path) -> Option<Texture> {
    if let Ok(mut f) = flic::FlicFile::open(path) {
        let w = f.width() as usize;
        let h = f.height() as usize;
        let mut buf = vec![0; w * h];
        let mut pal = [0; 3 * 256];

        let res = f.read_next_frame(
                &mut flic::RasterMut::new(w, h, &mut buf, &mut pal));
        if res.is_err() {
            return None;
        }

        let texture = renderer.create_texture_streaming(
                PixelFormatEnum::ABGR8888, w as u32, h as u32);
        if texture.is_err() {
            return None;
        }

        let mut t = texture.unwrap();
        render_to_texture(&mut t, w, h, &buf, &pal);
        t.set_blend_mode(BlendMode::Blend);
        return Some(t);
    }

    None
}

#[cfg(feature = "flic")]
fn render_to_texture(
        texture: &mut Texture,
        w: usize, h: usize, buf: &[u8], pal: &[u8]) {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..h {
            for x in 0..w {
                let offset = pitch * y + 4 * x;
                let c = buf[w * y + x] as usize;

                buffer[offset + 0] = pal[3 * c + 0];
                buffer[offset + 1] = pal[3 * c + 1];
                buffer[offset + 2] = pal[3 * c + 2];
                buffer[offset + 3] = if c == 0 { 0 } else { 255 };
            }
        }
    }).unwrap();
}

/*--------------------------------------------------------------*/

#[cfg(not(feature = "png"))]
fn try_load_png(_: &Renderer) -> Option<Texture> {
    None
}

#[cfg(feature = "png")]
fn try_load_png(renderer: &Renderer) -> Option<Texture> {
    let path = Path::new("resource/sudoku.png");
    if let Ok(t) = renderer.load_texture(&path) {
        return Some(t);
    }

    if let Ok(mut path) = env::current_exe() {
        path.set_file_name("sudoku.png");
        if let Ok(t) = renderer.load_texture(&path) {
            return Some(t);
        }
    }

    None
}
