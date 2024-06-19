use std::path::PathBuf;
use std::convert::TryFrom;

use rand::Rng;

use animation::{Animation, load_animation, glyph_from_animation, Size, Position, blank_animation};
use color_glyph::ColorGlyph;
use error::error;
use open_json::open_json;

pub struct Tank {
    pub dynamic_size: bool,
    pub size: Size,
    pub depth: usize,
    pub fg_anim: Animation,
    pub fg_frame: usize,
    pub bg_anim: Animation,
    pub bg_frame: usize,
}
impl Tank {
    pub fn new(path: &PathBuf, name: &str) -> Self {
        let tank_json = open_json(path, name, "tank");
        let mut depth: usize = 0; 
        let fg_anim = load_animation(&tank_json, &format!("tank {}", name), "/foreground_animation");
        let bg_anim = load_animation(&tank_json, &format!("tank {}", name), "/background_animation");

        if fg_anim[0].len() != bg_anim[0].len() || fg_anim[0][0].len() != bg_anim[0][0].len() {
            error(&format!("tank {} has a mismatich in foreground and background size", name), 1);
        } 
        if tank_json["depth"].is_u64() {
            depth = usize::try_from(tank_json["depth"].as_u64().unwrap()).unwrap();
        }
    
        let mut rng = rand::thread_rng();
        return Self {
            size: Size {height: fg_anim[0].len(), width: fg_anim[0][0].len()},
            dynamic_size: false,
            depth,
            fg_frame: rng.gen_range(0..fg_anim.len()),
            fg_anim,
            bg_frame: rng.gen_range(0..bg_anim.len()),
            bg_anim,
        }
    }
    pub fn update(&mut self) {
        if self.dynamic_size {
            let terminal_size = crossterm::terminal::size().unwrap();
            let new_size = Size{width: terminal_size.0 as usize, height: (terminal_size.1 - 1) as usize};
            if self.size != new_size {
                self.size = new_size;
                self.fg_anim = blank_animation(new_size);
                self.bg_anim = blank_animation(new_size);
            }
        }
        self.fg_frame += 1;
        self.bg_frame += 1;
        if self.fg_frame >= self.fg_anim.len() { self.fg_frame = 0; }
        if self.bg_frame >= self.bg_anim.len() { self.bg_frame = 0; }
    }
    pub fn get_fg_glyph(&mut self, row_idx: usize, glyph_idx: usize) -> Option<ColorGlyph> {
        let position = Position{x: 0, y: 0};
        if let Some(glyph) = glyph_from_animation(&self.fg_anim, self.fg_frame, row_idx, glyph_idx, position) {
            if glyph.glyph != ' ' {
                return Some(glyph);
            }
        }
        return None;
    }
    pub fn get_bg_glyph(&mut self, row_idx: usize, glyph_idx: usize) -> Option<ColorGlyph> {
        let position = Position{x: 0, y: 0};
        return glyph_from_animation(&self.bg_anim, self.bg_frame, row_idx, glyph_idx, position);
    }
}
