extern crate rand;
use self::rand::Rng;
use std::fs::File;
extern crate serde_json;
use std::path::PathBuf;
use self::serde_json::*;
use load_file::*;
use tank::*;
use load_file::*;
use color_glyph::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    flip: bool,
    frame: usize,
    fish_anim: Animation,
    flip_anim: Animation, 
}

impl Fish {
    pub fn new(path: &PathBuf, name: &str, tank: &Tank) -> Self {
        let fish_file = File::open(path.join(format!("{}.json", name)))
            .expect(&format!("{}.json should open", name));
        let fish_json: serde_json::Value = serde_json::from_reader(fish_file)
            .expect(&format!("{}.json should be JSON", name));
        
        let fish_anim = load_animation(&fish_json, &format!("fish {}", name), "/animation");
        let flip_anim = load_animation(&fish_json, &format!("fish {}", name), "/flipped_animation");

        if fish_anim.len() != flip_anim.len(){
            panic!("{} mismatch fish and flip number of frames", name);
        }
        if fish_anim[0].len() != flip_anim[0].len() || fish_anim[0][0].len() != flip_anim[0][0].len() {
            panic!("{} mismatch fish and flip size", name);
        }

        let mut rng = rand::thread_rng(); 
        let size = (fish_anim[0].len(), fish_anim[0][0].len());
        return Self {
            pos:        (rng.gen_range(0 + tank.depth..=tank.size.0 - size.0),
                         rng.gen_range(0..=tank.size.1 - size.1)),
            dest:       (rng.gen_range(0 + tank.depth..=tank.size.0 - size.0),
                         rng.gen_range(0..=tank.size.1 - size.1)),
            size,       
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..fish_anim.len()),
            fish_anim,
            flip_anim, 
        }
    }
    pub fn update(&mut self, tank: &Tank) {
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
            self.dest = (rng.gen_range(0 + tank.depth..=tank.size.0 - self.size.0),
                         rng.gen_range(0..=tank.size.1 - self.size.1));
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<&ColorGlyph> {
        let glyph: Option<&ColorGlyph>;
        if self.flip {
            glyph = glyph_from_animation(&self.flip_anim,
                self.frame, row_idx, glyph_idx, self.pos);
        } else {
            glyph = glyph_from_animation(&self.fish_anim,
                self.frame, row_idx, glyph_idx, self.pos);
        }
        if glyph.is_some() && glyph.unwrap().glyph == ' ' {
            return None;
        }
        return glyph;        
    }
}
