use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    style::{Color, SetForegroundColor, SetBackgroundColor}
};

pub struct ColorGlyph {
    pub glyph: char,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

impl ColorGlyph {
    pub fn print(&self) {
        if let Err(e) = stdout().execute(SetForegroundColor(Color::Reset)) { panic!("{}", e); }
        if let Err(e) = stdout().execute(SetBackgroundColor(Color::Reset)) { panic!("{}", e); }
        if let Some(fg) = self.foreground_color  {
            if let Err(e) = stdout().execute(SetForegroundColor(fg)) { panic!("{}", e); }
        }
        if let Some(bg) = self.background_color {
            if let Err(e) = stdout().execute(SetBackgroundColor(bg)) { panic!("{}", e); }
        }
        print!("{}", self.glyph);
    }
}
