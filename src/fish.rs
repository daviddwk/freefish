extern crate rand;
use self::rand::Rng;

use load_file::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    tank_size: (usize, usize),
    flip: bool,
    frame: usize,
    fish_anim: Vec<Vec<String>>,
    flip_anim: Vec<Vec<String>>,
}

impl Fish {
    pub fn new(name: String, position: (usize, usize), tnk_size: (usize, usize)) -> Self {
        let (fish_frames, fish_num_lines, fish_num_chars) = load_file(name.clone() + ".fish");
        let (flip_frames, flip_num_lines, flip_num_chars) = load_file(name.clone() + ".flip");
        if fish_num_lines != flip_num_lines || flip_num_chars != flip_num_chars {
            panic!("{} mismatch fish and flip size", name);
        }
        if fish_frames.len() != flip_frames.len(){
            panic!("{} mismatch fish and flip number of frames", name);
        }
        return Self {
            pos: position, // rand input
            dest: position,
            size: (fish_num_lines, fish_num_chars), // load
            tank_size: tnk_size,
            flip: false,
            frame: 0, // rand
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
        }
        if self.pos.0 > self.dest.0 {
            self.pos.0 -= 1;
        }
        if self.pos.1 < self.dest.1 {
            self.pos.1 += 1;
        }
        if self.pos.1 > self.dest.1 {
            self.pos.1 -= 1;
        }
        if self.pos == self.dest {
            let mut rng = rand::thread_rng();
            self.dest = (rng.gen_range(0..(self.tank_size.0 - self.size.0)), rng.gen_range(0..(self.tank_size.1 - self.size.1)));
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<char> {
        if row_idx >= self.size.0 + self.pos.0 || row_idx < self.pos.0 ||
           glyph_idx >= self.size.1 + self.pos.1 || glyph_idx < self.pos.1
        {
            return None;
        }
        return Some(self.fish_anim[self.frame][row_idx - self.pos.0].chars().nth(glyph_idx - self.pos.1).unwrap());
    }
}

