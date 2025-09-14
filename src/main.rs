mod utils;
mod tester;

use std::path::Path;
use utils::dir::dir_utils::{create_output_dir, check_input_dir};
use utils::process::file_processor::FileProcessor;
use tester::own::run_all;

fn main() {
    create_output_dir("out");
    if !check_input_dir("input") {
        println!("нет инпута");
        return;
    }
    let fp = FileProcessor::new();
    fp.process_directory(Path::new("input"));
    run_all(5000);
    println!("готово");
}
