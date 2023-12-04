use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use color_glyph::*;
use crossterm::style::Color;
extern crate serde_json;
use self::serde_json::*;

pub fn load_animation(animation_symbols: &Value, animation_colors: &Value) -> Vec<Vec<Vec<ColorGlyph>>> {
    // TODO check size
    let mut out_animation: Vec<Vec<Vec<ColorGlyph>>> = Vec::new();
    for frame_idx in 0..animation_symbols.as_array().unwrap().len() {
        let mut out_frame: Vec<Vec<ColorGlyph>> = Vec::new();
        for line_idx in 0..animation_symbols[frame_idx].as_array().unwrap().len() {
            let mut out_line: Vec<ColorGlyph> = Vec::new();
            let line = animation_symbols[frame_idx][line_idx].as_str().unwrap();
            for symbol_idx in 0..line.len() {
                println!("frame_idx {} line_idx {} symbol_idx {}", frame_idx, line_idx, symbol_idx);
                out_line.push(ColorGlyph{
                    glyph: line.chars().nth(symbol_idx).unwrap(),
                    foreground_color: match_color( 
                        animation_colors.get("foreground").unwrap()
                        .as_array().unwrap()[frame_idx]
                        .as_array().unwrap()[line_idx]
                        .as_str().unwrap().chars().nth(symbol_idx).unwrap()
                    ),
                    background_color: match_color( 
                        animation_colors.get("background").unwrap()
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
