use std::env;
use std::thread;
use std::time;

mod fish;
mod tank;
mod load_file;

use fish::*;
use tank::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut help_arg: bool = false;
    let mut list_arg: bool = false;
    let mut speed_arg = String::new();
    let mut tank_arg = String::new();
    let mut fish_args: Vec<String> = Vec::new();
    
    let mut args_iter = args.iter().skip(1);

    while let Some(arg) = args_iter.next() {
        if arg.eq("-h") {
            help_arg = true;
        } else if arg.eq("-l") {
            list_arg = true;
        } else if arg.eq("-s") {
            let mut tmp_args_iter = args_iter.clone();
            if let Some(arg) = tmp_args_iter.next() {
                if arg.chars().next() != Some('-') {
                    args_iter.next();
                    speed_arg = arg.clone();
                }
            }
        } else if arg.eq("-t") {
            let mut tmp_args_iter = args_iter.clone();
            if let Some(arg) = tmp_args_iter.next() {
                if arg.chars().next() != Some('-') {
                    args_iter.next();
                    tank_arg = arg.clone();
                }
            }
        } else if arg.eq("-f") {
            let mut tmp_args_iter = args_iter.clone();
            while let Some(arg) = tmp_args_iter.next() {
                if arg.chars().next() != Some('-') {
                    args_iter.next();
                    fish_args.push(arg.clone()); 
                }
            }
        } else {
            println!("invalid argument {}", arg);
            std::process::exit(1);
        }
    }
    
    if help_arg {
        println!("Help text");
        std::process::exit(0);
    }

    let tank: Tank = Tank::new(tank_arg);
    let mut fishies: Vec<Fish> = Vec::new();
    for arg in fish_args {
        fishies.push(Fish::new(arg, (1, 1), tank.get_size()));
    }
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        for row_idx in 0..tank.size.0 {
            for glyph_idx in 0..tank.size.1 {
                let mut printed = false;
                for fish_idx in 0..fishies.len() {
                    if let Some(glyph) = fishies[fish_idx].get_glyph(row_idx, glyph_idx) {
                        print!{"{}", glyph};
                        printed = true;
                        break;
                    }
                }
                if !printed {
                    print!("{}", tank.anim[tank.frame][row_idx].chars().nth(glyph_idx).unwrap());
                }
            }
            print!("\n");
        }
        for fish_idx in 0..fishies.len() {
            fishies[fish_idx].update();
        }
        thread::sleep(time::Duration::from_millis(200));
    }
}
