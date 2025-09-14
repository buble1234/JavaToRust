use regex::Regex;
use crate::utils::java::transpiler::JavaTranspiler;

impl JavaTranspiler {
    pub fn conv_ternary(&self, s: &str) -> String {
        let re = Regex::new(r#"\(\s*(?P<c>[^?]+?)\s*\)\s*\?\s*(?P<t>[^:;]+?)\s*:\s*(?P<f>[^;]+)"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let c = caps.name("c").unwrap().as_str();
            let t = caps.name("t").unwrap().as_str();
            let f = caps.name("f").unwrap().as_str();
            format!("if {} {{ {} }} else {{ {} }}", c, t, f)
        }).into_owned()
    }
}
