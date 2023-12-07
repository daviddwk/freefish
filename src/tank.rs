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
    pub fn new(name: String) -> Self {
        let tank_file = File::open(name.clone() + ".json")
            .expect("tank file should open");
        println!("{}", name);
        let tank_json: serde_json::Value = 
            serde_json::from_reader(tank_file)
            .expect("file should be JSON");
        let foreground_json = tank_json.pointer("/foreground")
            .expect("file should have foreground");
        let background_json = tank_json.pointer("/background")
            .expect("file should have background");

        let mut depth: usize = 0;
        if tank_json["depth"].is_u64() {
            depth = usize::try_from(tank_json["depth"].as_u64().unwrap()).unwrap();
        }

        let foreground_animation = load_animation(foreground_json);
        let background_animation = load_animation(background_json);
    
        let mut rng = rand::thread_rng();
        return Self {
            size:     (foreground_animation[0].len(), foreground_animation[0][0].len()),
            depth:    depth,
            fg_frame: rng.gen_range(0..foreground_animation.len()),
            bg_frame: rng.gen_range(0..background_animation.len()),
            fg_anim:  foreground_animation, 
            bg_anim:  background_animation
        }
    }
    pub fn update(&mut self) {
        self.fg_frame += 1;
        self.bg_frame += 1;
        if self.fg_frame >= self.fg_anim.len() { self.fg_frame = 0; }
        if self.bg_frame >= self.bg_anim.len() { self.bg_frame = 0; }
    }
}
