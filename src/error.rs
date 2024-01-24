use std::process::exit;
use colored::Colorize;

pub fn error(msg: &str, code: i32) {
    println!("{} {}", "error:".red().bold(), msg);
    exit(code);
}
