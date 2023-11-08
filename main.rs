use std::env;

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

    while let Some(mut arg) = args_iter.next() {
        if arg.eq("-h") {
            help_arg = true;
        } else if arg.eq("-l") {
            list_arg = true;
        } else if arg.eq("-s") {
            if let Some(next_arg) = args_iter.next() {
                if next_arg.chars().next() != Some('-') {
                    arg = next_arg;
                    speed_arg = arg.clone();
                }
            }
        } else if arg.eq("-t") {
            if let Some(next_arg) = args_iter.next() {
                if next_arg.chars().next() != Some('-') {
                    arg = next_arg;
                    tank_arg = arg.clone();
                }
            }
        } else if arg.eq("-f") {
            while let Some(next_arg) = args_iter.next() {
                if next_arg.chars().next() == Some('-') {
                    break;
                }
                arg = next_arg;
                fish_args.push(arg.clone()); 
            }
        } else {
            println!("invalid argument {}", arg);
            std::process::exit(1);
        }
    }
    
    let mut tank: Tank = Tank::new("tank".to_string());
    let mut fish: Fish = Fish::new("fish".to_string(), (2, 3));

    //loop {
        print!("\x1B[2J\x1B[1;1H");
        for i in 0..tank.size.0 {
            for j in 0..tank.size.1 {
                print!("{}", tank.anim[tank.frame][i].as_bytes()[j]);
            }
            print!("\n");
        } 
    //}
    
}
