use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn split_multi_decl(&self, s: &str) -> Option<Vec<String>> {
        let re = Regex::new(r#"^\s*let\s+mut\s+(?P<a>\w+)\s*:\s*(?P<ty>[^=,;]+)\s*=\s*(?P<va>[^,;]+)\s*,\s*(?P<b>\w+)\s*=\s*(?P<vb>[^;]+)\s*;\s*$"#).unwrap();
        if let Some(c) = re.captures(s) {
            let a = c.name("a").unwrap().as_str();
            let ty = c.name("ty").unwrap().as_str().trim();
            let va = c.name("va").unwrap().as_str().trim();
            let b = c.name("b").unwrap().as_str();
            let vb = c.name("vb").unwrap().as_str().trim();
            return Some(vec![
                format!("let mut {}: {} = {};", a, ty, va),
                format!("let mut {} = {};", b, vb),
            ]);
        }
        None
    }

    pub fn conv_multi_decl_inline(&self, s: &str) -> String {
        if let Some(v) = self.split_multi_decl(s) {
            return v.join("\n");
        }
        s.to_string()
    }
}
