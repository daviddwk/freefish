use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use color_glyph::*;
use crossterm::style::Color;
extern crate serde_json;
use self::serde_json::*;

pub fn load_animation(symbols: &Value, colors: &Value, highlights: &Value) -> Vec<Vec<Vec<ColorGlyph>>> {
    let mut out_animation: Vec<Vec<Vec<ColorGlyph>>> = Vec::new();

    check_format(&symbols, "symbols");
    check_format(&colors, "colors");
    check_format(&highlights, "highlights");

    let num_frames = symbols.as_array().unwrap().len();
    let num_lines = symbols[0].as_array().unwrap().len();
    let num_symbols = symbols[0][0].as_str().unwrap().len();

    for frame_idx in 0..num_frames {
        let mut out_frame: Vec<Vec<ColorGlyph>> = Vec::new();

        check_array(&symbols[frame_idx], num_lines, 
                    &format!("symbols[{}]", frame_idx));
        check_array(&colors[frame_idx], num_lines, 
                    &format!("colors[{}]", frame_idx));
        check_array(&highlights[frame_idx], num_lines, 
                    &format!("highlights[{}]", frame_idx));

        for line_idx in 0..num_lines {
            let mut out_line: Vec<ColorGlyph> = Vec::new();

            check_string(&symbols[frame_idx][line_idx], num_symbols, 
                         &format!("symbols[{}][{}]", frame_idx, line_idx));
            check_string(&colors[frame_idx][line_idx], num_symbols);
                         &format!("colors[{}][{}]", frame_idx, line_idx));
            check_string(&highlights[frame_idx][line_idx], num_symbols);
                         &format!("highlights[{}][{}]", frame_idx, line_idx));

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
        out_animation.push(out_frame);
    }
    return out_animation;
}

pub fn check_format(json_array: &Value, name: &str) {
    if !json_array.is_array() { 
        panic!("{} is not an array", name);
    }
    if !json_array[0].is_array() { 
        panic!("{}[0] is not an array", name); 
    }
    if !json_array[0][0].is_string() { 
        panic!("{}[0][0] is not a string", name); 
    }
}

pub fn check_array(json_array: &Value, target_size: usize, name: &str) {
    if !json_array.is_array() { 
        panic!("{} is not an array", name); 
    }
    if json_array.as_array().unwrap().len() != target_size {
        panic!("{} differs in length", name);
    }
}

pub fn check_string(json_string: &Value, target_size: usize, name: &str) {
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
