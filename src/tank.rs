use load_file::*;
use color_glyph::ColorGlyph;
use home::*;
pub struct Tank {
    pub size: (usize, usize),
    pub frame: usize,
    pub anim: Vec<Vec<Vec<ColorGlyph>>>,
}

impl Tank {
    pub fn new(name: String) -> Self {
        let tank_frames = load_file(home_dir().unwrap().to_str().unwrap().to_owned() + "/.config/freefish/tank/" + &name.clone() + "/tank");
        return Self {
            size: (tank_frames[0].len(), tank_frames[0][0].len()),
            frame: 0,
            anim: tank_frames, 
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }
    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame >= self.anim.len() {
            self.frame = 0;
        }
    }
}
