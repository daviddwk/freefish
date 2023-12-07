extern crate rand;
use self::rand::Rng;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use load_file::*;
use tank::*;
use load_file::*;
use color_glyph::*;
use std::convert::TryFrom;

pub struct Duck {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    bouyancy: usize,
    tank_depth: usize,
    tank_size: (usize, usize),
    flip: bool,
    frame: usize,
    duck_anim: Vec<Vec<Vec<ColorGlyph>>>,
    flip_anim: Vec<Vec<Vec<ColorGlyph>>>,
}

impl Duck {
    pub fn new(name: &str, tank: &Tank) -> Self {
        let duck_file = File::open(format!("{}.json", name))
            .expect(&format!("{}.json should open", name));
        let duck_json: serde_json::Value = serde_json::from_reader(duck_file)
            .expect(&format!("{}.json should be JSON", name));
        
        let duck_anim = load_animation(&duck_json, &format!("tank {}", name), "/animation");
        let flip_anim = load_animation(&duck_json, &format!("tank {}", name), "/flipped_animation");

        let mut bouyancy: usize = 0;
        if duck_json["depth"].is_u64() {
            bouyancy = usize::try_from(duck_json["depth"].as_u64().unwrap()).unwrap();
        }
        
        /* TODO: redo with better errors

        if duck_anim.len() != flip_anim.len() ||
           duck_anim[0].len() != flip_anim[0].len() ||
           duck_anim[0][0].len() != flip_anim[0][0].len()
        {
            panic!("{}.json mismatch size of animation and flipped_animation", name);
        }
        if duck_anim.len() != flip_anim.len(){
            panic!("{}.json mismatch size of animation and flipped_animation", name);
            panic!("{} mismatch duck and flip number of frames", name);
        }
        */
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

