use std::path::PathBuf;

extern crate serde_json;
extern crate rand;
use self::rand::Rng;

use tank::Tank;
use animation::{Animation, load_animation, glyph_from_animation, Position, Size, PositionRange};
use color_glyph::ColorGlyph;
use error::error;
use open_json::open_json;

pub struct Fish {
    pos: Position,
    dest: Position,
    size: Size,
    pos_range: PositionRange,
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
        // pos_range seems to have replaced the need for size in the struct
        let size = Size { height: fish_anim[0].len(), width: fish_anim[0][0].len() };
        let pos_range = PositionRange {
            x: 0..=tank.size.width - size.width,
            y: 0 + tank.depth..=tank.size.height - size.height,
        };
        let mut rng = rand::thread_rng(); 

        if fish_anim.len() != flip_anim.len(){
            error(&format!("fish {} has a mismatch in fish and flip length", name), 1);
        }
        if fish_anim[0].len() != flip_anim[0].len() || fish_anim[0][0].len() != flip_anim[0][0].len() {
            error(&format!("fish {} has a mismatch in fish and flip size", name), 1);
        }
        
        if tank.size.height <= size.height + tank.depth || tank.size.width <= size.width {
            error(&format!("fish {} too large for tank", name), 1);
        }

        return Self {
            pos:        random_position(&pos_range),
            dest:       random_position(&pos_range),
            size,
            pos_range,
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
        if self.pos.y < self.dest.y {
            self.pos.y += 1;
        } else if self.pos.y > self.dest.y {
            self.pos.y -= 1;
        }
        if self.pos.x < self.dest.x {
            self.pos.x += 1;
            self.flip = false;
        } else if self.pos.x > self.dest.x {
            self.pos.x -= 1;
            self.flip = true;
        }
        if self.pos == self.dest {
            self.pos_range = PositionRange {
                x: 0..=tank.size.width - self.size.width,
                y: 0 + tank.depth..=tank.size.height - self.size.height,
            };
            self.dest = random_position(&self.pos_range);
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<&ColorGlyph> {
        let glyph: Option<&ColorGlyph>;
        if self.flip {
            glyph = glyph_from_animation(&self.flip_anim, self.frame, row_idx, glyph_idx, self.pos);
        } else {
            glyph = glyph_from_animation(&self.fish_anim, self.frame, row_idx, glyph_idx, self.pos);
        }
        if glyph.is_some() && glyph.unwrap().glyph == ' ' {
            return None;
        }
        return glyph;        
    }
}

fn random_position(pos_range: &PositionRange) -> Position {
    let mut rng = rand::thread_rng(); 
    return Position {
        x: rng.gen_range(pos_range.x.clone()),
        y: rng.gen_range(pos_range.y.clone()),
    };
}

