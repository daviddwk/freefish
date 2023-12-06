extern crate rand;
use self::rand::Rng;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use load_file::*;
use tank::*;
use load_file::*;
use color_glyph::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    tank_depth: usize,
    tank_size: (usize, usize),
    flip: bool,
    frame: usize,
    fish_anim: Vec<Vec<Vec<ColorGlyph>>>,
    flip_anim: Vec<Vec<Vec<ColorGlyph>>>,
}

impl Fish {
    pub fn new(name: String, tank: &Tank) -> Self {
        let fish_file = File::open(name.clone() + ".json")
            .expect("file should open");
        let json: serde_json::Value = serde_json::from_reader(fish_file)
            .expect("file should be JSON");
        let anim_symbols = json.pointer("/animation/symbols")
            .expect("file should have animation/symbols key");
        let anim_colors = json.pointer("/animation/colors")
            .expect("file should have animation/colors key");
        let anim_highlights = json.pointer("/animation/highlights")
            .expect("file should have animation/highlights key");
        let flip_symbols = json.pointer("/flipped_animation/symbols")
            .expect("file should have flipped_animation/symbols key");
        let flip_colors = json.pointer("/flipped_animation/colors")
            .expect("file should have flipped_animation/colors key");
        let flip_highlights = json.pointer("/flipped_animation/highlights")
            .expect("file should have flipped_animation/highlights key");
        
        let fish_frames = load_animation(anim_symbols, anim_colors, anim_highlights);
        let flip_frames = load_animation(flip_symbols, flip_colors, flip_highlights);

        if fish_frames.len() != flip_frames.len() ||
           fish_frames[0].len() != flip_frames[0].len() ||
           fish_frames[0][0].len() != flip_frames[0][0].len()
        {
            panic!("{} mismatch fish and flip size", name);
        }
        if fish_frames.len() != flip_frames.len(){
            panic!("{} mismatch fish and flip number of frames", name);
        }
        let mut rng = rand::thread_rng();
        return Self {
            pos:        (rng.gen_range(tank.depth..tank.size.0), rng.gen_range(0..tank.size.1)),
            dest:       (rng.gen_range(tank.depth..tank.size.0), rng.gen_range(0..tank.size.1)),
            size:       (fish_frames[0].len(), fish_frames[0][0].len()),
            tank_size:  tank.size,
            tank_depth: tank.depth,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..fish_frames.len()),
            fish_anim:  fish_frames,
            flip_anim:  flip_frames,
        }
    }
    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame == self.fish_anim.len() {
            self.frame = 0;
        }
        if self.pos.0 < self.dest.0 {
            self.pos.0 += 1;
        } else if self.pos.0 > self.dest.0 {
            self.pos.0 -= 1;
        }
        if self.pos.1 < self.dest.1 {
            self.pos.1 += 1;
            self.flip = false;
        } else if self.pos.1 > self.dest.1 {
            self.pos.1 -= 1;
            self.flip = true;
        }
        if self.pos == self.dest {
            let mut rng = rand::thread_rng();
            self.dest = (
                rng.gen_range(self.tank_depth..self.tank_size.0), 
                rng.gen_range(0..self.tank_size.1)
            );
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<&ColorGlyph> {

        if row_idx >= self.size.0 + self.pos.0 || row_idx < self.pos.0 ||
           glyph_idx >= self.size.1 + self.pos.1 || glyph_idx < self.pos.1
        {
            return None;
        }

        let glyph: &ColorGlyph;
        if self.flip {
            glyph = &self.flip_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1];
        } else {
            glyph = &self.fish_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1];
        }

        if glyph.glyph == ' '  {
            return None;
        }

        return Some(glyph);
    }
}

