use std::path::PathBuf;
use std::fs::File;
use error;

pub fn open_json (path: &PathBuf, name: &str, asset_type: &str) -> serde_json::Value{
    match File::open(path.join(format!("{}.json", name))) {
        Ok(f) => 
            match serde_json::from_reader(f) {
                Ok(j) => return j,
                Err(_e) => error(&format!("{} file {}.json is not proper json", asset_type, name), 1).into(),
            },
        Err(_e) => error(&format!("could not open {} file {}.json", asset_type, name), 1).into(),
    }
}
