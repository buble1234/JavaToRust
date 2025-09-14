use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct RunResult {
    pub ok: bool,
    pub time_s: f64,
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

pub struct Runner;

impl Runner {
    pub fn new() -> Self { Self }

    pub fn run(&self, bin: &Path, timeout_ms: u64) -> RunResult {
        let start = Instant::now();
        let mut child = match Command::new(bin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            Ok(c) => c,
            Err(e) => {
                return RunResult {
                    ok: false,
                    time_s: start.elapsed().as_secs_f64(),
                    code: -1,
                    stdout: String::new(),
                    stderr: format!("{}", e),
                    timed_out: false,
                }
            }
        };
        let timeout = Duration::from_millis(timeout_ms);
        let mut timed_out = false;
        loop {
            if let Ok(Some(_st)) = child.try_wait() { break; }
            if start.elapsed() >= timeout {
                let _ = child.kill();
                timed_out = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let dt = start.elapsed().as_secs_f64();
        let mut out_s = String::new();
        let mut err_s = String::new();
        if let Some(mut o) = child.stdout.take() {
            let _ = o.read_to_string(&mut out_s);
        }
        if let Some(mut e) = child.stderr.take() {
            let _ = e.read_to_string(&mut err_s);
        }
        let code = if timed_out { -1 } else { child.wait().map(|s| s.code().unwrap_or(-1)).unwrap_or(-1) };
        RunResult {
            ok: !timed_out && code == 0,
            time_s: dt,
            code,
            stdout: out_s,
            stderr: err_s,
            timed_out,
        }
    }
}
