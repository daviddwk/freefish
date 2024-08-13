use std::process::exit;
use std::time::{Duration, SystemTime};
use std::io::stdout;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all,read_dir, copy};
extern crate structopt;
use structopt::StructOpt;

extern crate crossterm;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show, MoveTo},
    terminal::{Clear, disable_raw_mode, enable_raw_mode},
    event::{Event, poll, read, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState},
};
extern crate rand;
extern crate home;
extern crate colored;
extern crate serde_json;

mod tank;
use tank::Tank;
mod fish;
use fish::Fish;
mod duck;
use duck::Duck;
mod crab;
use crab::Crab;
mod animation;
use animation::{Size, blank_animation};
mod color_glyph;
use color_glyph::{ColorGlyph, EMPTY_COLOR_GLYPH};
mod error;
use error::error;
mod open_json;


#[derive(StructOpt)]
#[structopt(name = "freefish", version = "0.0.1", about = "Displays an animated fish tank to your terminal!")]
struct Opt {
    #[structopt(short = "t", long = "tank", help = "(REQUIRED) Selects a tank")]
    tank: Vec<String>, 
    #[structopt(short = "f", long = "fish", help = "Adds the specified fish to your fish tank")]
    fish: Vec<String>,
    #[structopt(short = "d", long = "ducks", help = "Adds the specified ducks to your fish tank")]
    ducks: Vec<String>,
    #[structopt(short = "c", long = "crabs", help = "Adds the specified crabs to your fish tank")]
    crabs: Vec<String>,
    #[structopt(short = "s", long = "speed", default_value = "200", help = "Sets the delay between frames in ms")]
    speed: u64,
    #[structopt(short = "l", long = "list", help = "Lists available assets found in ~/.config/freefish/")]
    list: bool,
    #[structopt(short = "i", long = "init", help = "Copies assets from ./config to ~/.config/freefish/")]
    init: bool,
}

struct Creatures {
    fishies: Vec<Fish>,
    duckies: Vec<Duck>,
    crabies: Vec<Crab>,
}

fn main() {
    let args = Opt::from_args();
    // init assets
    let mut creatures;
    let mut tank;
    {
        let freefish_dir = home::home_dir().unwrap().join(".config").join("freefish");
        let init_dir = PathBuf::from("./config");

        let mut asset_names: HashMap<&str, &Vec<String>> = HashMap::new();
        asset_names.insert("tanks", &args.tank);
        asset_names.insert("fish", &args.fish);
        asset_names.insert("ducks", &args.ducks);
        asset_names.insert("crabs", &args.crabs);

        if args.init {
            init_assets(&init_dir, &freefish_dir, &asset_names);
        }
        if args.list {
            list_assets(&freefish_dir, &asset_names);
        }

        tank = load_tank(&freefish_dir, asset_names["tanks"]); 
        creatures = Creatures {
            fishies: asset_names["fish"].iter().map(|name| Fish::new(&freefish_dir.join("fish"), name, &tank)).collect(),
            duckies: asset_names["ducks"].iter().map(|name| Duck::new(&freefish_dir.join("ducks"), name, &tank)).collect(),
            crabies: asset_names["crabs"].iter().map(|name| Crab::new(&freefish_dir.join("crabs"), name, &tank)).collect(),
        };
    }
    // init terminal
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    stdout().execute(crossterm::terminal::DisableLineWrap).unwrap();
    stdout().execute(Clear(crossterm::terminal::ClearType::All)).unwrap();
    
    // main loop
    loop {
        let frame = build_frame(&mut tank, &mut creatures);
        // post processing goes here;
        print_frame(&frame);
        update_animations(&mut tank, &mut creatures);
        if poll_input(Duration::from_millis(args.speed)) {
            break;
        }
    }
    
    // return terminal to regular state
    stdout().execute(crossterm::terminal::EnableLineWrap).unwrap();
    stdout().execute(Show).unwrap();
    disable_raw_mode().unwrap();
    exit(0);
}

fn init_assets(from_dir: &PathBuf, to_dir: &PathBuf, assets: &HashMap<&str, &Vec<String>>) {
    create_dir_all(to_dir).unwrap();
    for (asset_type, _v) in assets {
        if !to_dir.join(asset_type).exists() {
            create_dir(to_dir.join(asset_type)).unwrap();
        }
        for file in read_dir(from_dir.join(asset_type)).unwrap() {
            let f = &file.unwrap();
            copy(f.path(), to_dir.join(asset_type).join(f.file_name())).unwrap();
        }
    }
    // print somthing here
    exit(0);
}

fn list_assets(asset_dir: &PathBuf, assets: &HashMap<&str, &Vec<String>>) {
    // TODO fix listing with no directories
    // and other things where this is no freefish dir
    for (asset_type, _v) in assets {
        println!("{}:", asset_type.to_uppercase());
        for file in read_dir(asset_dir.join(asset_type)).unwrap() {
            let f = &file.unwrap();
            if f.path().extension() == Some(OsStr::new("json")) {
                println!("    {}", f.path().file_stem().unwrap().to_str().unwrap()); 
            }
        }
    }
    exit(0);
}

fn load_tank(assets_dir: &PathBuf, tank_names: &Vec<String>) -> Tank {
    if tank_names.len() < 1 {
        let terminal_size = crossterm::terminal::size().unwrap();
        let tank_size = Size{width: terminal_size.0 as usize, height: (terminal_size.1 - 1) as usize};
        return Tank{
            size: tank_size,
            dynamic_size: true,
            depth: 0,
            fg_anim: blank_animation(tank_size),
            fg_frame: 0,
            bg_anim: blank_animation(tank_size),
            bg_frame: 0,
        }
    }
    if tank_names.len() > 1 {
        error("Too many tanks were provided", 1);
    }
    return Tank::new(&assets_dir.join("tanks"), &tank_names[0]);
}

fn build_frame(tank: &mut Tank, creatures: &mut Creatures) -> Vec<Vec<ColorGlyph>> {
    let mut frame_buffer = vec![vec![EMPTY_COLOR_GLYPH; tank.size.width]; tank.size.height];
    for row_idx in 0..tank.size.height {
        for glyph_idx in 0..tank.size.width {
            let mut glyph: Option<ColorGlyph> = tank.get_fg_glyph(row_idx, glyph_idx);
            if glyph.is_none() {
                for duck in creatures.duckies.iter() {
                    glyph = duck.get_glyph(row_idx, glyph_idx);
                    if glyph.is_some() { break; }
                }
            }
            if glyph.is_none() {
                for crab in creatures.crabies.iter() {
                    glyph = crab.get_glyph(row_idx, glyph_idx);
                    if glyph.is_some() { break; }
                }
            }
            if glyph.is_none() {
                for fish in creatures.fishies.iter() {
                    glyph = fish.get_glyph(row_idx, glyph_idx);
                    if glyph.is_some() { break; }
                }
            }
            if glyph.is_none() {
                glyph = tank.get_bg_glyph(row_idx, glyph_idx);
            }
            if glyph.is_none() {
                error(&format!("build_frame found no glyph at index [{}][{}]", row_idx, glyph_idx), 1);
            } 
            frame_buffer[row_idx][glyph_idx] = glyph.unwrap();
        }
    }
    return frame_buffer;
}

fn print_frame(frame_buffer: &Vec<Vec<ColorGlyph>>) {
    stdout().execute(MoveTo(0, 0)).unwrap();
    for frame in frame_buffer.iter() {
        for glyph in frame.iter() {
            glyph.print();
        }
        print!("\r\n");
    }

}

fn update_animations(tank: &mut Tank, creatures: &mut Creatures) { 
    tank.update();
    for duck in &mut creatures.duckies {
        duck.update(&tank);
    }
    for fish in &mut creatures.fishies {
        fish.update(&tank);
    }
    for crab in &mut creatures.crabies {
        crab.update(&tank);
    }
}

fn poll_input(duration: Duration) -> bool {
    let frame_start = SystemTime::now();
    let mut now = SystemTime::now();
    while now.duration_since(frame_start).unwrap() < duration {
        if poll(duration - now.duration_since(frame_start).unwrap()).unwrap() {
            match read().unwrap() {
                // TODO: Capital Q does not work here
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => return true,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => return true,
                _ => (),
            }
        }
        now = SystemTime::now();
    }
    return false;
}
