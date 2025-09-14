use std::path::Path;
use crate::tester::checker::Checker;

pub fn run_all(timeout_ms: u64) {
    let ch = Checker::new();
    ch.run_out_dir(Path::new("out"), timeout_ms);
}
