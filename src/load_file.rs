use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use color_glyph::*;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo},
    terminal::Clear,
    style::{Color, SetForegroundColor, SetBackgroundColor}
};
extern crate serde_json;
use self::serde_json::*;

pub fn load_file(name: String) -> Vec<Vec<Vec<ColorGlyph>>> {    
    let mut i: usize = 0;
    let mut file_out: Vec<Vec<Vec<ColorGlyph>>> = Vec::new();
    let mut file_loc = name.clone() + &i.to_string();
    let mut file_path = Path::new(&file_loc);
    let mut empty: bool = true;
    println!("{}", file_path.display());
    while file_path.exists() {
        println!("file is there yes");
        let mut curr_fg_color: Option<Color> = None;
        if let Ok(lines) = read_lines(file_path) {
            let mut new_frame: Vec<Vec<ColorGlyph>> = Vec::new();
            for line in lines {
                let mut new_line: Vec<ColorGlyph> = Vec::new();
                let chars = line.unwrap().to_string();
                let mut char_idx = 0;
                while char_idx < chars.len() {
                    if chars.chars().nth(char_idx).unwrap() == '\\' {
                        char_idx += 1;
                        if let Some(escape) = chars.chars().nth(char_idx) {
                            match escape{
                                'd'=>curr_fg_color = None,
                                'r'=>curr_fg_color = Some(Color::Red),
                                'g'=>curr_fg_color = Some(Color::Green),
                                'y'=>curr_fg_color = Some(Color::Yellow),
                                'b'=>curr_fg_color = Some(Color::Blue),
                                'm'=>curr_fg_color = Some(Color::Magenta),
                                'c'=>curr_fg_color = Some(Color::Cyan),
                                'w'=>curr_fg_color = Some(Color::White),
                                _=>panic!("invalid escape code {}", escape),
                            }
                            println!("Color {}", escape);
                        }
                    } else {
                        empty = false;
                        new_line.push(ColorGlyph{
                            glyph: chars.chars().nth(char_idx).unwrap(),
                            foreground_color: curr_fg_color,
                            background_color: None
                        });
                    }
                    char_idx += 1;
                }
                new_frame.push(new_line);
            }
            file_out.push(new_frame)
        }
        i += 1;
        file_loc = name.clone() + &i.to_string();
        file_path = Path::new(&file_loc); 
    }

    if empty{
        panic!("empty file {}", &name);
    }
    
    let num_lines: usize = file_out[0].len();
    let num_chars: usize = file_out[0][0].len();
    for frame in &file_out {
        if num_lines != frame.len(){
            panic!("mismatch num_lines in {}", &name);
        }
        for line in frame {
            if num_chars != line.len() {
                panic!("mismatch num_chars in {}: {} {}", &name, num_chars, line.len());
            }
        }
    }
    return file_out;
}

pub fn load_animation(animation_symbols: &Value, animation_colors: &Value) -> Vec<Vec<Vec<ColorGlyph>>> {
    
    // check size
    let mut out_animation: Vec<Vec<Vec<ColorGlyph>>> = Vec::new();
    for frame_idx in 0..animation_symbols.as_array().unwrap().len() {
        let mut out_frame: Vec<Vec<ColorGlyph>> = Vec::new();
        for line_idx in 0..animation_symbols[frame_idx].as_array().unwrap().len() {
            let mut out_line: Vec<ColorGlyph> = Vec::new();
            let line = animation_symbols[frame_idx][line_idx].as_str().unwrap();
            for symbol_idx in 0..line.len() {
                // color switch statement
                out_line.push(ColorGlyph{
                    glyph: line.chars().nth(symbol_idx).unwrap(),
                    foreground_color: None,
                    background_color: None
                });
            }
            out_frame.push(out_line);
        }
        out_animation.push(out_frame);
    }
    return out_animation;
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    return Ok(io::BufReader::new(file).lines())
}
