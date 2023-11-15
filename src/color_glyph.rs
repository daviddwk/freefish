use std::io::{self, Write};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo},
    terminal::Clear,
    style::{Color, Colors, Print, SetColors, SetForegroundColor, SetBackgroundColor}
};

pub struct ColorGlyph {
    pub glyph: char,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

impl ColorGlyph {
    pub fn print(&self) {
        io::stdout().execute(SetForegroundColor(Color::Reset));
        io::stdout().execute(SetBackgroundColor(Color::Reset));
        if let Some(fg) = self.foreground_color  {
            io::stdout().execute(SetForegroundColor(fg));
        }
        if let Some(bg) = self.background_color {
            io::stdout().execute(SetForegroundColor(bg));
        }
        print!("{}", self.glyph);
    }
}
