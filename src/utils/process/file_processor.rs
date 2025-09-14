use std::fs;
use std::path::Path;
use std::time::Instant;
use crate::utils::java::transpiler::JavaTranspiler;
use crate::utils::dir::dir_utils::{create_output_dir, check_input_dir};

pub struct FileProcessor {
    t: JavaTranspiler,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self { t: JavaTranspiler::new() }
    }

    pub fn process_directory(&self, p: &Path) {
        if !check_input_dir(p) { return; }
        create_output_dir("out");
        if let Ok(es) = fs::read_dir(p) {
            for e in es.flatten() {
                let q = e.path();
                if q.is_dir() {
                    self.process_directory(&q);
                } else if self.is_java(&q) {
                    self.proc_java(&q);
                }
            }
        }
    }

    fn is_java(&self, p: &Path) -> bool {
        p.extension().and_then(|s| s.to_str()) == Some("java")
    }

    fn proc_java(&self, p: &Path) {
        let name = p.file_stem().unwrap().to_str().unwrap();
        let out = format!("out/{}.rs", name);
        let start = Instant::now();
        if let Ok(src) = fs::read_to_string(p) {
            let rs = self.t.transpile_java_to_rust(&src, name);
            let dt = start.elapsed().as_secs_f64();
            if fs::write(&out, rs).is_ok() {
                println!("ок: {} -> {} [{:.3}s]", p.display(), out, dt);
            } else {
                println!("ошибка записи: {} -> {} [{:.3}s]", p.display(), out, dt);
            }
        } else {
            let dt = start.elapsed().as_secs_f64();
            println!("ошибка чтения: {} [{:.3}s]", p.display(), dt);
        }
    }
}
