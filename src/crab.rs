use std::path::PathBuf;
use std::convert::TryFrom;

use rand::Rng;

use tank::Tank;
use animation::{Animation, load_animation, Size, Position, glyph_from_animation, PositionRange};
use color_glyph::*;
use error::error;
use open_json::open_json;

pub struct Crab {
    pos: Position,
    dest: Position,
    size: Size,
    buoyancy: usize,
    pos_range: PositionRange,
    flip: bool,
    frame: usize,
    crab_anim: Animation,
    flip_anim: Animation,
}

impl Crab {
    pub fn new(path: &PathBuf, name: &str, tank: &Tank) -> Self {
        let crab_json: serde_json::Value = open_json(path, name, "crab"); 
        let crab_anim = load_animation(&crab_json, &format!("crab {}", name), "/forward_animation");
        let flip_anim = load_animation(&crab_json, &format!("crab {}", name), "/flipped_animation");
        let size = Size { height: crab_anim[0].len(), width: crab_anim[0][0].len() };
        let mut buoyancy: usize = 0;

        if crab_json["buoyancy"].is_u64() {
            buoyancy = usize::try_from(crab_json["buoyancy"].as_u64().unwrap()).unwrap();
        } else if !crab_json["buoyancy"].is_null() {
            error(&format!("crab {} /buoyancy is not a whole number", name), 1); 
        }

        let pos_range = PositionRange {
            x: 0..=tank.size.width - size.width,
            y: tank.size.height - size.height..=tank.size.height - size.height, // goofy
        };

        if crab_anim.len() != flip_anim.len(){
            error(&format!("crab {} has a mismatch in crab and flip length", name), 1);
        }
        if crab_anim[0].len() != flip_anim[0].len() || crab_anim[0][0].len() != flip_anim[0][0].len() {
            error(&format!("crab {} has a mismatch in crab and flip size", name), 1);
        }
        // TODO fix to just check size like fish
        if tank.depth < buoyancy {
            error(&format!("crab {} does not fit in the tank\n try adding depth to the tank for headroom", name), 1);
        }

        let mut rng = rand::thread_rng();
        return Self {
            pos:        random_position(&pos_range),
            dest:       random_position(&pos_range),
            size,
            buoyancy,
            pos_range,
            flip:       rng.gen::<bool>(),
            frame:      rng.gen_range(0..crab_anim.len()),
            crab_anim,
            flip_anim,
        }
    }
    pub fn update(&mut self, tank: &Tank) {
        self.frame += 1;
        if self.frame == self.crab_anim.len() {
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
                y: tank.size.height - self.size.height..=tank.size.height - self.size.height, // goofy
            };
            self.dest = random_position(&self.pos_range);
        }
    }

    pub fn get_glyph(&self, row_idx: usize, glyph_idx: usize) -> Option<ColorGlyph> {
        let mut animation: &Animation = &self.crab_anim;
        if self.flip {
            animation = &self.flip_anim;
        }
        if let Some(glyph) = glyph_from_animation(animation, self.frame, row_idx, glyph_idx, self.pos) {
            if glyph.glyph != ' ' {
                return Some(glyph);
            }
        }
        return None;
    }
}

fn random_position(pos_range: &PositionRange) -> Position {
    let mut rng = rand::thread_rng(); 
    return Position {
        x: rng.gen_range(pos_range.x.clone()),
        y: rng.gen_range(pos_range.y.clone()),
    };
}

