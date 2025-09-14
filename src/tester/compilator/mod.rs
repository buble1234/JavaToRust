use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

pub struct CompileResult {
    pub ok: bool,
    pub bin_path: PathBuf,
    pub time_s: f64,
    pub stdout: String,
    pub stderr: String,
}

pub struct Compilator;

impl Compilator {
    pub fn new() -> Self { Self }

    pub fn ensure_dirs() {
        let _ = fs::create_dir_all("target/tester/bin");
        let _ = fs::create_dir_all("target/tester/tmp");
    }

    fn has_main(s: &str) -> bool {
        s.lines().any(|l| l.trim_start().starts_with("pub fn main("))
    }

    fn detect_type_name(src: &str, fallback: &str) -> String {
        for line in src.lines() {
            let t = line.trim();
            if t.starts_with("pub struct ") {
                let rest = &t["pub struct ".len()..];
                let name = rest.split(|c: char| c.is_whitespace() || c == '{').next().unwrap_or("");
                if !name.is_empty() { return name.to_string(); }
            }
            if t.starts_with("struct ") {
                let rest = &t["struct ".len()..];
                let name = rest.split(|c: char| c.is_whitespace() || c == '{').next().unwrap_or("");
                if !name.is_empty() { return name.to_string(); }
            }
        }
        fallback.to_string()
    }

    fn write_single_harness(target: &Path, type_name: &str, call_main: bool) -> PathBuf {
        let stem = target.file_stem().unwrap().to_str().unwrap();
        let harness = PathBuf::from(format!("target/tester/tmp/{}_single.rs", stem));
        let abs = fs::canonicalize(target).unwrap_or_else(|_| target.to_path_buf());
        let mut code = String::new();
        code.push_str("#![allow(unconditional_panic)]\n");
        code.push_str(&format!("include!(r#\"{}\"#);\n", abs.display()));
        if call_main {
            code.push_str(&format!("fn main() {{ {}::main(&[]); }}\n", type_name));
        } else {
            code.push_str("fn main() {}\n");
        }
        let _ = fs::write(&harness, code);
        harness
    }

    pub fn compile(&self, out_file: &Path) -> CompileResult {
        Self::ensure_dirs();
        let start = Instant::now();
        let src_text = fs::read_to_string(out_file).unwrap_or_default();
        let type_name = Self::detect_type_name(&src_text, &out_file.file_stem().unwrap().to_string_lossy());
        let call_main = Self::has_main(&src_text);
        let harness = Self::write_single_harness(out_file, &type_name, call_main);
        let bin = PathBuf::from(format!("target/tester/bin/{}", out_file.file_stem().unwrap().to_string_lossy()));
        let output = Command::new("rustc")
            .args(&["--edition", "2021", "--error-format", "short"])
            .arg(&harness)
            .arg("-o").arg(&bin)
            .output();
        let dt = start.elapsed().as_secs_f64();
        match output {
            Ok(o) => CompileResult {
                ok: o.status.success(),
                bin_path: bin,
                time_s: dt,
                stdout: String::from_utf8_lossy(&o.stdout).to_string(),
                stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            },
            Err(e) => CompileResult {
                ok: false,
                bin_path: bin,
                time_s: dt,
                stdout: String::new(),
                stderr: format!("{}", e),
            }
        }
    }
}
