use std::fs;
use std::path::Path;

pub fn create_output_dir<P: AsRef<Path>>(p: P) {
    let _ = fs::create_dir_all(p);
}

pub fn check_input_dir<P: AsRef<Path>>(p: P) -> bool {
    p.as_ref().is_dir()
}
