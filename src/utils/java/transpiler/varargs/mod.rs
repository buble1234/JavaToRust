use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_params_varargs(&self, params: &str) -> (String, Vec<String>) {
        let mut names = Vec::new();
        if params.trim().is_empty() { return (String::new(), names); }
        let mut out = Vec::new();
        for p in params.split(',') {
            let t = p.trim();
            if t.is_empty() { continue; }
            if let Some(c) = Regex::new(r#"^(?P<ty>\w+)\s*\.\.\.\s*(?P<nm>\w+)$"#).unwrap().captures(t) {
                let jt = c.name("ty").unwrap().as_str();
                let nm = c.name("nm").unwrap().as_str();
                let rt = self.t.convert_type(jt);
                out.push(format!("{}: &[{}]", nm, rt));
                names.push(nm.to_string());
            } else if let Some(c) = Regex::new(r#"^(?P<ty>\w+)\s+(?P<nm>\w+)$"#).unwrap().captures(t) {
                let jt = c.name("ty").unwrap().as_str();
                let nm = c.name("nm").unwrap().as_str();
                out.push(format!("{}: {}", nm, self.t.convert_type(jt)));
            } else {
                out.push(t.to_string());
            }
        }
        (out.join(", "), names)
    }

    pub fn conv_varargs_calls(&self, s: &str, class: &str, varargs: &[String]) -> String {
        if varargs.is_empty() { return s.to_string(); }
        let mut out = s.to_string();
        for name in varargs {
            let re_bare = Regex::new(&format!(r#"(?m)(^|[^:\.\w]){}\s*\((?P<args>[^)]*)\)"#, regex::escape(name))).unwrap();
            out = re_bare.replace_all(&out, |caps: &regex::Captures| {
                let pre = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let args = caps.name("args").map(|m| m.as_str().trim()).unwrap_or("");
                let wrap = if args.is_empty() { "&[]".to_string() } else { format!("&[{}]", args) };
                format!("{}Self::{}({})", pre, name, wrap)
            }).into_owned();

            let re_class = Regex::new(&format!(r#"\b([A-Z]\w*)\s*\.\s*{}\s*\((?P<args>[^)]*)\)"#, regex::escape(name))).unwrap();
            out = re_class.replace_all(&out, |caps: &regex::Captures| {
                let cls = caps.get(1).unwrap().as_str();
                let args = caps.name("args").map(|m| m.as_str().trim()).unwrap_or("");
                let wrap = if args.is_empty() { "&[]".to_string() } else { format!("&[{}]", args) };
                format!("{}::{}({})", cls, name, wrap)
            }).into_owned();

            let re_inst = Regex::new(&format!(r#"\b(\w+)\s*\.\s*{}\s*\((?P<args>[^)]*)\)"#, regex::escape(name))).unwrap();
            out = re_inst.replace_all(&out, |caps: &regex::Captures| {
                let recv = caps.get(1).unwrap().as_str();
                let args = caps.name("args").map(|m| m.as_str().trim()).unwrap_or("");
                let wrap = if args.is_empty() { "&[]".to_string() } else { format!("&[{}]", args) };
                format!("{}.{}({})", recv, name, wrap)
            }).into_owned();
        }
        out
    }

    pub fn conv_varargs_in_expr(&self, s: &str, class: &str, varargs: &[String]) -> String {
        self.conv_varargs_calls(s, class, varargs)
    }
}
