use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use color_glyph::*;
use crossterm::style::Color;
extern crate serde_json;
use self::serde_json::*;

pub fn load_animation(animation_symbols: &Value, animation_colors: &Value) -> Vec<Vec<Vec<ColorGlyph>>> {
    let mut out_animation: Vec<Vec<Vec<ColorGlyph>>> = Vec::new();
    
    if !animation_symbols.is_array() { 
        println!("symbols is not array");
    }
    let num_frames = animation_symbols.as_array().unwrap().len();
    if !animation_symbols[0].is_array() { 
        println!("symbols[0] is not array"); 
    }
    let num_lines = animation_symbols[0].as_array().unwrap().len();
    if !animation_symbols[0][0].is_string() { 
        println!("symbols[0][0] is not a string"); 
    }
    let num_symbols = animation_symbols[0][0].as_str().unwrap().len();

    for frame_idx in 0..num_frames {
        let mut out_frame: Vec<Vec<ColorGlyph>> = Vec::new();
        if !animation_symbols[frame_idx].is_array() { 
            println!("symbols[{}] is not an array", frame_idx); 
        }
        if animation_symbols[frame_idx].as_array().unwrap().len() != num_lines {
            println!("symbols[{}] differs in length from symbols[0]", frame_idx);
        }
        for line_idx in 0..num_lines {
            let mut out_line: Vec<ColorGlyph> = Vec::new();
            if !animation_symbols[frame_idx][line_idx].is_string() { 
                println!("symbols[{}][{}] is not a string", frame_idx, line_idx); 
            }
            let line = animation_symbols[frame_idx][line_idx].as_str().unwrap();
            if line.len() != num_symbols {
                println!("symbols[{}][{}] differs in length from symbols[0][0]", frame_idx, line_idx);
            } 
            for symbol_idx in 0..num_symbols {
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
