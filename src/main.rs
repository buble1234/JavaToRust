mod utils;

use utils::dir::dir_utils::{create_output_dir, check_input_dir};
use utils::process::file_processor::FileProcessor;
use std::path::Path;

fn main() {
    create_output_dir();
    if !check_input_dir() {
        println!("чек инпут дира провален");
        return;
    }
    let file_processor = FileProcessor::new();
    file_processor.process_directory(Path::new("input"));
    println!("триггер на process_input_dir");
}