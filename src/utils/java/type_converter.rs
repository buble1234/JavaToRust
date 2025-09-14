use std::collections::HashMap;

pub struct TypeConverter {
    m: HashMap<String, String>,
    g: HashMap<String, String>,
}

impl TypeConverter {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        let mut g = HashMap::new();
        m.insert("void".to_string(), "()".to_string());
        m.insert("int".to_string(), "i32".to_string());
        m.insert("long".to_string(), "i64".to_string());
        m.insert("short".to_string(), "i16".to_string());
        m.insert("byte".to_string(), "i8".to_string());
        m.insert("double".to_string(), "f64".to_string());
        m.insert("float".to_string(), "f32".to_string());
        m.insert("boolean".to_string(), "bool".to_string());
        m.insert("char".to_string(), "char".to_string());
        m.insert("String".to_string(), "String".to_string());
        m.insert("Integer".to_string(), "i32".to_string());
        m.insert("Long".to_string(), "i64".to_string());
        m.insert("Double".to_string(), "f64".to_string());
        m.insert("Float".to_string(), "f32".to_string());
        m.insert("Boolean".to_string(), "bool".to_string());
        m.insert("Character".to_string(), "char".to_string());
        m.insert("Object".to_string(), "Box<dyn std::any::Any>".to_string());
        g.insert("List".to_string(), "Vec".to_string());
        g.insert("ArrayList".to_string(), "Vec".to_string());
        g.insert("LinkedList".to_string(), "std::collections::VecDeque".to_string());
        g.insert("HashMap".to_string(), "std::collections::HashMap".to_string());
        g.insert("HashSet".to_string(), "std::collections::HashSet".to_string());
        g.insert("Optional".to_string(), "Option".to_string());
        Self { m, g }
    }

    pub fn convert_type(&self, t: &str) -> String {
        let s = t.trim();
        if s.ends_with("[]") {
            let b = &s[..s.len()-2];
            return format!("Vec<{}>", self.convert_type(b));
        }
        if let (Some(i), Some(j)) = (s.find('<'), s.rfind('>')) {
            let base = &s[..i];
            let inner = &s[i+1..j];
            if let Some(rb) = self.g.get(base) {
                let xs: Vec<String> = inner.split(',').map(|x| self.convert_type(x.trim())).collect();
                return format!("{}<{}>", rb, xs.join(", "));
            }
        }
        self.m.get(s).cloned().unwrap_or_else(|| s.to_string())
    }

    pub fn def(&self, r: &str) -> String {
        match r {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => "0".into(),
            "f32" | "f64" => "0.0".into(),
            "bool" => "false".into(),
            "char" => "'\\0'".into(),
            "String" => "String::new()".into(),
            s if s.starts_with("Vec<") => "Vec::new()".into(),
            s if s.starts_with("std::collections::HashMap<") => "std::collections::HashMap::new()".into(),
            s if s.starts_with("Option<") => "None".into(),
            _ => "Default::default()".into(),
        }
    }
}
