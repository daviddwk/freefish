use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_file(name: String) -> Vec<Vec<String>> {    
    let mut file_out = Vec::new();
    let mut i: usize = 0;
    let mut file_loc = name.clone() + &i.to_string();
    let mut file_path = Path::new(&file_loc);

    while file_path.exists() { 
        println!("{}{}", name.clone(), i);
        if let Ok(lines) = read_lines(file_path) {
            let mut frame = Vec::new();
            for line in lines {
                if let Ok(l) = line {
                    frame.push(l);
                } 
            }
            file_out.push(frame)
        }
        i += 1;
        file_loc = name.clone() + &i.to_string();
        file_path = Path::new(&file_loc); 
    }
    return file_out;
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    return Ok(io::BufReader::new(file).lines())
}
