use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_foreach(&self, s: &str) -> String {
        let re_one = Regex::new(
            r#"^\s*for\s*\(\s*(?:int|Integer|long|Long|double|Double|float|Float|boolean|Boolean|char|Character|String)\s+(?P<v>\w+)\s*:\s*(?P<arr>[A-Za-z_][A-Za-z0-9_]*)\s*\)\s*(?P<body>[^;{]+)\s*;\s*$"#
        ).unwrap();
        if let Some(c) = re_one.captures(s) {
            let v = c.name("v").unwrap().as_str();
            let arr = c.name("arr").unwrap().as_str();
            let body = c.name("body").unwrap().as_str().trim();
            let body_rust = self.conv_function_line(body);
            return format!("for &{} in {}.iter() {{ {} ; }}", v, arr, body_rust);
        }

        let re_block = Regex::new(
            r#"^\s*for\s*\(\s*(?:int|Integer|long|Long|double|Double|float|Float|boolean|Boolean|char|Character|String)\s+(?P<v>\w+)\s*:\s*(?P<arr>[A-Za-z_][A-Za-z0-9_]*)\s*\)\s*\{?\s*$"#
        ).unwrap();
        if let Some(c) = re_block.captures(s) {
            let v = c.name("v").unwrap().as_str();
            let arr = c.name("arr").unwrap().as_str();
            return format!("for &{} in {}.iter() {{", v, arr);
        }

        s.to_string()
    }

    pub fn conv_do_start(&self, s: &str) -> String {
        let re = Regex::new(r#"^\s*do\s*\{\s*$"#).unwrap();
        if re.is_match(s) { "loop {".to_string() } else { s.to_string() }
    }

    pub fn conv_do_end_while(&self, s: &str) -> String {
        let re = Regex::new(r#"^\s*while\s*\(\s*(?P<cond>.+?)\s*\)\s*;\s*$"#).unwrap();
        if let Some(c) = re.captures(s) {
            let cond = c.name("cond").unwrap().as_str();
            return format!("if !({}) {{ break; }}", cond);
        }
        s.to_string()
    }
}
