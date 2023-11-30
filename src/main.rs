mod basic_calculator;
mod unknown_operator_error;

use basic_calculator::Calculator;

fn main() {
    // Get args
    let first_number = std::env::args()
        .nth(1)
        .expect("First number should be input")
        .parse::<f32>()
        .unwrap();
    let operation = std::env::args()
        .nth(2)
        .expect("Mathematical operator should be input");
    let second_number = std::env::args()
        .nth(3)
        .expect("Second number should be input")
        .parse::<f32>()
        .unwrap();

    let args = Calculator {
        first_number,
        operation,
        second_number,
    };

    let calculation_result = args.perform_calculation();
    match calculation_result {
        Ok(value) => println!("Calculation result: {}", value),
        Err(err) => eprintln!("Error: {}", err),
    }
}
