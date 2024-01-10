use std::fs::File;
use std::path::PathBuf;
use std::convert::TryFrom;

extern crate serde_json;
use rand::Rng;

use tank::Tank;
use animation::{Animation, load_animation};
use color_glyph::*;

pub struct Duck {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    bouyancy: usize,
    tank_depth: usize,
    tank_size: (usize, usize),
    flip: bool,
    frame: usize,
    duck_anim: Animation,
    flip_anim: Animation,
}

impl Duck {
    pub fn new(path: &PathBuf, name: &str, tank: &Tank) -> Self {
        let duck_file = File::open(path.join(format!("{}.json", name)))
            .expect(&format!("{}.json should open", name));
        let duck_json: serde_json::Value = serde_json::from_reader(duck_file)
            .expect(&format!("{}.json should be JSON", name));
        
        let duck_anim = load_animation(&duck_json, &format!("tank {}", name), "/animation");
        let flip_anim = load_animation(&duck_json, &format!("tank {}", name), "/flipped_animation");

        let mut bouyancy: usize = 0;
        if duck_json["depth"].is_u64() {
            bouyancy = usize::try_from(duck_json["depth"].as_u64().unwrap()).unwrap();
        }

        if duck_anim.len() != flip_anim.len(){
            panic!("{} mismatch duck and flip number of frames", name);
        }
        if duck_anim[0].len() != flip_anim[0].len() || duck_anim[0][0].len() != flip_anim[0][0].len() {
            panic!("{} mismatch duck and flip size", name);
        }
        
        let mut rng = rand::thread_rng();
        return Self {
            pos:        (tank.depth - bouyancy, rng.gen_range(0..tank.size.1)),
            dest:       (tank.depth - bouyancy, rng.gen_range(0..tank.size.1)),
            size:       (duck_anim[0].len(), duck_anim[0][0].len()),
            bouyancy:   bouyancy,
            tank_size:  tank.size,
            tank_depth: tank.depth,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..duck_anim.len()),
            duck_anim:  duck_anim,
            flip_anim:  flip_anim,
        }
    }
    pub fn update(&mut self) {
        self.frame += 1;
        if self.frame == self.duck_anim.len() {
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
                self.tank_depth - self.bouyancy, 
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
            glyph = &self.duck_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1];
        }

        if glyph.glyph == ' '  {
            return None;
        }

        return Some(glyph);
    }
}

