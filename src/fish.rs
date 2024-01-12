use std::path::PathBuf;

extern crate serde_json;
extern crate rand;
use self::rand::Rng;

use tank::Tank;
use animation::{Animation, load_animation, glyph_from_animation};
use color_glyph::ColorGlyph;
use error::error;
use open_json::open_json;

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
        let fish_json = open_json(path, name, "fish");
        let fish_anim = load_animation(&fish_json, &format!("fish {}", name), "/forward_animation");
        let flip_anim = load_animation(&fish_json, &format!("fish {}", name), "/flipped_animation");

        if fish_anim.len() != flip_anim.len(){
            error(&format!("fish {} has a mismatch in fish and flip length", name), 1);
        }
        if fish_anim[0].len() != flip_anim[0].len() || fish_anim[0][0].len() != flip_anim[0][0].len() {
            error(&format!("fish {} has a mismatch in fish and flip size", name), 1);
        }

        let mut rng = rand::thread_rng(); 
        let size = (fish_anim[0].len(), fish_anim[0][0].len());
        
        if tank.size.0 <= size.0 + tank.depth || tank.size.1 <= size.1 {
            error(&format!("fish {} too large for tank", name), 1);
        }
        let pos_range = (0 + tank.depth..=tank.size.0 - size.0, 0..=tank.size.1 - size.1);

        return Self {
            pos:        (rng.gen_range(pos_range.0.clone()), rng.gen_range(pos_range.1.clone())),
            dest:       (rng.gen_range(pos_range.0.clone()), rng.gen_range(pos_range.1.clone())),
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
