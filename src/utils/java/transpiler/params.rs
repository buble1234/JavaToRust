use regex::Regex;
use crate::utils::java::java_constants::*;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_params(&self, s: &str) -> String {
        if s.trim().is_empty() { return "".into(); }
        s.split(',').map(|p| {
            let xs: Vec<&str> = p.trim().split_whitespace().collect();
            if xs.len() >= 2 {
                let jt = xs[xs.len()-2];
                let nm = xs[xs.len()-1];
                if jt == JAVA_STRING_ARRAY && nm == "args" { format!("{}: {}", nm, RUST_STR_SLICE_ARRAY) }
                else { format!("{}: {}", nm, self.t.convert_type(jt)) }
            } else { p.to_string() }
        }).collect::<Vec<String>>().join(", ")
    }

    pub fn body(&self, src: &str, name: &str) -> String {
        let mut b = String::new();
        let mut on = false;
        let mut d = 0i32;
        let re = Regex::new(&format!(r"^\s*(?:public|private|protected)?\s*(?:static)?\s*\w+(?:\s+)?{}\s*\(.*\)\s*\{{", regex::escape(name))).unwrap();
        for l in src.lines() {
            let t = l.trim();
            if !on {
                if re.is_match(t) { on = true; d = 1; continue; }
                continue;
            }
            for c in t.chars() {
                match c {
                    '{' => d += 1,
                    '}' => { d -= 1; if d == 0 { return b; } }
                    _ => {}
                }
            }
            b.push_str(l);
            b.push('\n');
        }
        b
    }

    pub fn conv_return(&self, s: &str, rt: &str) -> String {
        if s.starts_with("return ") {
            let mut v = s.replace("return ", "");
            if v.ends_with(';') { v.pop(); }
            if rt == "String" { return format!("return {}.to_string();", v); }
            return format!("return {};", v);
        }
        s.to_string()
    }
}
