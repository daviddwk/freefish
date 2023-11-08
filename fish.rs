use load_file::*;

pub struct Fish {
    pos: (usize, usize),
    dest: (usize, usize),
    size: (usize, usize),
    flip: bool,
    frame: usize,
    fish_chars: Vec<Vec<String>>,
    flip_chars: Vec<Vec<String>>,
}

impl Fish {
    pub fn new(name: String, position: (usize, usize)) -> Self {
        return Self {
            pos: position, // rand input
            dest: position,
            size: (0, 0), // load
            flip: false,
            frame: 0, // rand
            fish_chars: load_file(name.clone() + ".fish"), //load
            flip_chars: load_file(name.clone() + ".flip"), //load
        }
    }
}

