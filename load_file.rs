use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_file(name: String) -> (Vec<Vec<String>>, usize, usize) {    
    let mut i: usize = 0;
    let mut file_out = Vec::new();
    let mut file_loc = name.clone() + &i.to_string();
    let mut file_path = Path::new(&file_loc);
    let mut empty: bool = true;
    println!("{}", file_path.display());
    while file_path.exists() { 
        if let Ok(lines) = read_lines(file_path) {
            let mut frame = Vec::new();
            for line in lines {
                if let Ok(l) = line {
                    frame.push(l);
                    empty = false;
                } 
            }
            file_out.push(frame)
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
                panic!("mismatch num_chars in {}", &name);
            }
        }
    }
    return (file_out, num_lines, num_chars);
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    return Ok(io::BufReader::new(file).lines())
}
