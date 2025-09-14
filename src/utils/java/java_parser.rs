use regex::Regex;

pub struct JavaParser;

#[derive(Debug)]
pub enum JavaType { Class, Enum }

#[derive(Debug)]
pub struct JavaTypeInfo {
    pub name: String,
    pub type_info: JavaType,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<VariableInfo>,
    pub enum_variants: Vec<String>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub return_type: String,
    pub name: String,
    pub parameters: String,
    pub is_static: bool,
}

#[derive(Debug)]
pub struct VariableInfo {
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
    pub is_static: bool,
}

#[derive(Debug)]
pub struct ForLoopInfo {
    pub var_name: String,
    pub start: String,
    pub end: String,
}

impl JavaParser {
    pub fn new() -> Self { Self }

    pub fn should_skip_line(&self, s: &str) -> bool {
        s.is_empty() || s.starts_with("//") || s.starts_with("/*") || s.starts_with("*")
    }

    pub fn is_import_or_package(&self, s: &str) -> bool {
        s.starts_with("import ") || s.starts_with("package ")
    }

    pub fn extract_class_declaration(&self, s: &str) -> Option<(String, String)> {
        let re = Regex::new(r"(?:public\s+)?(class|enum)\s+(\w+)").unwrap();
        re.captures(s).map(|c| {
            let k = c.get(1).unwrap().as_str().to_string();
            let name = c.get(2).unwrap().as_str().to_string();
            (name, k)
        })
    }

    pub fn parse_method_declaration(&self, s: &str) -> Option<MethodInfo> {
        let re = Regex::new(
            r"^(?P<mods>(?:public|private|protected|static|final|abstract|synchronized|native|strictfp)\s+)*(?P<ret>[\w\[\]<>?,]+)\s+(?P<nm>\w+)\s*\((?P<ps>.*?)\)\s*\{?"
        ).unwrap();
        re.captures(s).map(|c| {
            let mods = c.name("mods").map(|m| m.as_str()).unwrap_or("");
            let is_static = mods.split_whitespace().any(|w| w == "static");
            MethodInfo {
                return_type: c.name("ret").unwrap().as_str().to_string(),
                name: c.name("nm").unwrap().as_str().to_string(),
                parameters: c.name("ps").unwrap().as_str().to_string(),
                is_static,
            }
        })
    }

    pub fn parse_variable_declaration(&self, s: &str) -> Option<VariableInfo> {
        if s.trim().starts_with("return ") { return None; }
        let re1 = Regex::new(r"^(?:public|private|protected)?\s*(?P<st>static\s+)?(?:final\s+)?(?P<ty>\w+(?:\[\])?)\s+(?P<nm>\w+)\s*=\s*(?P<val>.*?);\s*$").unwrap();
        if let Some(c) = re1.captures(s) {
            return Some(VariableInfo {
                var_type: c.name("ty").unwrap().as_str().to_string(),
                name: c.name("nm").unwrap().as_str().to_string(),
                value: Some(c.name("val").unwrap().as_str().to_string()),
                is_static: c.name("st").is_some(),
            });
        }
        let re2 = Regex::new(r"^(?:public|private|protected)?\s*(?P<st>static\s+)?(?:final\s+)?(?P<ty>\w+(?:\[\])?)\s+(?P<nm>\w+);\s*$").unwrap();
        re2.captures(s).map(|c| VariableInfo {
            var_type: c.name("ty").unwrap().as_str().to_string(),
            name: c.name("nm").unwrap().as_str().to_string(),
            value: None,
            is_static: c.name("st").is_some(),
        })
    }

    pub fn extract_enum_variants(&self, src: &str) -> Vec<String> {
        let mut variants = Vec::new();
        let header = Regex::new(r"(?:^|\s)(?:public\s+)?enum\s+\w+\s*\{").unwrap();
        if let Some(m) = header.find(src) {
            let mut depth: i32 = 1;
            let mut buf = String::new();
            for ch in src[m.end()..].chars() {
                match ch {
                    '{' => { depth += 1; buf.push(ch); }
                    '}' => {
                        depth -= 1;
                        if depth == 0 { break; }
                        buf.push(ch);
                    }
                    ';' if depth == 1 => { break; }
                    _ => buf.push(ch),
                }
            }
            let re = Regex::new(r"(?m)^\s*([A-Z][A-Z0-9_]*)\s*(?:[({,;])").unwrap();
            for cap in re.captures_iter(&buf) {
                if let Some(m) = cap.get(1) {
                    variants.push(m.as_str().to_string());
                }
            }
        }
        variants
    }

    pub fn class_body(&self, src: &str, class_name: &str) -> Option<String> {
        let re = Regex::new(&format!(r"(?:public\s+)?class\s+{}\s*\{{", regex::escape(class_name))).unwrap();
        let m = re.find(src)?;
        let mut depth: i32 = 1;
        let mut buf = String::new();
        for ch in src[m.end()..].chars() {
            match ch {
                '{' => { depth += 1; buf.push(ch); }
                '}' => {
                    depth -= 1;
                    if depth == 0 { break; }
                    buf.push(ch);
                }
                _ => buf.push(ch),
            }
        }
        Some(buf)
    }

    pub fn extract_inner_enums(&self, src: &str, class_name: &str) -> Vec<(String, Vec<String>)> {
        let mut res = Vec::new();
        if let Some(body) = self.class_body(src, class_name) {
            let mut i = 0usize;
            let b = body.as_bytes();
            while i < b.len() {
                if let Some(m) = body[i..].find("enum ") {
                    let j = i + m + "enum ".len();
                    let mut k = j;
                    while k < b.len() && (body.as_bytes()[k].is_ascii_alphanumeric() || body.as_bytes()[k] == b'_') { k += 1; }
                    let name = body[j..k].trim().to_string();
                    while k < b.len() && body.as_bytes()[k].is_ascii_whitespace() { k += 1; }
                    if k >= b.len() || body.as_bytes()[k] != b'{' { i = k; continue; }
                    k += 1;
                    let mut depth = 1i32;
                    let start = k;
                    while k < b.len() && depth > 0 {
                        match body.as_bytes()[k] {
                            b'{' => depth += 1,
                            b'}' => depth -= 1,
                            _ => {}
                        }
                        k += 1;
                    }
                    let block = &body[start..k-1];
                    let mut vars = Vec::new();
                    let re = Regex::new(r"(?m)^\s*([A-Z][A-Z0-9_]*)\s*(?:[({,;])").unwrap();
                    for cap in re.captures_iter(block) {
                        if let Some(mv) = cap.get(1) { vars.push(mv.as_str().to_string()); }
                    }
                    if !name.is_empty() && !vars.is_empty() { res.push((name, vars)); }
                    i = k;
                } else {
                    break;
                }
            }
        }
        res
    }

    pub fn parse_java_class(&self, src: &str) -> Option<JavaTypeInfo> {
        let mut n: Option<String> = None;
        let mut k: Option<JavaType> = None;
        let mut ms = Vec::new();
        let mut fs = Vec::new();
        let mut depth: i32 = 0;
        for l in src.lines() {
            let t = l.trim();
            if self.should_skip_line(t) || self.is_import_or_package(t) { continue; }
            if n.is_none() {
                if let Some((nm, kw)) = self.extract_class_declaration(t) {
                    n = Some(nm);
                    k = Some(match kw.as_str() { "enum" => JavaType::Enum, _ => JavaType::Class });
                }
            } else {
                let cur_depth = depth;
                if cur_depth == 1 {
                    if let Some(m) = self.parse_method_declaration(t) { ms.push(m); }
                    else if let Some(f) = self.parse_variable_declaration(t) { fs.push(f); }
                }
            }
            depth += t.chars().filter(|&c| c == '{').count() as i32;
            depth -= t.chars().filter(|&c| c == '}').count() as i32;
        }
        if let Some(name) = n {
            let mut ev = Vec::new();
            if let Some(JavaType::Enum) = k { ev = self.extract_enum_variants(src); }
            Some(JavaTypeInfo { name, type_info: k.unwrap_or(JavaType::Class), methods: ms, fields: fs, enum_variants: ev })
        } else { None }
    }
}
