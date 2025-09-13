use std::fs;
use std::path::Path;

use crate::utils::java::java_transpiler::JavaTranspiler;

pub struct FileProcessor {
    transpiler: JavaTranspiler,
}

impl FileProcessor {
    pub fn new() -> Self {
        Self {
            transpiler: JavaTranspiler::new(),
        }
    }

    pub fn process_directory(&self, input_dir: &Path) {
        if let Ok(entries) = fs::read_dir(input_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if self.is_java_file(&path) {
                    self.process_java_file(&path);
                }
            }
        }
    }

    fn is_java_file(&self, path: &Path) -> bool {
        path.extension().and_then(|s| s.to_str()) == Some("java")
    }

    fn process_java_file(&self, path: &Path) {
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let output_path = format!("out/{}.rs", file_name);

        match fs::read_to_string(path) {
            Ok(java_code) => {
                let rust_code = self.transpiler.transpile_java_to_rust(&java_code, file_name);

                if let Err(e) = fs::write(&output_path, rust_code) {
                    eprintln!("Ошибка записи файла {}: {}", output_path, e);
                } else {
                    println!("Успешно преобразован: {} -> {}", path.display(), output_path);
                }
            }
            Err(e) => eprintln!("Ошибка чтения файла {}: {}", path.display(), e),
        }
    }
}