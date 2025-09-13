pub struct MathConverter;

impl MathConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_math_expression(&self, expression: &str) -> String {
        let mut converted_expression = expression.to_string();

        converted_expression = converted_expression.replace("==", "==");
        converted_expression = converted_expression.replace("!=", "!=");
        converted_expression = converted_expression.replace(">=", ">=");
        converted_expression = converted_expression.replace("<=", "<=");
        converted_expression = converted_expression.replace(">", ">");
        converted_expression = converted_expression.replace("<", "<");
        converted_expression = converted_expression.replace("+", "+");
        converted_expression = converted_expression.replace("-", "-");
        converted_expression = converted_expression.replace("*", "*");
        converted_expression = converted_expression.replace("/", "/");
        converted_expression = converted_expression.replace("%", "%");

        converted_expression
    }
}
