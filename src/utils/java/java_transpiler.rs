use crate::utils::java::java_parser::{JavaParser, JavaType};
use crate::utils::java::type_converter::TypeConverter;
use regex::Regex;
use crate::utils::java::java_constants::{JAVA_PRINT, JAVA_PRINTLN, RUST_PRINT, RUST_PRINTLN, RUST_UNIT, JAVA_STRING_ARRAY, RUST_STR_SLICE_ARRAY, JAVA_LANG_STRING};
use crate::utils::java::math_converter::MathConverter;

pub struct JavaTranspiler {
    parser: JavaParser,
    converter: TypeConverter,
    math_converter: MathConverter,
}

impl JavaTranspiler {
    pub fn new() -> Self {
        Self {
            parser: JavaParser::new(),
            converter: TypeConverter::new(),
            math_converter: MathConverter::new(),
        }
    }

    pub fn transpile_java_to_rust(&self, java_code: &str, _file_name: &str) -> String {
        let mut rust_code = String::new();
        if let Some(class_info) = self.parser.parse_java_class(java_code) {
            match class_info.type_info {
                JavaType::Class => {
                    rust_code.push_str(&format!("pub struct {} {{\n", class_info.name));
                    for field in &class_info.fields {
                        rust_code.push_str(&format!("    pub {}: {},\n", field.name, self.converter.convert_type(&field.var_type)));
                    }
                    rust_code.push_str("}\n\n");

                    rust_code.push_str(&format!("impl {} {{\n", class_info.name));
                    for method in &class_info.methods {
                        let rust_params = self.convert_parameters(&method.parameters);
                        let rust_return_type = self.converter.convert_type(&method.return_type);

                        let method_body = self.extract_method_body(java_code, &method.name);

                        let needs_mut_self = self.method_needs_mut_self(&method_body, &class_info.fields);

                        let full_params = if method.name == "main" {
                            let uses_self = method_body.contains("self.") ||
                                class_info.fields.iter().any(|field| method_body.contains(&field.name));

                            if uses_self {
                                if rust_params.is_empty() {
                                    if needs_mut_self { "&mut self" } else { "&self" }.to_string()
                                } else {
                                    format!("{}, {}", if needs_mut_self { "&mut self" } else { "&self" }, rust_params)
                                }
                            } else {
                                rust_params
                            }
                        } else {
                            if rust_params.is_empty() {
                                if needs_mut_self { "&mut self" } else { "&self" }.to_string()
                            } else {
                                format!("{}, {}", if needs_mut_self { "&mut self" } else { "&self" }, rust_params)
                            }
                        };

                        let mut current_indent: i32 = 2;

                        if method.name == "main" {
                            rust_code.push_str(&format!("    pub fn main({}) {{\n", full_params));
                        } else {
                            let return_str = if rust_return_type == RUST_UNIT { "".to_string() } else { format!(" -> {}", rust_return_type) };
                            rust_code.push_str(&format!("    pub fn {}({}){} {{\n", method.name, full_params, return_str));
                        }

                        for body_line in method_body.lines() {
                            let trimmed_body_line = body_line.trim();
                            let mut converted_body_line = trimmed_body_line.to_string();

                            if converted_body_line.starts_with("return ") {
                                if rust_return_type == RUST_UNIT {
                                    converted_body_line = "return;".to_string();
                                } else {
                                    if converted_body_line.contains(" + ") {
                                        converted_body_line = self.handle_return_concatenation(&converted_body_line, &class_info.fields);
                                    } else {
                                        converted_body_line = converted_body_line.replace("return ", "");
                                        if converted_body_line.ends_with(";") {
                                            converted_body_line.pop();
                                        }
                                        converted_body_line = format!("return {}", converted_body_line);

                                        if rust_return_type == "String" {
                                            converted_body_line = converted_body_line.replace("return ", "return ") + ".clone()";
                                        }
                                    }
                                }
                            } else {
                                let var_assignment_re = Regex::new(r"^\s*(?:\w+\s+)?(\bself\.\w+\b)\s*=\s*(.*?);$").unwrap();
                                if let Some(caps) = var_assignment_re.captures(&converted_body_line) {
                                    let var_name = caps.get(1).unwrap().as_str();
                                    let value = self.math_converter.convert_math_expression(caps.get(2).unwrap().as_str());
                                    converted_body_line = format!("{} = {};", var_name, value);
                                } else {
                                    if converted_body_line.contains("System.out.println") || converted_body_line.contains("System.out.print") {
                                        converted_body_line = self.convert_print_statement(&converted_body_line);
                                    }
                                }
                            }

                            converted_body_line = self.replace_field_names(&converted_body_line, &class_info.fields);

                            let if_re = Regex::new(r"^\s*if\s*\((.*)\)\s*\{?\s*$").unwrap();
                            if let Some(caps) = if_re.captures(&converted_body_line) {
                                let condition = caps.get(1).unwrap().as_str();
                                converted_body_line = format!("if {} {{", condition);
                            }

                            let var_init_re = Regex::new(r"^\s*int\s+(\bself\.\w+\b)\s*=\s*(.*?);$").unwrap();
                            if let Some(caps) = var_init_re.captures(&converted_body_line) {
                                let var_name = caps.get(1).unwrap().as_str();
                                let value = self.math_converter.convert_math_expression(caps.get(2).unwrap().as_str());
                                converted_body_line = format!("{} = {};", var_name, value);
                            }

                            if !trimmed_body_line.is_empty() && !converted_body_line.is_empty() {
                                if trimmed_body_line.starts_with("}") {
                                    current_indent = current_indent.saturating_sub(1);
                                }

                                rust_code.push_str(&format!("{:indent$}{}\n", "", converted_body_line, indent = (current_indent * 4) as usize));

                                if converted_body_line.ends_with(" {") {
                                    current_indent += 1;
                                }
                            }
                        }
                        rust_code.push_str("    }\n");
                    }
                    rust_code.push_str("}\n");
                }
                JavaType::Enum => {
                    rust_code.push_str(&format!("pub enum {} {{\n", class_info.name));
                    for variant in &class_info.enum_variants {
                        rust_code.push_str(&format!("    {},\n", variant));
                    }
                    rust_code.push_str("}\n\n");

                    rust_code.push_str(&format!("impl {} {{\n", class_info.name));
                    rust_code.push_str("}\n");
                }
            }
        } else {
            rust_code.push_str("// Failed to parse Java class\n");
            rust_code.push_str(java_code);
        }

        rust_code
    }

    fn method_needs_mut_self(&self, method_body: &str, fields: &[crate::utils::java::java_parser::VariableInfo]) -> bool {
        for field in fields {
            if method_body.contains(&format!("self.{} =", field.name)) ||
                method_body.contains(&format!("{} =", field.name)) {
                return true;
            }
        }
        false
    }

    fn replace_field_names(&self, line: &str, fields: &[crate::utils::java::java_parser::VariableInfo]) -> String {
        let mut result = line.to_string();

        for field in fields {
            let field_regex = Regex::new(&format!(r"\b{}\b", regex::escape(&field.name))).unwrap();

            let mut replacements = Vec::new();
            for mat in field_regex.find_iter(&result) {
                let start = mat.start();
                let end = mat.end();

                let prefix_start = if start >= 5 { start - 5 } else { 0 };
                let prefix = &result[prefix_start..start];

                if !prefix.ends_with("self.") {
                    replacements.push((start, end, format!("self.{}", field.name)));
                }
            }

            for (start, end, replacement) in replacements.iter().rev() {
                result.replace_range(*start..*end, replacement);
            }
        }

        result
    }

    fn handle_return_concatenation(&self, line: &str, fields: &[crate::utils::java::java_parser::VariableInfo]) -> String {
        if !line.starts_with("return ") || !line.contains(" + ") {
            return line.to_string();
        }

        let content = line.strip_prefix("return ").unwrap_or(line);
        let content = content.trim_end_matches(';');
        let parts: Vec<&str> = content.split(" + ").collect();

        if parts.len() >= 2 {
            let mut format_parts = Vec::new();
            let mut args = Vec::new();

            for part in parts {
                let trimmed = part.trim();

                if trimmed.starts_with('"') && trimmed.ends_with('"') {
                    format_parts.push(trimmed[1..trimmed.len()-1].to_string());
                } else if fields.iter().any(|f| f.name == trimmed && f.var_type == "String") {
                    format_parts.push("{}".to_string());
                    args.push(format!("self.{}", trimmed));
                } else if fields.iter().any(|f| f.name == trimmed) {
                    format_parts.push("{}".to_string());
                    args.push(format!("self.{}", trimmed));
                } else if trimmed.starts_with("self.") {
                    format_parts.push("{}".to_string());
                    args.push(trimmed.to_string());
                } else {
                    format_parts.push("{}".to_string());
                    args.push(trimmed.to_string());
                }
            }

            let format_string = format_parts.join("");
            if args.is_empty() {
                return format!("return \"{}\";", format_string);
            } else {
                return format!("return format!(\"{}\", {});", format_string, args.join(", "));
            }
        }

        line.to_string()
    }

    fn convert_print_statement(&self, line: &str) -> String {
        let mut result = line.to_string();

        if result.contains(" + ") {
            let content_re = Regex::new(r"System\.out\.(println|print)\((.*)\);").unwrap();
            if let Some(caps) = content_re.captures(&result) {
                let macro_name = match caps.get(1).unwrap().as_str() {
                    "println" => "println",
                    "print" => "print",
                    _ => "println"
                };
                let content = caps.get(2).unwrap().as_str();

                let parts: Vec<&str> = content.split(" + ").collect();
                if parts.len() >= 2 {
                    let mut format_parts = Vec::new();
                    let mut args = Vec::new();

                    for part in parts {
                        let trimmed = part.trim();

                        if trimmed.starts_with('"') && trimmed.ends_with('"') {
                            format_parts.push(trimmed[1..trimmed.len()-1].to_string());
                        } else if trimmed.starts_with("self.") {
                            format_parts.push("{}".to_string());
                            args.push(trimmed.to_string());
                        } else {
                            format_parts.push("{}".to_string());
                            args.push(format!("self.{}", trimmed));
                        }
                    }

                    let format_string = format_parts.join("");
                    if args.is_empty() {
                        result = format!("{}!(\"{}\");", macro_name, format_string);
                    } else {
                        result = format!("{}!(\"{}\", {});", macro_name, format_string, args.join(", "));
                    }
                }
            }
        } else {
            result = result.replace(JAVA_PRINTLN, RUST_PRINTLN);
            result = result.replace(JAVA_PRINT, RUST_PRINT);
        }

        result
    }

    fn is_string_field(&self, field_name: &str) -> bool {
        field_name.to_uppercase().contains("STRING") ||
            field_name.to_uppercase().contains("NAME") ||
            field_name.to_uppercase().contains("HWID") ||
            field_name.to_uppercase().contains("CLIENT") ||
            field_name.to_uppercase().contains("DOMEN")
    }

    fn convert_parameters(&self, java_params: &str) -> String {
        if java_params.is_empty() {
            return "".to_string();
        }

        java_params
            .split(",")
            .map(|param| {
                let parts: Vec<&str> = param.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    let java_type = parts[0];
                    let name = parts[1];
                    if java_type == JAVA_STRING_ARRAY && name == "args" {
                        format!("{}: {}", name, RUST_STR_SLICE_ARRAY)
                    } else {
                        format!("{}: {}", name, self.converter.convert_type(java_type))
                    }
                } else {
                    param.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn extract_method_body(&self, java_code: &str, method_name: &str) -> String {
        let mut body = String::new();
        let mut in_method = false;
        let mut brace_count = 0;

        let method_signature_pattern = Regex::new(&format!(r"^\s*(?:public|private|protected)?\s*(?:static)?\s*\w+\s+{}\s*\(.*\)\s*\{{", regex::escape(method_name))).unwrap();

        for line in java_code.lines() {
            let trimmed_line = line.trim();

            if !in_method {
                if method_signature_pattern.is_match(trimmed_line) {
                    in_method = true;
                    brace_count = 1;
                    continue;
                }
                continue;
            }

            for ch in trimmed_line.chars() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return body;
                        }
                    },
                    _ => {}
                }
            }

            body.push_str(&format!("{}\n", line));
        }
        body
    }
}