use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    fn strip_lambda_params(&self, params: &str) -> String {
        let p = params.trim();
        if p.is_empty() { return "".into(); }
        p.split(',').map(|x| {
            let xs: Vec<&str> = x.trim().split_whitespace().collect();
            xs.last().unwrap_or(&"").to_string()
        }).collect::<Vec<String>>().join(", ")
    }

    fn map_java_type_to_rust(&self, t: &str) -> &'static str {
        match t.trim() {
            "int" | "Integer" => "i32",
            "long" | "Long" => "i64",
            "short" | "Short" => "i16",
            "byte" | "Byte" => "i8",
            "double" | "Double" => "f64",
            "float" | "Float" => "f32",
            "boolean" | "Boolean" => "bool",
            "char" | "Character" => "char",
            "String" => "String",
            _ => "String",
        }
    }

    pub fn conv_lambda(&self, s: &str) -> String {
        let mut x = s.to_string();

        let re_blk = Regex::new(r"(?P<ps>\([^\)]*\))\s*->\s*\{(?P<body>[^}]*)\}").unwrap();
        x = re_blk.replace_all(&x, |c: &regex::Captures| {
            let ps = c.name("ps").unwrap().as_str();
            let body = c.name("body").unwrap().as_str().trim();
            let params = self.strip_lambda_params(&ps[1..ps.len()-1]);
            format!("|{}| {{ {} }}", params, body)
        }).into_owned();

        let re_paren = Regex::new(r"(?P<ps>\([^\)]*\))\s*->\s*(?P<body>[^\);]+)").unwrap();
        x = re_paren.replace_all(&x, |c: &regex::Captures| {
            let ps = c.name("ps").unwrap().as_str();
            let body = c.name("body").unwrap().as_str().trim();
            let params = self.strip_lambda_params(&ps[1..ps.len()-1]);
            format!("|{}| {}", params, body)
        }).into_owned();

        let re_id_blk = Regex::new(r"(?P<p>\b\w+\b)\s*->\s*\{(?P<body>[^}]*)\}").unwrap();
        x = re_id_blk.replace_all(&x, |c: &regex::Captures| {
            let p = c.name("p").unwrap().as_str().trim();
            let body = c.name("body").unwrap().as_str().trim();
            format!("|{}| {{ {} }}", p, body)
        }).into_owned();

        let re_id = Regex::new(r"(?P<p>\b\w+\b)\s*->\s*(?P<body>[^\);]+)").unwrap();
        x = re_id.replace_all(&x, |c: &regex::Captures| {
            let p = c.name("p").unwrap().as_str().trim();
            let body = c.name("body").unwrap().as_str().trim();
            format!("|{}| {}", p, body)
        }).into_owned();

        let re_decl = Regex::new(
            r"(?m)^\s*java\.util\.function\.Function\s*<\s*(?P<tin>\w+)\s*,\s*(?P<tout>\w+)\s*>\s*(?P<nm>\w+)\s*=\s*(?P<lam>\|[^;]+);\s*$"
        ).unwrap();
        x = re_decl.replace_all(&x, |c: &regex::Captures| {
            let tin = self.map_java_type_to_rust(c.name("tin").unwrap().as_str());
            let nm = c.name("nm").unwrap().as_str();
            let lam = c.name("lam").unwrap().as_str();
            let parts: Vec<&str> = lam.splitn(2, '|').collect();
            let rest = parts.get(1).copied().unwrap_or("");
            let parts2: Vec<&str> = rest.splitn(2, '|').collect();
            let params = parts2.get(0).copied().unwrap_or("").trim().to_string();
            let body = parts2.get(1).copied().unwrap_or("").trim().to_string();
            let simple = !body.contains(';') && !body.contains('{') && !body.contains('}');
            if simple {
                format!("let mut {} = |{}: {}| {};", nm, params, tin, body)
            } else {
                format!("let mut {} = |{}: {}| {{ {} }};", nm, params, tin, body)
            }
        }).into_owned();

        let re_apply = Regex::new(r"(?P<nm>\b\w+\b)\.apply\s*\((?P<args>[^)]*)\)").unwrap();
        x = re_apply.replace_all(&x, "$nm($args)").into_owned();

        x
    }
}
