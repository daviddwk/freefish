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

extern crate structopt;
use structopt::StructOpt;

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


#[derive(StructOpt)]
#[structopt(name = "freefish", about = "Displays an animated fish tank to your terminal!")]
struct Opt {
    //#[structopt(short = "h", long = "help")]
    //help: bool,
    #[structopt(short = "l", long = "list", help = "Lists available assets found in ~/.config/freefish/")]
    list: bool,
    #[structopt(short = "i", long = "init", help = "Copies assets from ./config to ~/.config/freefish/")]
    init: bool,
    #[structopt(short = "s", long = "speed", default_value = "200", help = "Sets the delay between frames in ms")]
    speed: u64,
    #[structopt(short = "f", long = "fish", help = "Adds the specified fish to your fish tank")]
    fish: Vec<String>,
    #[structopt(short = "d", long = "ducks", help = "Adds the specified ducks to your fish tank")]
    ducks: Vec<String>,
    #[structopt(short = "t", long = "tank", help = "Selects a tank")]
    tank: String, 

}

fn main() {
    let args = Opt::from_args();
    let freefish_dir = home::home_dir().unwrap().join(".config").join("freefish");
    let fish_dir = freefish_dir.join("fish");
    let tanks_dir = freefish_dir.join("tanks");
    let ducks_dir = freefish_dir.join("ducks");
    
    /*
    if args.help {
        println!("Help text");
        exit(0);
    }
    */

    if args.init {
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

    if args.list {
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
    
    if args.tank.is_empty() { 
        error("No tank was selected", 1);
    }
    let mut tank: Tank = Tank::new(&tanks_dir, &args.tank);
    let mut fishies: Vec<Fish> = Vec::new();
    let mut duckies: Vec<Duck> = Vec::new();

    for arg in args.fish {
        fishies.push(Fish::new(&fish_dir, &arg, &tank));
    }
    for arg in args.ducks {
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
        let frame_duration = Duration::from_millis(args.speed);
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
