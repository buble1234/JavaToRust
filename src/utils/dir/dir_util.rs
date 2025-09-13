use std::fs;
use std::path::Path;

pub fn create_output_dir() {
    fs::create_dir_all("out").unwrap();
}

pub fn check_input_dir() -> bool {
    let input_dir = Path::new("input");
    input_dir.exists()
}