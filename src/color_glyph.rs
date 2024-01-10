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
        std::io::stdout().execute(SetForegroundColor(Color::Reset));
        std::io::stdout().execute(SetBackgroundColor(Color::Reset));
        if let Some(fg) = self.foreground_color  {
            std::io::stdout().execute(SetForegroundColor(fg));
        }
        if let Some(bg) = self.background_color {
            std::io::stdout().execute(SetBackgroundColor(bg));
        }
        print!("{}", self.glyph);
    }
}
