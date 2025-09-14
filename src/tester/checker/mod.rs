use std::fs;
use std::path::{Path, PathBuf};
use crate::tester::compilator::{Compilator, CompileResult};
use crate::tester::runner::{Runner, RunResult};

pub struct Checker {
    c: Compilator,
    r: Runner,
}

impl Checker {
    pub fn new() -> Self {
        Self { c: Compilator::new(), r: Runner::new() }
    }

    fn first_error_line(s: &str) -> String {
        for ln in s.lines() {
            let t = ln.trim();
            if t.contains(": error[") || t.starts_with("error[") || t.contains(": error:") { return t.to_string(); }
        }
        s.lines().next().unwrap_or("").to_string()
    }

    fn is_rs(p: &Path) -> bool {
        p.extension().and_then(|x| x.to_str()) == Some("rs")
    }

    pub fn run_out_dir(&self, out_dir: &Path, timeout_ms: u64) {
        if let Ok(rd) = fs::read_dir(out_dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_file() && Self::is_rs(&p) {
                    self.proc_one(&p, timeout_ms);
                }
            }
        }
    }

    fn proc_one(&self, file: &Path, timeout_ms: u64) {
        let cr: CompileResult = self.c.compile(file);
        if !cr.ok {
            println!("compile err: {} [{:.3}s] {}", file.display(), cr.time_s, Self::first_error_line(&cr.stderr));
            return;
        }
        let rr: RunResult = self.r.run(&cr.bin_path, timeout_ms);
        if rr.ok {
            println!("ok: {} [compile {:.3}s, run {:.3}s]", file.display(), cr.time_s, rr.time_s);
        } else if rr.timed_out {
            println!("run timeout: {} [compile {:.3}s, run {:.3}s]", file.display(), cr.time_s, rr.time_s);
        } else {
            println!("run err: {} [code {}, compile {:.3}s, run {:.3}s] {}", file.display(), rr.code, cr.time_s, rr.time_s, Self::first_error_line(&rr.stderr));
        }
    }
}
