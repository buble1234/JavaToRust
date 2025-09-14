use regex::Regex;
use crate::utils::java::java_parser::VariableInfo;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn need_self(&self, b: &str, fs: &[VariableInfo]) -> bool {
        for f in fs {
            let pat = format!(r"\b{}\b|\bself\.{}\b", regex::escape(&f.name), regex::escape(&f.name));
            let re = Regex::new(&pat).unwrap();
            if re.is_match(b) { return true; }
        }
        false
    }

    pub fn repl_fields(&self, s: &str, fs: &[VariableInfo], in_static: bool) -> String {
        let mut r = s.to_string();
        for f in fs {
            let rg = Regex::new(&format!(r"\b{}\b", regex::escape(&f.name))).unwrap();
            if f.is_static {
                r = rg.replace_all(&r, format!("Self::{}", f.name)).to_string();
            } else if !in_static {
                r = rg.replace_all(&r, format!("self.{}", f.name)).to_string();
            }
        }
        r
    }
}
