use std::process::exit;

pub fn error(msg: &str, code: i32) {
    println!("{}", msg);
    exit(code);
}
