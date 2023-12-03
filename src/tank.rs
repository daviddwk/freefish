use load_file::*;
use color_glyph::ColorGlyph;
use home::*;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use rand::Rng;
pub struct Tank {
    pub size: (usize, usize),
    pub frame: usize,
    pub fg_anim: Vec<Vec<Vec<ColorGlyph>>>,
    pub bg_anim: Vec<Vec<Vec<ColorGlyph>>>
}
impl Tank {
    pub fn new(name: String) -> Self {
        let tank_file = File::open(name.clone() + ".json")
            .expect("tank file should open");
        let tank_json: serde_json::Value = 
            serde_json::from_reader(tank_file)
            .expect("file should be JSON");
        let foreground_symbols = tank_json.pointer("/foreground/symbols")
            .expect("file should have foreground/symbols key");
        let foreground_colors = tank_json.pointer("/foreground/colors")
            .expect("file should have foreground/colors key");
        let background_symbols = tank_json.pointer("/background/symbols")
            .expect("file should have background/symbols key");
        let background_colors = tank_json.pointer("/background/colors")
            .expect("file should have background/colors key");
        
        let foreground_animation = load_animation(foreground_symbols, foreground_colors);
        let background_animation = load_animation(background_symbols,background_colors);
    
        let tank_frames = load_file(
            home_dir().unwrap().to_str().unwrap().to_owned() + 
            "/.config/freefish/tank/" + &name.clone() + "/tank"
        );
        let mut rng = rand::thread_rng();
        return Self {
            size: (tank_frames[0].len(), tank_frames[0][0].len()),
            frame: rng.gen_range(0..tank_frames.len()),
            fg_anim: foreground_animation, 
            bg_anim: background_animation
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }
    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame >= self.bg_anim.len() {
            self.frame = 0;
        }
    }
}
