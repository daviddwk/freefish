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
        stdout().execute(SetForegroundColor(Color::Reset)).unwrap();
        stdout().execute(SetBackgroundColor(Color::Reset)).unwrap();
        if let Some(fg) = self.foreground_color  {
            stdout().execute(SetForegroundColor(fg)).unwrap();
        }
        if let Some(bg) = self.background_color {
            stdout().execute(SetBackgroundColor(bg)).unwrap();
        }
        print!("{}", self.glyph);
    }
}
