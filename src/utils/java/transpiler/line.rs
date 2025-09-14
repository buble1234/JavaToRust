use crate::utils::java::java_parser::VariableInfo;
use crate::utils::java::transpiler::JavaTranspiler;
use crate::utils::java::transpiler::{print::*, fields::*, control::*, switch::*, lambda::*};
use regex::Regex;

impl JavaTranspiler {
    pub fn conv_line(&self, s: &str, fs: &[VariableInfo], rt: &str, in_static: bool) -> String {
        self.conv_line_with_all(s, fs, rt, in_static, "", &[], &[])
    }

    pub fn conv_line_with_all(&self, s: &str, fs: &[VariableInfo], rt: &str, in_static: bool, class: &str, statics: &[String], varargs: &[String]) -> String {
        let st = s.trim();

        let mut x = self.conv_lambda(st);
        x = self.conv_function_line(&x);
        x = self.conv_multi_decl_inline(&x);
        x = self.conv_string_assign(&x);
        x = self.conv_call_str_args(&x);
        x = self.conv_array_decl_stmt(&x);

        x = self.conv_foreach(&x);
        x = self.conv_do_start(&x);
        x = self.conv_do_end_while(&x);
        x = self.conv_ternary(&x);

        x = self.conv_array_literal_block(&x);
        x = self.conv_new_array2d(&x);
        x = self.conv_new_array1d(&x);
        x = self.conv_array_length(&x);

        let y = self.concat_fmt_ex(&x, fs, in_static, class, statics);
        if y != x { return y; }

        if let Some(cap) = Regex::new(r#"^\s*(?P<nm>\w+)\s*=\s*(?P<lit>-?\d+(?:\.\d+)?|"[^"]*")\s*;\s*$"#).unwrap().captures(&x) {
            let lit = cap.name("lit").unwrap().as_str();
            return format!("let _ = {};", lit);
        }

        if x.starts_with("return ") && rt == "String" && x.contains(" + ") {
            let mut expr = x.strip_prefix("return ").unwrap().trim().to_string();
            if expr.ends_with(';') { expr.pop(); }
            let (fmt, args) = self.mk_fmt(&expr, fs, in_static);
            return if args.is_empty() { format!("return \"{}\".to_string();", fmt) } else { format!("return format!(\"{}\", {});", fmt, args.join(", ")) };
        }

        let ty_pat = r"(?:int|long|short|byte|double|float|boolean|char|String|Integer|Long|Double|Float|Boolean|Character|Object)(?:\[\])?";
        if let Some(c) = Regex::new(&format!(r#"^(?:final\s+)?(?P<ty>{})\s+(?P<rest>[^;]+)\s*;\s*$"#, ty_pat)).unwrap().captures(&x) {
            let jt = c.name("ty").unwrap().as_str();
            let rest = c.name("rest").unwrap().as_str();
            let rty = self.t.convert_type(jt);

            if rest.contains(',') {
                let mut lines: Vec<String> = Vec::new();
                for part in rest.split(',') {
                    let p = part.trim();
                    if p.is_empty() { continue; }
                    if let Some(eq) = p.find('=') {
                        let name = p[..eq].trim();
                        let mut expr = p[eq+1..].trim().to_string();
                        expr = self.m.convert_math_expression(&expr);
                        expr = self.conv_new_array1d(&expr);
                        expr = self.conv_new_array2d(&expr);
                        expr = self.conv_array_literal_block(&expr);
                        if rty == "String" {
                            let qt = expr.trim();
                            if qt.starts_with('"') && qt.ends_with('"') { expr = format!("{}.to_string()", qt); }
                        }
                        lines.push(format!("let mut {}: {} = {};", name, rty, expr));
                    } else {
                        lines.push(format!("let mut {}: {};", p, rty));
                    }
                }
                return lines.join("\n");
            } else {
                if let Some(c2) = Regex::new(&format!(r#"^(?:final\s+)?(?P<ty>{})\s+(?P<nm>\w+)\s*=\s*(?P<expr>.*)$"#, ty_pat)).unwrap().captures(&x) {
                    let jt2 = c2.name("ty").unwrap().as_str();
                    let name = c2.name("nm").unwrap().as_str();
                    let mut expr = c2.name("expr").unwrap().as_str().trim_end_matches(';').trim().to_string();
                    expr = self.m.convert_math_expression(&expr);
                    expr = self.conv_new_array1d(&expr);
                    expr = self.conv_new_array2d(&expr);
                    expr = self.conv_array_literal_block(&expr);
                    let rty2 = self.t.convert_type(jt2);
                    if rty2 == "String" {
                        let qt = expr.trim();
                        if qt.starts_with('"') && qt.ends_with('"') { expr = format!("{}.to_string()", qt); }
                    }
                    return format!("let mut {}: {} = {};", name, rty2, expr);
                }
                if let Some(c3) = Regex::new(&format!(r#"^(?:final\s+)?(?P<ty>{})\s+(?P<nm>\w+)\s*$"#, ty_pat)).unwrap().captures(&x) {
                    let jt3 = c3.name("ty").unwrap().as_str();
                    let name = c3.name("nm").unwrap().as_str();
                    return format!("let mut {}: {};", name, self.t.convert_type(jt3));
                }
            }
        }

        let mut x2 = x.clone();
        x2 = self.m.convert_math_expression(&x2);
        x2 = self.conv_new_array1d(&x2);
        x2 = self.conv_new_array2d(&x2);
        x2 = self.conv_array_literal_block(&x2);
        x2 = self.repl_fields(&x2, fs, in_static);
        x2 = self.conv_enum_access(&x2);
        x2 = self.conv_index_cast_usize(&x2);

        x2 = self.conv_if(&x2);
        x2 = self.conv_for(&x2);
        x2 = self.conv_while(&x2);
        x2 = self.conv_try(&x2);
        x2 = self.conv_switch(&x2);

        x2 = self.conv_varargs_calls(&x2, class, varargs);
        x2 = self.conv_static_calls(&x2, class, statics);
        x2 = self.conv_return(&x2, rt);
        x2
    }

    pub fn conv_enum_access(&self, s: &str) -> String {
        let re = Regex::new(r#"\b([A-Z][A-Za-z0-9_]*)\.([A-Z][A-Z0-9_]*)\b"#).unwrap();
        re.replace_all(s, "$1::$2").into_owned()
    }

    pub fn conv_string_assign(&self, s: &str) -> String {
        let re = Regex::new(r#"^\s*(?P<lhs>[^=;]+?)\s=\s(?P<q>"[^"]*")\s*;\s*$"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let lhs = caps.name("lhs").unwrap().as_str().trim();
            let q = caps.name("q").unwrap().as_str();
            format!("{} = {}.to_string();", lhs, q)
        }).into_owned()
    }

    pub fn conv_call_str_args(&self, s: &str) -> String {
        let t = s.trim_start();
        if t.starts_with("println!") || t.starts_with("eprintln!") || t.starts_with("print!") || t.starts_with("eprint!") || t.starts_with("format!") { return s.to_string(); }
        let re = Regex::new(r#""(?P<q>[^"]*)"\s*(?P<d>[,\)\}])"#).unwrap();
        re.replace_all(s, |caps: &regex::Captures| {
            let q = caps.name("q").unwrap().as_str();
            let d = caps.name("d").unwrap().as_str();
            format!("\"{}\".to_string(){}", q, d)
        }).into_owned()
    }

    pub fn conv_static_calls(&self, s: &str, class: &str, statics: &[String]) -> String {
        if class.is_empty() || statics.is_empty() { return s.to_string(); }
        let mut out = s.to_string();
        for name in statics {
            if name == "main" { continue; }
            let re1 = Regex::new(&format!(r#"\b(\w+)\s*\.\s*{}\s*\("#, regex::escape(name))).unwrap();
            out = re1.replace_all(&out, |caps: &regex::Captures| {
                let recv = caps.get(1).unwrap().as_str();
                if recv == "self" { caps.get(0).unwrap().as_str().to_string() } else { format!("{}::{}(", class, name) }
            }).into_owned();
            let re2 = Regex::new(&format!(r#"(?m)(^|[^:\.\w]){}\s*\("#, regex::escape(name))).unwrap();
            out = re2.replace_all(&out, |caps: &regex::Captures| {
                let pre = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                format!("{}Self::{}(", pre, name)
            }).into_owned();
        }
        out
    }
}
