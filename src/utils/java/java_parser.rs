use regex::Regex;

pub struct JavaParser;

impl JavaParser {
    pub fn new() -> Self {
        Self
    }

    pub fn should_skip_line(&self, line: &str) -> bool {
        line.is_empty()
            || line.starts_with("//")
            || line.starts_with("/*")
            || line.starts_with("*")
    }

    pub fn is_import_or_package(&self, line: &str) -> bool {
        line.starts_with("import ") || line.starts_with("package ")
    }

    pub fn extract_class_declaration(&self, line: &str) -> Option<(String, String)> {
        let re = Regex::new(r"(?:public\s+)?(?:class|enum)\s+(\w+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let type_keyword = caps.get(0).unwrap().as_str().split_whitespace().nth(1).unwrap_or("class").to_string();
            Some((caps[1].to_string(), type_keyword))
        } else {
            None
        }
    }
    
    pub fn parse_method_declaration(&self, line: &str) -> Option<MethodInfo> {
        let re = Regex::new(r"(?:public|private|protected)?\s*(?:static)?\s*(\w+)\s+(\w+)\s*\((.*?)\)\s*\{?").unwrap();
        if let Some(caps) = re.captures(line) {
            Some(MethodInfo {
                return_type: caps[1].to_string(),
                name: caps[2].to_string(),
                parameters: caps[3].to_string(),
            })
        } else {
            None
        }
    }

    pub fn parse_variable_declaration(&self, line: &str) -> Option<VariableInfo> {
        // игнорир строк стартующих с return, для избежания ошибочной интерпретации как полей
        if line.trim().starts_with("return ") {
            return None;
        }

        let var_re = Regex::new(r"(\w+)\s+(\w+)\s*=\s*(.*?);").unwrap();
        if let Some(caps) = var_re.captures(line) {
            Some(VariableInfo {
                var_type: caps[1].to_string(),
                name: caps[2].to_string(),
                value: Some(caps[3].to_string()),
            })
        } else {
            let decl_re = Regex::new(r"(\w+)\s+(\w+);").unwrap(); // обьяление без инициализации
            if let Some(caps) = decl_re.captures(line) {
                Some(VariableInfo {
                    var_type: caps[1].to_string(),
                    name: caps[2].to_string(),
                    value: None,
                })
            } else {
                None
            }
        }
    }

    pub fn extract_enum_variants(&self, java_code: &str) -> Vec<String> {
        let re = Regex::new(r"^\s*([A-Z_]+),\s*$").unwrap();
        let mut variants = Vec::new();
        for line in java_code.lines() {
            let trimmed = line.trim();
            if let Some(caps) = re.captures(trimmed) {
                variants.push(caps[1].to_string());
            }
        }
        variants
    }

    pub fn parse_java_class(&self, java_code: &str) -> Option<JavaTypeInfo> {
        let mut type_name: Option<String> = None;
        let mut type_kind: Option<JavaType> = None;
        let mut methods: Vec<MethodInfo> = Vec::new();
        let mut fields: Vec<VariableInfo> = Vec::new();

        for line in java_code.lines() {
            let trimmed = line.trim();

            if self.should_skip_line(trimmed) || self.is_import_or_package(trimmed) {
                continue;
            }

            if type_name.is_none() {
                if let Some((name, type_keyword)) = self.extract_class_declaration(trimmed) {
                    type_name = Some(name);
                    type_kind = Some(match type_keyword.as_str() {
                        "enum" => JavaType::Enum,
                        _ => JavaType::Class,
                    });
                    continue;
                }
            }

            if let Some(method_info) = self.parse_method_declaration(trimmed) {
                methods.push(method_info);
                continue;
            }

            if let Some(variable_info) = self.parse_variable_declaration(trimmed) {
                fields.push(variable_info);
                continue;
            }
        }

        if let Some(name) = type_name {
            let mut enum_variants = Vec::new();
            if let Some(JavaType::Enum) = type_kind {
                enum_variants = self.extract_enum_variants(java_code);
            }
            Some(JavaTypeInfo {
                name,
                type_info: type_kind.unwrap_or(JavaType::Class),
                methods,
                fields,
                enum_variants,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum JavaType {
    Class,
    Enum,
}

#[derive(Debug)]
pub struct JavaTypeInfo {
    pub name: String,
    pub type_info: JavaType,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<VariableInfo>,
    pub enum_variants: Vec<String>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub return_type: String,
    pub name: String,
    pub parameters: String,
}

#[derive(Debug)]
pub struct VariableInfo {
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct ForLoopInfo {
    pub var_name: String,
    pub start: String,
    pub end: String,
}