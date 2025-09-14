use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    fn map_java_type_to_rust_fn(&self, t: &str) -> &'static str {
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

    pub fn conv_function_line(&self, s: &str) -> String {
        let mut x = s.to_string();

        let re_decl = Regex::new(
            r#"(?m)^\s*java\.util\.function\.Function\s*<\s*(?P<tin>\w+)\s*,\s*(?P<tout>\w+)\s*>\s*(?P<nm>\w+)\s*=\s*(?P<lam>\|[^;]+);\s*$"#
        ).unwrap();
        x = re_decl.replace_all(&x, |c: &regex::Captures| {
            let tin = self.map_java_type_to_rust_fn(c.name("tin").unwrap().as_str());
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

        let re_bidecl = Regex::new(
            r#"(?m)^\s*java\.util\.function\.BiFunction\s*<\s*(?P<t1>\w+)\s*,\s*(?P<t2>\w+)\s*,\s*(?P<tr>\w+)\s*>\s*(?P<nm>\w+)\s*=\s*(?P<lam>\|[^;]+);\s*$"#
        ).unwrap();
        x = re_bidecl.replace_all(&x, |c: &regex::Captures| {
            let t1 = self.map_java_type_to_rust_fn(c.name("t1").unwrap().as_str());
            let t2 = self.map_java_type_to_rust_fn(c.name("t2").unwrap().as_str());
            let nm = c.name("nm").unwrap().as_str();
            let lam = c.name("lam").unwrap().as_str();
            let parts: Vec<&str> = lam.splitn(2, '|').collect();
            let rest = parts.get(1).copied().unwrap_or("");
            let parts2: Vec<&str> = rest.splitn(2, '|').collect();
            let params = parts2.get(0).copied().unwrap_or("").trim().to_string();
            let body = parts2.get(1).copied().unwrap_or("").trim().to_string();
            let p1 = params.split(',').next().unwrap_or("x").trim();
            let p2 = params.split(',').nth(1).unwrap_or("y").trim();
            let simple = !body.contains(';') && !body.contains('{') && !body.contains('}');
            if simple {
                format!("let mut {} = |{}: {}, {}: {}| {};", nm, p1, t1, p2, t2, body)
            } else {
                format!("let mut {} = |{}: {}, {}: {}| {{ {} }};", nm, p1, t1, p2, t2, body)
            }
        }).into_owned();

        let re_apply2 = Regex::new(r#"(?P<nm>\b\w+\b)\.apply\s*\(\s*(?P<a>[^,]*)\s*,\s*(?P<b>[^)]*)\s*\)"#).unwrap();
        x = re_apply2.replace_all(&x, "$nm($a, $b)").into_owned();

        let re_apply1 = Regex::new(r#"(?P<nm>\b\w+\b)\.apply\s*\(\s*(?P<a>[^)]*)\s*\)"#).unwrap();
        x = re_apply1.replace_all(&x, "$nm($a)").into_owned();

        x
    }
}
