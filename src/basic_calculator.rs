use crate::unknown_operator_error;

pub struct Calculator {
    pub first_number: f32,
    pub operation: String,
    pub second_number: f32,
}

impl Calculator {
    pub fn perform_calculation(&self) -> Result<f32, unknown_operator_error::UnknownOperatorError> {
        match self.operation.as_str() {
            "+" => Ok(self.first_number + self.second_number),
            "-" => Ok(self.first_number - self.second_number),
            "*" => Ok(self.first_number * self.second_number),
            "/" => Ok(self.first_number / self.second_number),
            _ => Err(unknown_operator_error::UnknownOperatorError),
        }
    }
}
