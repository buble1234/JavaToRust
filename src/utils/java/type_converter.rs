use std::collections::HashMap;
use crate::utils::java::java_constants::{JAVA_INT, JAVA_VOID, RUST_I32, RUST_UNIT, JAVA_LANG_STRING, RUST_STD_STRING};

pub struct TypeConverter {
    type_mappings: HashMap<String, String>,
}

impl TypeConverter {
    pub fn new() -> Self {
        let mut type_mappings = HashMap::new();

        // примитивные типы
        type_mappings.insert(JAVA_VOID.to_string(), RUST_UNIT.to_string());
        type_mappings.insert(JAVA_INT.to_string(), RUST_I32.to_string());
        type_mappings.insert("double".to_string(), "f64".to_string());
        type_mappings.insert("float".to_string(), "f32".to_string());
        type_mappings.insert("long".to_string(), "i64".to_string());
        type_mappings.insert("short".to_string(), "i16".to_string());
        type_mappings.insert("byte".to_string(), "i8".to_string());
        type_mappings.insert("char".to_string(), "char".to_string());
        type_mappings.insert("boolean".to_string(), "bool".to_string());

        // объектные типы
        type_mappings.insert(JAVA_LANG_STRING.to_string(), RUST_STD_STRING.to_string());
        type_mappings.insert("Integer".to_string(), RUST_I32.to_string());
        type_mappings.insert("Double".to_string(), "f64".to_string());
        type_mappings.insert("Float".to_string(), "f32".to_string());
        type_mappings.insert("Long".to_string(), "i64".to_string());
        type_mappings.insert("Short".to_string(), "i16".to_string());
        type_mappings.insert("Byte".to_string(), "i8".to_string());
        type_mappings.insert("Character".to_string(), "char".to_string());
        type_mappings.insert("Boolean".to_string(), "bool".to_string());

        Self { type_mappings }
    }

    pub fn convert_type(&self, java_type: &str) -> String {
        self.type_mappings
            .get(java_type)
            .cloned()
            .unwrap_or_else(|| java_type.to_string())
    }
}