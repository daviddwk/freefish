extern crate rand;
use self::rand::Rng;
use std::fs::File;
extern crate serde_json;
use self::serde_json::*;
use home::*;
use load_file::*;


extern crate crossterm;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo},
    terminal::Clear,
    style::{Color, Colors, Print, SetColors, SetForegroundColor, SetBackgroundColor}
};

use load_file::*;
use color_glyph::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    tank_size: (usize, usize),
    flip: bool,
    frame: usize,
    fish_anim: Vec<Vec<Vec<ColorGlyph>>>,
    flip_anim: Vec<Vec<Vec<ColorGlyph>>>,
}

impl Fish {
    pub fn new(name: String, position: (usize, usize), tnk_size: (usize, usize)) -> Self {
        let file = File::open(name.clone() + ".json")
            .expect("file should open");
        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("file should be JSON");
        let anim_symbols = json.pointer("/animation/symbols")
            .expect("file should have animation/symbols key");
        let anim_colors = json.pointer("/animation/colors")
            .expect("file should have animation/colors key");
        let flip_symbols = json.pointer("/flipped_animation/symbols")
            .expect("file should have flipped_animation/symbols key");
        let flip_colors = json.pointer("/flipped_animation/colors")
            .expect("file should have flipped_animation/colors key");

        
        let fish_frames = load_animation(anim_symbols, anim_colors);
        //let fish_frames = load_file(home_dir().unwrap().to_str().unwrap().to_owned() + 
        //                            "/.config/freefish/fish/" + &name.clone() + "/fish");
        let flip_frames = load_file(home_dir().unwrap().to_str().unwrap().to_owned() + 
                                    "/.config/freefish/fish/" + &name.clone() + "/flip");
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
            pos: position, // rand input
            dest: position,
            size: (fish_frames[0].len(), fish_frames[0][0].len()), // load
            tank_size: tnk_size,
            flip: rng.gen::<bool>(),
            frame: rng.gen_range(0..fish_frames.len()), // rand
            fish_anim: fish_frames, //load
            flip_anim: flip_frames, //load
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
            self.dest = (rng.gen_range(0..(self.tank_size.0 - self.size.0)), 
                         rng.gen_range(0..(self.tank_size.1 - self.size.1)));
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

