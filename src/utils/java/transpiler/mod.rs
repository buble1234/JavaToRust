pub mod builder;
pub mod format;
pub mod line;
pub mod control;
pub mod print;
pub mod fields;
pub mod params;
pub mod switch;
pub mod lambda;

pub mod function;
pub mod varargs;
pub mod arrays;
pub mod loops;
pub mod ternary;
pub mod decl;

use crate::utils::java::java_parser::JavaParser;
use crate::utils::java::type_converter::TypeConverter;
use crate::utils::java::math_converter::MathConverter;

pub struct JavaTranspiler {
    pub(crate) p: JavaParser,
    pub(crate) t: TypeConverter,
    pub(crate) m: MathConverter,
}

impl JavaTranspiler {
    pub fn new() -> Self {
        Self { p: JavaParser::new(), t: TypeConverter::new(), m: MathConverter::new() }
    }
}
