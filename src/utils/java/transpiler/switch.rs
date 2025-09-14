use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_switch(&self, s: &str) -> String {
        let st = s.trim();

        let re_switch_open = Regex::new(r"^\s*switch\s*\(\s*(?P<expr>.+?)\s*\)\s*\{\s*$").unwrap();
        if let Some(c) = re_switch_open.captures(st) {
            let expr = c.name("expr").unwrap().as_str().trim();
            return format!("match {} {{", expr);
        }

        let re_case_single = Regex::new(r"^\s*case\s+(?P<val>[^:]+)\s*:\s*(?P<stmt>[^;]+)\s*;\s*break\s*;\s*$").unwrap();
        if let Some(c) = re_case_single.captures(st) {
            let val = c.name("val").unwrap().as_str().trim();
            let stmt = c.name("stmt").unwrap().as_str().trim();

            let rust_val = if Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap().is_match(val) {
                format!("Color::{}", val)
            } else {
                val.to_string()
            };

            let rust_stmt = if stmt.starts_with("System.out.println") {
                let content_re = Regex::new(r#"System\.out\.println\s*\(\s*"([^"]*)"\s*\)"#).unwrap();
                if let Some(m) = content_re.captures(stmt) {
                    let content = m.get(1).unwrap().as_str();
                    format!("println!(\"{}\");", content)
                } else {
                    stmt.to_string()
                }
            } else {
                stmt.to_string()
            };

            return format!("{} => {{ {} }},", rust_val, rust_stmt);
        }

        let re_case_block = Regex::new(r"^\s*case\s+(?P<val>[^:]+)\s*:\s*$").unwrap();
        if let Some(c) = re_case_block.captures(st) {
            let val = c.name("val").unwrap().as_str().trim();
            let rust_val = if Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap().is_match(val) {
                format!("Color::{}", val)
            } else {
                val.to_string()
            };
            return format!("{} => {{", rust_val);
        }

        let re_default_single = Regex::new(r"^\s*default\s*:\s*(?P<stmt>[^;]+)\s*;\s*break\s*;\s*$").unwrap();
        if let Some(c) = re_default_single.captures(st) {
            let stmt = c.name("stmt").unwrap().as_str().trim();
            let rust_stmt = if stmt.starts_with("System.out.println") {
                let content_re = Regex::new(r#"System\.out\.println\s*\(\s*"([^"]*)"\s*\)"#).unwrap();
                if let Some(m) = content_re.captures(stmt) {
                    let content = m.get(1).unwrap().as_str();
                    format!("println!(\"{}\");", content)
                } else {
                    stmt.to_string()
                }
            } else {
                stmt.to_string()
            };
            return format!("_ => {{ {} }},", rust_stmt);
        }

        let re_default_block = Regex::new(r"^\s*default\s*:\s*$").unwrap();
        if re_default_block.is_match(st) {
            return "_ => {".to_string();
        }

        let re_break = Regex::new(r"^\s*break\s*;\s*$").unwrap();
        if re_break.is_match(st) {
            return "},".to_string();
        }

        let re_close_brace = Regex::new(r"^\s*\}\s*$").unwrap();
        if re_close_brace.is_match(st) {
            return "}".to_string();
        }

        s.to_string()
    }
}
