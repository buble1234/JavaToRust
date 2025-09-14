use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_array_decl_stmt(&self, s: &str) -> String {
        let st = s.trim();

        let re_arr_decl = Regex::new(r"^(?:final\s+)?(?P<ty>\w+)\s*\[\]\s+(?P<name>\w+)\s*=\s*new\s+(?P<ty2>\w+)\s*\[\s*(?P<size>[^\]]+)\s*\]\s*;\s*$").unwrap();
        if let Some(c) = re_arr_decl.captures(st) {
            let jt = c.name("ty").unwrap().as_str();
            let name = c.name("name").unwrap().as_str();
            let size = c.name("size").unwrap().as_str().trim();

            let (rust_type, default_val) = match jt {
                "int" | "Integer" => ("i32", "0"),
                "long" | "Long" => ("i64", "0"),
                "short" => ("i16", "0"),
                "byte" => ("i8", "0"),
                "double" | "Double" => ("f64", "0.0"),
                "float" | "Float" => ("f32", "0.0"),
                "boolean" | "Boolean" => ("bool", "false"),
                "char" | "Character" => ("char", "'\\0'"),
                "String" => ("String", "String::new()"),
                _ => ("String", "String::new()"),
            };

            return format!("let mut {}: Vec<{}> = vec![{}; {}];", name, rust_type, default_val, size);
        }

        let re_obj_creation = Regex::new(r"^(?P<ty>\w+)\s+(?P<name>\w+)\s*=\s*new\s+(?P<ty2>\w+)\s*\(\s*\)\s*;\s*$").unwrap();
        if let Some(c) = re_obj_creation.captures(st) {
            let name = c.name("name").unwrap().as_str();
            let ty2 = c.name("ty2").unwrap().as_str();
            return format!("let mut {} = {}::new();", name, ty2);
        }

        st.to_string()
    }

    pub fn conv_new_array1d(&self, s: &str) -> String {
        let re = Regex::new(r#"new\s+(?P<ty>\w+)\s*\[\s*(?P<n>[^\]\s]+)\s*\]"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let ty = caps.name("ty").unwrap().as_str();
            let n  = caps.name("n").unwrap().as_str();
            let d = match ty {
                "int" | "Integer" => "0",
                "long" | "Long" => "0",
                "short" => "0",
                "byte" => "0",
                "double" | "Double" => "0.0",
                "float" | "Float" => "0.0",
                "boolean" | "Boolean" => "false",
                "char" | "Character" => "'\\0'",
                "String" => "String::new()",
                _ => "Default::default()",
            };
            format!("vec![{}; {}]", d, n)
        }).into_owned()
    }

    pub fn conv_new_array2d(&self, s: &str) -> String {
        let re2 = Regex::new(r#"new\s+(?P<ty>\w+)\s*\[\s*(?P<n1>[^\]\s]+)\s*\]\s*\[\s*(?P<n2>[^\]\s]+)\s*\]"#).unwrap();
        re2.replace_all(s, |caps: &regex::Captures| {
            let ty = caps.name("ty").unwrap().as_str();
            let n1 = caps.name("n1").unwrap().as_str();
            let n2 = caps.name("n2").unwrap().as_str();
            let d = match ty {
                "int" | "Integer" => "0",
                "long" | "Long" => "0",
                "short" => "0",
                "byte" => "0",
                "double" | "Double" => "0.0",
                "float" | "Float" => "0.0",
                "boolean" | "Boolean" => "false",
                "char" | "Character" => "'\\0'",
                "String" => "String::new()",
                _ => "Default::default()",
            };
            format!("vec![vec![{}; {}]; {}]", d, n2, n1)
        }).into_owned()
    }

    pub fn conv_array_literal_block(&self, s: &str) -> String {
        let re = Regex::new(r#"=\s*\{\s*([^\}]*)\s*\}\s*;?"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let inside = caps.get(1).unwrap().as_str().trim();
            let elems = inside.replace('\n', " ").replace("  ", " ");
            format!("= vec![{}];", elems)
        }).into_owned()
    }

    pub fn conv_index_cast_usize(&self, s: &str) -> String {
        let re = Regex::new(r#"\b([A-Za-z_][A-Za-z0-9_]*)\s*\[\s*([^\]\[]+?)\s*\]"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let arr = caps.get(1).unwrap().as_str();
            let idx = caps.get(2).unwrap().as_str().trim();
            if idx.contains("..") { format!("{}[{}]", arr, idx) } else { format!("{}[({}) as usize]", arr, idx) }
        }).into_owned()
    }

    pub fn conv_array_length(&self, s: &str) -> String {
        let re = Regex::new(r#"\b([A-Za-z_][A-Za-z0-9_]*)\.length\b"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let arr = caps.get(1).unwrap().as_str();
            format!("{}.len()", arr)
        }).into_owned()
    }
}
