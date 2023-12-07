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
    pub fn new(name: String, tank: &Tank) -> Self {
        let duck_file = File::open(name.clone() + ".json")
            .expect("file should open");
        let duck_json: serde_json::Value = serde_json::from_reader(duck_file)
            .expect("file should be JSON");
        let anim_json = duck_json.pointer("/animation")
            .expect("file should have animation");
        let flip_json = duck_json.pointer("/flipped_animation")
            .expect("file should have flipped_animation");
        let mut bouyancy: usize = 0;
        if duck_json["depth"].is_u64() {
            bouyancy = usize::try_from(duck_json["depth"].as_u64().unwrap()).unwrap();
            println!("{}fdfdffd", bouyancy + tank.depth);
        }
        
        let duck_frames = load_animation(anim_json);
        let flip_frames = load_animation(flip_json);

        if duck_frames.len() != flip_frames.len() ||
           duck_frames[0].len() != flip_frames[0].len() ||
           duck_frames[0][0].len() != flip_frames[0][0].len()
        {
            panic!("{} mismatch duck and flip size", name);
        }
        if duck_frames.len() != flip_frames.len(){
            panic!("{} mismatch duck and flip number of frames", name);
        }
        let mut rng = rand::thread_rng();
        return Self {
            pos:        (tank.depth - bouyancy, rng.gen_range(0..tank.size.1)),
            dest:       (tank.depth - bouyancy, rng.gen_range(0..tank.size.1)),
            size:       (duck_frames[0].len(), duck_frames[0][0].len()),
            bouyancy:   bouyancy,
            tank_size:  tank.size,
            tank_depth: tank.depth,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..duck_frames.len()),
            duck_anim:  duck_frames,
            flip_anim:  flip_frames,
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

