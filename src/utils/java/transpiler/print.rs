use regex::Regex;
use crate::utils::java::java_parser::VariableInfo;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn concat_fmt(&self, s: &str, fs: &[VariableInfo], in_static: bool) -> String {
        let re = Regex::new(r#"System\.(out|err)\.(println|print)\((.*)\);\s*"#).unwrap();
        if let Some(c) = re.captures(s) {
            let stream = c.get(1).unwrap().as_str();
            let m = c.get(2).unwrap().as_str();
            let content = c.get(3).unwrap().as_str();
            let (fmt, args) = self.mk_fmt(content, fs, in_static);
            let mac = match (stream, m) {
                ("out", "println") => "println",
                ("out", "print") => "print",
                ("err", "println") => "eprintln",
                ("err", "print") => "eprint",
                _ => "println",
            };
            return if args.is_empty() { format!("{}!(\"{}\");", mac, fmt) } else { format!("{}!(\"{}\", {});", mac, fmt, args.join(", ")) };
        }
        s.to_string()
    }

    pub fn concat_fmt_ex(&self, s: &str, fs: &[VariableInfo], in_static: bool, class: &str, statics: &[String]) -> String {
        let re = Regex::new(r#"System\.(out|err)\.(println|print)\((.*)\);\s*"#).unwrap();
        if let Some(c) = re.captures(s) {
            let stream = c.get(1).unwrap().as_str();
            let m = c.get(2).unwrap().as_str();
            let content = c.get(3).unwrap().as_str();
            let (fmt, args) = self.mk_fmt_ex(content, fs, in_static, class, statics);
            let mac = match (stream, m) {
                ("out", "println") => "println",
                ("out", "print") => "print",
                ("err", "println") => "eprintln",
                ("err", "print") => "eprint",
                _ => "println",
            };
            return if args.is_empty() { format!("{}!(\"{}\");", mac, fmt) } else { format!("{}!(\"{}\", {});", mac, fmt, args.join(", ")) };
        }
        s.to_string()
    }

    pub fn mk_fmt(&self, s: &str, fs: &[VariableInfo], in_static: bool) -> (String, Vec<String>) {
        let mut parts = Vec::new();
        let mut args = Vec::new();
        let re_enum = Regex::new(r#"\b([A-Z][A-Za-z0-9_]*)::([A-Z][A-Z0-9_]*)\b"#).unwrap();
        for p in s.split(" + ") {
            let q0 = p.trim();
            if q0.starts_with('"') && q0.ends_with('"') && q0.len() >= 2 {
                parts.push(q0[1..q0.len()-1].to_string());
            } else {
                let mut q = self.m.convert_math_expression(q0);
                q = self.conv_enum_access(&q);
                q = self.conv_index_cast_usize(&q);
                let ph = if re_enum.is_match(&q) { "{:?}" } else { "{}" };
                parts.push(ph.into());
                if q.starts_with("self.") {
                    args.push(q.to_string());
                } else if let Some(f) = fs.iter().find(|f| f.name == q) {
                    if f.is_static { args.push(format!("Self::{}", q)); }
                    else if !in_static { args.push(format!("self.{}", q)); }
                    else { args.push(q.to_string()); }
                } else {
                    args.push(q.to_string());
                }
            }
        }
        (parts.join(""), args)
    }

    pub fn mk_fmt_ex(&self, s: &str, fs: &[VariableInfo], in_static: bool, class: &str, statics: &[String]) -> (String, Vec<String>) {
        let mut parts = Vec::new();
        let mut args = Vec::new();
        let re_enum = Regex::new(r#"\b([A-Z][A-Za-z0-9_]*)::([A-Z][A-Z0-9_]*)\b"#).unwrap();
        let re_str_in_call = Regex::new(r#""[^"]*""#).unwrap();
        for p in s.split(" + ") {
            let q0 = p.trim();
            if q0.starts_with('"') && q0.ends_with('"') && q0.len() >= 2 {
                parts.push(q0[1..q0.len()-1].to_string());
            } else {
                let mut q = self.m.convert_math_expression(q0);
                q = self.conv_enum_access(&q);
                q = self.conv_index_cast_usize(&q);
                if !class.is_empty() && !statics.is_empty() {
                    q = self.conv_static_in_expr(&q, class, statics);
                }
                if q.contains('(') && q.contains(')') && q.contains('"') {
                    q = re_str_in_call.replace_all(&q, |m: &regex::Captures| {
                        let lit = m.get(0).unwrap().as_str();
                        format!("{}.to_string()", lit)
                    }).into_owned();
                }
                let ph = if re_enum.is_match(&q) { "{:?}" } else { "{}" };
                parts.push(ph.into());
                if q.starts_with("self.") {
                    args.push(q.to_string());
                } else if let Some(f) = fs.iter().find(|f| f.name == q) {
                    if f.is_static { args.push(format!("Self::{}", q)); }
                    else if !in_static { args.push(format!("self.{}", q)); }
                    else { args.push(q.to_string()); }
                } else {
                    args.push(q.to_string());
                }
            }
        }
        (parts.join(""), args)
    }

    pub fn conv_static_in_expr(&self, s: &str, class: &str, statics: &[String]) -> String {
        let mut out = s.to_string();
        for name in statics {
            if name == "main" { continue; }
            let re = Regex::new(&format!(r#"(?m)(^|[^:\.\w]){}\s*\("#, regex::escape(name))).unwrap();
            out = re.replace_all(&out, |caps: &regex::Captures| {
                let pre = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                format!("{}Self::{}(", pre, name)
            }).into_owned();
        }
        out
    }
}
