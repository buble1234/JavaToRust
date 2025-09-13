Java to Rust Transpiler
A simple transpiler that converts Java code to Rust. Written in Rust and handles basic language constructs.
What it does:

Converts Java classes to Rust structs
Translates methods with automatic &self/&mut self detection
Handles basic data types (int -> i32, String -> String, etc.)
Properly translates string concatenation
Supports if-else blocks with correct indentation
Converts enums
Processes println statements

Usage:
Put .java files in the input/ folder, run cargo run. Output goes to out/ folder.
Example:
Java classes with fields and getters become Rust structs with impl blocks. Methods get proper signatures, strings are cloned where needed, concatenation works through .clone() + & pattern.
This is a basic version - only simple constructs for now, no complex generics or advanced features. Planning to expand functionality.
Status: work in progress, will be updated.

//\\

Транспилер Java в Rust
Простой транспилер для конвертации Java кода в Rust. Написан на Rust и обрабатывает основные конструкции языка.
Что умеет:

Конвертирует Java классы в Rust структуры
Переводит методы с автоматическим определением &self/&mut self
Обрабатывает базовые типы данных (int -> i32, String -> String и т.д.)
Правильно транслирует конкатенацию строк
Поддерживает if-else блоки с корректными отступами
Конвертирует enum'ы
Обрабатывает println операторы

Использование:
Кладите .java файлы в папку input/, запускайте cargo run. Результат в папке out/.
Пример:
Java класс с полями и геттерами превращается в Rust структуру с impl блоком. Методы получают правильные сигнатуры, строки клонируются где нужно, конкатенация работает через .clone() + & паттерн.
Это базовая версия - пока только простые конструкции, без сложных дженериков и продвинутых фич. Планирую расширять функционал.
Статус: в разработке, будет дополняться.
