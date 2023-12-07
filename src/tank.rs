use load_file::*;
use color_glyph::ColorGlyph;
use home::*;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use rand::Rng;
use std::convert::TryFrom;
pub struct Tank {
    pub size: (usize, usize),
    pub depth: usize,
    pub fg_frame: usize,
    pub bg_frame: usize,
    pub fg_anim: Vec<Vec<Vec<ColorGlyph>>>,
    pub bg_anim: Vec<Vec<Vec<ColorGlyph>>>
}
impl Tank {
    pub fn new(name: &str) -> Self {
        let tank_file = File::open(format!("{}.json", name))
            .expect(&format!("{}.json should open", name));
        let tank_json: serde_json::Value = serde_json::from_reader(tank_file)
            .expect(&format!("{}.json should be JSON", name));
        let mut depth: usize = 0;
        
        let fg_anim = load_animation(&tank_json, &format!("tank {}", name), "/foreground");
        let bg_anim = load_animation(&tank_json, &format!("tank {}", name), "/background");
        
        if tank_json["depth"].is_u64() {
            depth = usize::try_from(tank_json["depth"].as_u64().unwrap()).unwrap();
        }
    
        let mut rng = rand::thread_rng();
        return Self {
            size:     (fg_anim[0].len(), fg_anim[0][0].len()),
            depth:    depth,
            fg_frame: rng.gen_range(0..fg_anim.len()),
            bg_frame: rng.gen_range(0..bg_anim.len()),
            fg_anim:  fg_anim, 
            bg_anim:  bg_anim
        }
    }
    pub fn update(&mut self) {
        self.fg_frame += 1;
        self.bg_frame += 1;
        if self.fg_frame >= self.fg_anim.len() { self.fg_frame = 0; }
        if self.bg_frame >= self.bg_anim.len() { self.bg_frame = 0; }
    }
}
