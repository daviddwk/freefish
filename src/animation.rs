extern crate serde_json;
use crossterm::style::Color;

use color_glyph::ColorGlyph;

pub type Animation = Vec<Vec<Vec<ColorGlyph>>>;

pub fn load_animation(json: &serde_json::Value, name: &str, anim_key: &str) -> Animation {
    let mut out_anim: Animation = Vec::new();
    let symbols = json.pointer(&format!("{}/symbols", anim_key))
        .expect(&format!("{} should have symbols key", anim_key));
    let colors = json.pointer(&format!("{}/colors", anim_key))
        .expect(&format!("{} should have colors key", anim_key));
    let highlights = json.pointer(&format!("{}/highlights", anim_key))
        .expect(&format!("{} should have highlights key", anim_key));

    check_format(&symbols, "symbols");
    check_format(&colors, "colors");
    check_format(&highlights, "highlights");

    let num_frames = symbols.as_array().unwrap().len();
    let num_lines = symbols[0].as_array().unwrap().len();
    let num_symbols = symbols[0][0].as_str().unwrap().len();

    check_array(&symbols, num_frames, &format!("{} symbols", name));
    check_array(&colors, num_frames, &format!("{} colors", name));
    check_array(&highlights, num_frames, &format!("{} highlights", name));

    for frame_idx in 0..num_frames {
        let mut out_frame: Vec<Vec<ColorGlyph>> = Vec::new();

        check_array(&symbols[frame_idx], num_lines, 
                    &format!("{} symbols[{}]", name, frame_idx));
        check_array(&colors[frame_idx], num_lines, 
                    &format!("{} colors[{}]", name, frame_idx));
        check_array(&highlights[frame_idx], num_lines, 
                    &format!("{} highlights[{}]", name, frame_idx));

        for line_idx in 0..num_lines {
            let mut out_line: Vec<ColorGlyph> = Vec::new();

            check_string(&symbols[frame_idx][line_idx], num_symbols, 
                         &format!("{} symbols[{}][{}]", name, frame_idx, line_idx));
            check_string(&colors[frame_idx][line_idx], num_symbols,
                         &format!("{} colors[{}][{}]", name, frame_idx, line_idx));
            check_string(&highlights[frame_idx][line_idx], num_symbols,
                         &format!("{} highlights[{}][{}]", name, frame_idx, line_idx));

            let line = symbols[frame_idx][line_idx].as_str().unwrap();

            for symbol_idx in 0..num_symbols {
                out_line.push(ColorGlyph{
                    glyph: line.chars().nth(symbol_idx).unwrap(),
                    foreground_color: match_color( 
                        colors
                        .as_array().unwrap()[frame_idx]
                        .as_array().unwrap()[line_idx]
                        .as_str().unwrap().chars().nth(symbol_idx).unwrap()
                    ),
                    background_color: match_color( 
                        highlights
                        .as_array().unwrap()[frame_idx]
                        .as_array().unwrap()[line_idx]
                        .as_str().unwrap().chars().nth(symbol_idx).unwrap()
                    )
                });
            }
            out_frame.push(out_line);
        }
        out_anim.push(out_frame);
    }
    return out_anim;
}

fn check_format(json_array: &serde_json::Value, name: &str) {
    if !json_array.is_array() { 
        panic!("{} is not an array", name);
    }
    if !json_array[0].is_array() { 
        panic!("{}[0] is not an array", name); 
    }
    if !json_array[0][0].is_string() { 
        panic!("{}[0][0] is not a string", name); 
    }
    if !json_array[0][0].as_str().unwrap().len() == 0 {
        panic!("{}[0][0] is an empty string", name)
    }
}

fn check_array(json_array: &serde_json::Value, target_size: usize, name: &str) {
    if !json_array.is_array() { 
        panic!("{} is not an array", name); 
    }
    if json_array.as_array().unwrap().len() != target_size {
        panic!("{} differs in length", name);
    }
}

fn check_string(json_string: &serde_json::Value, target_size: usize, name: &str) {
    if !json_string.is_string() { 
        panic!("{} is not a string", name); 
    }
    if json_string.as_str().unwrap().len() != target_size {
        panic!("{} differs in length", name);
    }
}

fn match_color(color: char) -> Option<Color> {
    match color {
        'a'=> return Some(Color::DarkGrey),
        'r'=> return Some(Color::Red),
        'g'=> return Some(Color::Green),
        'y'=> return Some(Color::Yellow),
        'b'=> return Some(Color::Blue),
        'm'=> return Some(Color::Magenta),
        'c'=> return Some(Color::Cyan),
        'w'=> return Some(Color::White),

        'A'=> return Some(Color::Black),
        'R'=> return Some(Color::DarkRed),
        'G'=> return Some(Color::DarkGreen),
        'Y'=> return Some(Color::DarkYellow),
        'B'=> return Some(Color::DarkBlue),
        'M'=> return Some(Color::DarkMagenta),
        'C'=> return Some(Color::DarkCyan),
        'W'=> return Some(Color::Grey),

         _ => return None
    }
}

pub fn glyph_from_animation(anim: &Vec<Vec<Vec<ColorGlyph>>>, frame_idx: usize, row_idx: usize, glyph_idx: usize, position: (usize, usize)) -> Option<&ColorGlyph> {
    let frame_idx_oob = frame_idx >= anim.len();
    if frame_idx_oob {
        panic!("Attempted to access frame out of bounds")
    }
    let row_idx_oob = (row_idx < position.0) || (row_idx - position.0 >= anim[frame_idx].len());
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

