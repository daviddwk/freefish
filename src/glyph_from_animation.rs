use::color_glyph::*;

pub fn glyph_from_animation(anim: &Vec<Vec<Vec<ColorGlyph>>>, frame_idx: usize, row_idx: usize, glyph_idx: usize, position: (usize, usize)) -> Option<&ColorGlyph> {
    let frame_idx_oob = frame_idx >= anim.len();
    if frame_idx_oob {
        panic!("Attempted to access frame out of bounds")
    }
    let row_idx_oob = (row_idx < position.0) ||
                      (row_idx - position.0 >= anim[frame_idx].len());
    if row_idx_oob {
        return None;
    }
    let glyph_idx_oob = (glyph_idx < position.1) ||
                        (glyph_idx - position.1 >= anim[frame_idx][row_idx - position.0].len()); 
    if glyph_idx_oob {
        return None;
    }
    return Some(&anim[frame_idx][row_idx - position.0][glyph_idx - position.1]);
}
