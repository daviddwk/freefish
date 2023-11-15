use load_file::*;
use color_glyph::ColorGlyph;

pub struct Tank {
    pub size: (usize, usize),
    pub frame: usize,
    pub anim: Vec<Vec<Vec<ColorGlyph>>>,
}

impl Tank {
    pub fn new(name: String) -> Self {
        let tank_frames = load_file(name + ".tank");
        return Self {
            size: (tank_frames[0].len(), tank_frames[0][0].len()),
            frame: 0,
            anim: tank_frames, 
        }
    }
    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }
}
