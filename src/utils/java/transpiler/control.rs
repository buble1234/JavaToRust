use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_if(&self, s: &str) -> String {
        let st = s.trim();
        let re_if = Regex::new(r"^\s*if\s*\((?P<expr>.*)\)\s*\{?\s*$").unwrap();
        if let Some(c) = re_if.captures(st) {
            let e = c.name("expr").map(|m| m.as_str()).unwrap_or("");
            return format!("if {} {{", e);
        }
        let re_ei = Regex::new(r"^\s*else\s+if\s*\((?P<expr>.*)\)\s*\{?\s*$").unwrap();
        if let Some(c) = re_ei.captures(st) {
            let e = c.name("expr").map(|m| m.as_str()).unwrap_or("");
            return format!("}} else if {} {{", e);
        }
        if st == "else {" || st == "else" {
            return "else {".into();
        }
        s.to_string()
    }

    pub fn conv_for(&self, s: &str) -> String {
        let re = Regex::new(
            r#"^\s*for\s*\(\s*int\s+(?P<v>\w+)\s*=\s*(?P<st>[^;]+?)\s*;\s*(?P<v2>\w+)\s*(?P<op><=|<)\s*(?P<ed>[^;]+?)\s*;\s*(?P<v3>\w+)\+\+\s*\)\s*\{?\s*$"#
        ).unwrap();
        if let Some(c) = re.captures(s) {
            let v  = c.name("v").unwrap().as_str();
            let v2 = c.name("v2").unwrap().as_str();
            let v3 = c.name("v3").unwrap().as_str();
            if v != v2 || v != v3 { return s.to_string(); }
            let st = c.name("st").unwrap().as_str().trim();
            let op = c.name("op").unwrap().as_str();
            let ed = c.name("ed").unwrap().as_str().trim();
            let rng = if op == "<=" { format!("{}..={}", st, ed) } else { format!("{}..{}", st, ed) };
            return format!("for {} in {} {{", v, rng);
        }
        s.to_string()
    }

    pub fn conv_while(&self, s: &str) -> String {
        let st = s.trim();
        let re = Regex::new(r"^\s*while\s*\((?P<expr>.*)\)\s*\{?\s*$").unwrap();
        if let Some(caps) = re.captures(st) {
            let e = caps.name("expr").map(|m| m.as_str()).unwrap_or("");
            return format!("while {} {{", e);
        }
        s.to_string()
    }

    pub fn conv_try(&self, s: &str) -> String {
        let st = s.trim();
        let re_try = Regex::new(r"^\s*try\s*\{\s*$").unwrap();
        if re_try.is_match(st) { return "{".into(); }
        let re_catch_same = Regex::new(r"^\s*\}\s*catch\s*\(\s*[\w\.\[\]<>?, ]+\s+\w+\s*\)\s*\{\s*$").unwrap();
        if re_catch_same.is_match(st) { return "}{".into(); }
        let re_catch = Regex::new(r"^\s*catch\s*\(\s*[\w\.\[\]<>?, ]+\s+\w+\s*\)\s*\{\s*$").unwrap();
        if re_catch.is_match(st) { return "{".into(); }
        let re_finally_same = Regex::new(r"^\s*\}\s*finally\s*\{\s*$").unwrap();
        if re_finally_same.is_match(st) { return "}{".into(); }
        let re_finally = Regex::new(r"^\s*finally\s*\{\s*$").unwrap();
        if re_finally.is_match(st) { return "{".into(); }
        s.to_string()
    }
}
