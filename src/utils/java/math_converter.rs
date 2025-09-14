use regex::Regex;

pub struct MathConverter;

impl MathConverter {
    pub fn new() -> Self { Self }

    pub fn convert_math_expression(&self, s: &str) -> String {
        let mut x = s.to_string();

        x = x.replace("Math.PI", "std::f64::consts::PI");
        x = x.replace("Math.E", "std::f64::consts::E");

        x = Regex::new(r"Math\.max\(\s*([^,()]+)\s*,\s*([^)]+)\)")
            .unwrap()
            .replace_all(&x, "std::cmp::max($1, $2)")
            .into_owned();
        x = Regex::new(r"Math\.min\(\s*([^,()]+)\s*,\s*([^)]+)\)")
            .unwrap()
            .replace_all(&x, "std::cmp::min($1, $2)")
            .into_owned();

        x = Regex::new(r"Math\.abs\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1).abs()")
            .into_owned();

        x = Regex::new(r"Math\.sqrt\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).sqrt()")
            .into_owned();

        x = Regex::new(r"Math\.pow\(\s*([^,()]+)\s*,\s*([^)]+)\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).powf(($2) as f64)")
            .into_owned();

        x = Regex::new(r"Math\.floor\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).floor()")
            .into_owned();
        x = Regex::new(r"Math\.ceil\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).ceil()")
            .into_owned();
        x = Regex::new(r"Math\.round\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).round() as i64")
            .into_owned();

        x = Regex::new(r"Math\.sin\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).sin()")
            .into_owned();
        x = Regex::new(r"Math\.cos\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).cos()")
            .into_owned();
        x = Regex::new(r"Math\.tan\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).tan()")
            .into_owned();
        x = Regex::new(r"Math\.atan2\(\s*([^,()]+)\s*,\s*([^)]+)\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).atan2(($2) as f64)")
            .into_owned();

        x = Regex::new(r"Math\.log10\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).log10()")
            .into_owned();
        x = Regex::new(r"Math\.log\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).ln()")
            .into_owned();
        x = Regex::new(r"Math\.exp\(\s*([^\(\)]*?)\s*\)")
            .unwrap()
            .replace_all(&x, "($1 as f64).exp()")
            .into_owned();

        x = Regex::new(r"Math\.random\(\s*\)")
            .unwrap()
            .replace_all(&x, "rand::random::<f64>()")
            .into_owned();

        x
    }
}
