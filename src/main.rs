use std::env::args;
use std::process::exit;
use std::time::{Duration, SystemTime};
use std::io::stdout;
use std::ffi::OsStr;
use std::fs::{
    create_dir, 
    create_dir_all,
    read_dir, 
    copy
};

extern crate crossterm;
use crossterm::{
    ExecutableCommand,
    cursor::{
        Hide, 
        Show, 
        MoveTo
    },
    terminal::{
        Clear, 
        disable_raw_mode, 
        enable_raw_mode
    },
    event::{
        Event, 
        poll, 
        read, 
        KeyCode, 
        KeyEvent, 
        KeyModifiers, 
        KeyEventKind, 
        KeyEventState
    },
};
extern crate rand;
extern crate home;
extern crate serde_json;

mod tank;
use tank::Tank;
mod fish;
use fish::Fish;
mod duck;
use duck::Duck;
mod animation;
mod color_glyph;
mod error;
use error::error;
mod open_json;

fn main() {
    let args: Vec<String> = args().collect();

    let mut help_arg: bool = false;
    let mut list_arg: bool = false;
    let mut init_arg: bool = false;
    let mut speed_arg: u64 = 200;
    let mut tank_arg = String::new();
    let mut fish_args: Vec<String> = Vec::new();
    let mut duck_args: Vec<String> = Vec::new();
    
    let freefish_dir = home::home_dir().unwrap().join(".config").join("freefish");
    let fish_dir = freefish_dir.join("fish");
    let tanks_dir = freefish_dir.join("tanks");
    let ducks_dir = freefish_dir.join("ducks");
    
    let mut args_iter = args.iter().skip(1);

    while let Some(arg) = args_iter.next() {
        if arg.eq("-h") {
            help_arg = true;
        } else if arg.eq("-i") {
            init_arg = true;
        } else if arg.eq("-l") {
            list_arg = true;
        } else if arg.eq("-s") {
            let mut tmp_args_iter = args_iter.clone();
            if let Some(arg) = tmp_args_iter.next() {
                if arg.chars().next() != Some('-') {
                    args_iter.next();
                    match arg.parse::<u64>() {
                        Err(_e) => error("Invalid argument supplied with -s", 1),
                        Ok(a) => speed_arg = a,
                    }
                    if speed_arg == 0 {
                        error("Invalid argument supplied with -s", 1);
                    }
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
                if arg.chars().next() == Some('-') { break; }
                args_iter.next();
                fish_args.push(arg.clone()); 
            }
        } else if arg.eq("-d") {
            let mut tmp_args_iter = args_iter.clone();
            while let Some(arg) = tmp_args_iter.next() {
                if arg.chars().next() == Some('-') { break; }
                args_iter.next();
                duck_args.push(arg.clone()); 
            }
        } else {
            println!("invalid argument {}\ntry -h for help", arg);
            exit(1);
        }
    }
    
    if help_arg {
        println!("Help text");
        exit(0);
    }

    if init_arg {
        create_dir_all(freefish_dir.clone()).unwrap();
        for asset_dir in [&fish_dir, &tanks_dir, &ducks_dir] {
            if !asset_dir.exists() {
                create_dir(asset_dir).unwrap();
            }
        }

        let fish_files = read_dir("./config/fish").unwrap();
        let tank_files = read_dir("./config/tanks").unwrap();
        let duck_files = read_dir("./config/ducks").unwrap();

        for asset_files in [(fish_files, &fish_dir), (tank_files, &tanks_dir), (duck_files, &ducks_dir)] {
            for file in asset_files.0 {
                match file {
                    Err(e) => panic!("{}", e),
                    Ok(f) => copy(&f.path(), asset_files.1.join(f.file_name())).unwrap(),
                };
            }
        }
        exit(0);
    }

    if list_arg {
        let fish_files = read_dir("./config/fish").unwrap();
        let tank_files = read_dir("./config/tanks").unwrap();
        let duck_files = read_dir("./config/ducks").unwrap();

        for asset_files in [(fish_files, "fish"), (tank_files, "tanks"), (duck_files, "ducks")] {
            println!("{}", asset_files.1);
            for file in asset_files.0 {
                match file {
                    Err(e) => panic!("{}", e),
                    Ok(f) =>
                        if f.path().extension() == Some(OsStr::new("json")) {
                            // should use file_prefix instead of file_name, but is experimental
                            println!(" {}", f.path().file_name().unwrap().to_str().unwrap()); 
                        },
                };
            }
        }
        exit(0);
    }
    
    if tank_arg.is_empty() { 
        error("No tank was selected", 1);
    }
    let mut tank: Tank = Tank::new(&tanks_dir, &tank_arg);
    let mut fishies: Vec<Fish> = Vec::new();
    let mut duckies: Vec<Duck> = Vec::new();

    for arg in fish_args {
        fishies.push(Fish::new(&fish_dir, &arg, &tank));
    }
    for arg in duck_args {
        duckies.push(Duck::new(&ducks_dir, &arg, &tank));
    }
    
    enable_raw_mode().unwrap();

    stdout().execute(Hide).unwrap();
    stdout().execute(Clear(crossterm::terminal::ClearType::All)).unwrap();
    'render_loop: loop {
        stdout().execute(MoveTo(0, 0)).unwrap();
        for row_idx in 0..tank.size.0 {
            for glyph_idx in 0..tank.size.1 {
                let mut printed = false;
                if tank.fg_anim[tank.fg_frame][row_idx][glyph_idx].glyph != ' ' {
                    tank.fg_anim[tank.fg_frame][row_idx][glyph_idx].print(); 
                    printed = true;
                }
                // combine fishies and ducks vector to reduce this copied code
                if !printed {
                    for duck_idx in 0..duckies.len() {
                        if let Some(glyph) = duckies[duck_idx].get_glyph(row_idx, glyph_idx) {
                            glyph.print();
                            printed = true;
                            break;
                        }
                    }
                }
                if !printed {
                    for fish_idx in 0..fishies.len() {
                        if let Some(glyph) = fishies[fish_idx].get_glyph(row_idx, glyph_idx) {
                            glyph.print();
                            printed = true;
                            break;
                        }
                    }
                }
                if !printed {
                    tank.bg_anim[tank.bg_frame][row_idx][glyph_idx].print();
                }
            }
            if row_idx != tank.size.0 - 1 {
                print!("\r\n");
            }
        }
        for fish_idx in 0..fishies.len() {
            fishies[fish_idx].update(&tank);
        }
        for duck_idx in 0..duckies.len() {
            duckies[duck_idx].update();
        }
        tank.update();
        
        // there must be a better way
        let frame_duration = Duration::from_millis(speed_arg);
        let frame_start = SystemTime::now();
        let mut now = SystemTime::now();
        while now.duration_since(frame_start).unwrap() < frame_duration {
            if poll(frame_duration - now.duration_since(frame_start).unwrap()).unwrap() {
                match read().unwrap() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => break 'render_loop,
                    Event::Key(KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => break 'render_loop,
                    _ => (),
                }
            }
            now = SystemTime::now();
        }
    }
    
    stdout().execute(Show).unwrap();
    disable_raw_mode().unwrap();
    exit(0);
}
