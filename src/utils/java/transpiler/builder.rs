use regex::Regex;
use crate::utils::java::java_constants::*;
use crate::utils::java::java_parser::{JavaType, VariableInfo};
use crate::utils::java::transpiler::JavaTranspiler;
use crate::utils::java::transpiler::{params::*, fields::*, line::*, format::*};

impl JavaTranspiler {
    pub fn transpile_java_to_rust(&self, src: &str, _f: &str) -> String {
        let mut out = String::new();
        if let Some(ci) = self.p.parse_java_class(src) {
            match ci.type_info {
                JavaType::Class => {
                    out.push_str(&format!("pub struct {} {{\n", ci.name));
                    for f in &ci.fields {
                        if !f.is_static {
                            out.push_str(&format!("    pub {}: {},\n", f.name, self.t.convert_type(&f.var_type)));
                        }
                    }
                    out.push_str("}\n\n");

                    let inners = self.p.extract_inner_enums(src, &ci.name);
                    for (en, vs) in inners {
                        out.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum {} {{\n", en));
                        for v in vs { out.push_str(&format!("    {},\n", v)); }
                        out.push_str("}\n\n");
                    }

                    out.push_str(&format!("impl {} {{\n", ci.name));

                    for f in &ci.fields {
                        if f.is_static {
                            if let Some(v) = &f.value {
                                let ty = self.t.convert_type(&f.var_type);
                                let vv = v.trim();
                                let vv2 = self.m.convert_math_expression(vv);
                                if ty == "String" && vv2.starts_with('"') && vv2.ends_with('"') {
                                    out.push_str(&format!("    pub const {}: &'static str = {};\n", f.name, vv2));
                                } else {
                                    out.push_str(&format!("    pub const {}: {} = {};\n", f.name, ty, vv2));
                                }
                            }
                        }
                    }

                    if ci.fields.iter().any(|f| !f.is_static) {
                        out.push_str("    pub fn new() -> Self {\n        Self {\n");
                        for f in &ci.fields {
                            if !f.is_static && !f.var_type.is_empty() {
                                let rt = self.t.convert_type(&f.var_type);
                                out.push_str(&format!("            {}: {},\n", f.name, self.t.def(&rt)));
                            }
                        }
                        out.push_str("        }\n    }\n");
                    }

                    let static_names: Vec<String> = ci.methods.iter().filter(|mm| mm.is_static || mm.name == "main").map(|mm| mm.name.clone()).collect();
                    let vararg_names: Vec<String> = ci.methods.iter().filter_map(|mm| if mm.parameters.contains("...") { Some(mm.name.clone()) } else { None }).collect();

                    for m in &ci.methods {
                        if m.name == ci.name { continue; }
                        let (ps_rust, _va) = if m.parameters.contains("...") { self.conv_params_varargs(&m.parameters) } else { (self.conv_params(&m.parameters), Vec::new()) };
                        let rt = self.t.convert_type(&m.return_type);
                        let body = self.body(src, &m.name);
                        let is_static = m.is_static || m.name == "main";

                        let head = if is_static {
                            let pr = ps_rust.clone();
                            if rt == RUST_UNIT { format!("    pub fn {}({})", m.name, pr) } else { format!("    pub fn {}({}) -> {}", m.name, pr, rt) }
                        } else {
                            let mut pr = String::from("&mut self");
                            if !ps_rust.is_empty() { pr.push_str(", "); pr.push_str(&ps_rust); }
                            if rt == RUST_UNIT { format!("    pub fn {}({})", m.name, pr) } else { format!("    pub fn {}({}) -> {}", m.name, pr, rt) }
                        };

                        out.push_str(&format!("{} {{\n", head));
                        for ln in body.lines() {
                            let mut s = ln.trim().to_string();
                            if s.is_empty() { continue; }
                            s = self.conv_line_with_all(&s, &ci.fields, &rt, is_static, &ci.name, &static_names, &vararg_names);
                            out.push_str("        ");
                            out.push_str(&s);
                            out.push('\n');
                        }
                        out.push_str("    }\n");
                    }
                    out.push_str("}\n");
                }
                JavaType::Enum => {
                    out.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum {} {{\n", ci.name));
                    for v in &ci.enum_variants {
                        out.push_str(&format!("    {},\n", v));
                    }
                    out.push_str("}\n");
                }
            }
        } else {
            out.push_str("/* пуста */");
        }
        self.fmt_rust(&out)
    }
}
