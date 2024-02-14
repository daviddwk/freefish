use std::path::PathBuf;
use std::convert::TryFrom;

use rand::Rng;

use animation::{Animation, load_animation, Size};
use error::error;
use open_json::open_json;

pub struct Tank {
    pub size: Size,
    pub depth: usize,
    pub fg_frame: usize,
    pub bg_frame: usize,
    pub fg_anim: Animation,
    pub bg_anim: Animation
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
            depth,
            fg_frame: rng.gen_range(0..fg_anim.len()),
            bg_frame: rng.gen_range(0..bg_anim.len()),
            fg_anim, 
            bg_anim,
        }
    }
    pub fn update(&mut self) {
        self.fg_frame += 1;
        self.bg_frame += 1;
        if self.fg_frame >= self.fg_anim.len() { self.fg_frame = 0; }
        if self.bg_frame >= self.bg_anim.len() { self.bg_frame = 0; }
    }
}
