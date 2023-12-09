extern crate rand;
use self::rand::Rng;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use load_file::*;
use tank::*;
use load_file::*;
use color_glyph::*;
use glyph_from_animation::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    wait: usize,
    flip: bool,
    idle: bool,
    frame: usize,
    fish_anim: Vec<Vec<Vec<ColorGlyph>>>,
    flip_anim: Vec<Vec<Vec<ColorGlyph>>>,
    idle_anim: Vec<Vec<Vec<ColorGlyph>>>
}

impl Fish {
    pub fn new(name: &str, tank: &Tank) -> Self {
        let fish_file = File::open(format!("{}.json", name))
            .expect(&format!("{}.json should open", name));
        let fish_json: serde_json::Value = serde_json::from_reader(fish_file)
            .expect(&format!("{}.json should be JSON", name));
        
        let fish_anim = load_animation(&fish_json, &format!("fish {}", name), "/animation");
        let flip_anim = load_animation(&fish_json, &format!("fish {}", name), "/flipped_animation");
        let mut idle_anim = Vec::new();
        let mut idle = false;
        if !fish_json["idle_animation"].is_null() {
            idle = true;
            idle_anim = load_animation(&fish_json, &format!("fish {}", name), "/idle_animation");
        }
        if fish_anim.len() != flip_anim.len() ||
           fish_anim[0].len() != flip_anim[0].len() ||
           fish_anim[0][0].len() != flip_anim[0][0].len()
        {
            panic!("{} mismatch fish and flip size", name);
        }
        if fish_anim.len() != flip_anim.len(){
            panic!("{} mismatch fish and flip number of frames", name);
        }

        let mut rng = rand::thread_rng();
        let fish_size = (fish_anim[0].len(), fish_anim[0][0].len());
        return Self {
            pos:        (rng.gen_range(0 + tank.depth..=tank.size.0 - fish_size.0),
                         rng.gen_range(0..=tank.size.1 - fish_size.1)),
            dest:       (rng.gen_range(0 + tank.depth..=tank.size.0 - fish_size.0),
                         rng.gen_range(0..=tank.size.1 - fish_size.1)),
            size:       fish_size,
            wait:       0,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..fish_anim.len()),
            idle:       idle,
            fish_anim:  fish_anim,
            flip_anim:  flip_anim,
            idle_anim:  idle_anim
        }
    }
    pub fn update(&mut self, tank: &Tank) {
        self.frame += 1;
        if self.frame == self.fish_anim.len() {
            self.frame = 0;
        }
        if self.wait == 0 {
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
                self.wait = 5;
            }
        } else {
            self.wait -= 1;
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<&ColorGlyph> {
        /*
        if row_idx >= self.size.0 + self.pos.0 || row_idx < self.pos.0 ||
           glyph_idx >= self.size.1 + self.pos.1 || glyph_idx < self.pos.1
        {
            return None;
        }

        let glyph: &ColorGlyph;
        if self.wait != 0 && self.idle == true {
            return Some(&self.idle_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1]); 
        } else if self.flip {
            glyph = &self.flip_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1];
        } else {
            glyph = &self.fish_anim[self.frame][row_idx - self.pos.0][glyph_idx - self.pos.1];
        }

        if glyph.glyph == ' '  {
            return None;
        }
        */
        if self.wait != 0 && self.idle == true {
            return glyph_from_animation(&self.idle_anim,
                self.frame, row_idx, glyph_idx, self.pos);
        } else if self.flip {
            return glyph_from_animation(&self.flip_anim,
                self.frame, row_idx, glyph_idx, self.pos);
        } else {
            return glyph_from_animation(&self.fish_anim,
                self.frame, row_idx, glyph_idx, self.pos);
        }
    }
}

