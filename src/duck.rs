use std::path::PathBuf;
use std::convert::TryFrom;

use rand::Rng;

use tank::Tank;
use animation::{Animation, load_animation, Size, Position, glyph_from_animation, PositionRange};
use color_glyph::*;
use error::error;
use open_json::open_json;

pub struct Duck {
    pos: Position,
    dest: Position,
    size: Size,
    buoyancy: usize,
    pos_range: PositionRange,
    flip: bool,
    frame: usize,
    duck_anim: Animation,
    flip_anim: Animation,
}

impl Duck {
    pub fn new(path: &PathBuf, name: &str, tank: &Tank) -> Self {
        let duck_json: serde_json::Value = open_json(path, name, "duck"); 
        let duck_anim = load_animation(&duck_json, &format!("duck {}", name), "/forward_animation");
        let flip_anim = load_animation(&duck_json, &format!("duck {}", name), "/flipped_animation");
        let size = Size { height: duck_anim[0].len(), width: duck_anim[0][0].len() };
        let mut buoyancy: usize = 0;

        if duck_json["buoyancy"].is_u64() {
            buoyancy = usize::try_from(duck_json["buoyancy"].as_u64().unwrap()).unwrap();
        } else if !duck_json["buoyancy"].is_null() {
            error(&format!("duck {} /buoyancy is not a whole number", name), 1); 
        }

        let pos_range = PositionRange {
            x: 0..=tank.size.width - size.width,
            y: tank.depth - buoyancy..=tank.depth - buoyancy, // goofy
        };

        if duck_anim.len() != flip_anim.len(){
            error(&format!("duck {} has a mismatch in duck and flip length", name), 1);
        }
        if duck_anim[0].len() != flip_anim[0].len() || duck_anim[0][0].len() != flip_anim[0][0].len() {
            error(&format!("duck {} has a mismatch in duck and flip size", name), 1);
        }
        if tank.depth < buoyancy {
            error(&format!("duck {} does not fit in the tank\n try adding depth to the tank for headroom", name), 1);
        }

        let mut rng = rand::thread_rng();
        return Self {
            pos:        random_position(&pos_range),
            dest:       random_position(&pos_range),
            size,
            buoyancy,
            pos_range,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..duck_anim.len()),
            duck_anim,
            flip_anim,
        }
    }
    pub fn update(&mut self, tank: &Tank) {
        self.frame += 1;
        if self.frame == self.duck_anim.len() {
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
                y: tank.depth - self.buoyancy..=tank.depth - self.buoyancy, // goofy
            };
            self.dest = random_position(&self.pos_range);
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<&ColorGlyph> {
        let glyph: Option<&ColorGlyph>;
        if self.flip {
            glyph = glyph_from_animation(&self.flip_anim, self.frame, row_idx, glyph_idx, self.pos);
        } else {
            glyph = glyph_from_animation(&self.duck_anim, self.frame, row_idx, glyph_idx, self.pos);
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

