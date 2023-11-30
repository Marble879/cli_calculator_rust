mod calculator {

    enum Input {
        Number(f64),
        Operator(Operation),
    }

    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    struct Calculator {
        result: f64,
        memory: f64,
    }

    impl Calculator {
        pub fn calculate(&self, operation: Operation, operand: f64) -> () {
            match operation {
                Operation::Add => self.result += operand,
                Operation::Subtract => self.result -= operand,
                Operation::Multiply => self.result *= operand,
                Operation::Divide => self.divide(operand),
            }
        }

        pub fn parse_input(&self, input: Input) {
            match input {
                Input::Number(num) => {
                    // handle numeric input
                    self.handle_number_input()
                    // update calcualtor state accordingly
                }
                Input::Operator(op) => {
                    // handle operator input
                    self.handle_operator_input()
                    // perform calculation based on the operator
                }
            }
        }

        fn divide(&self, operand: f64) {
            if operand != 0.0 {
                self.result /= operand;
            } else {
                println!("Error: Cannot divide by zero");
            }
        }

        fn handle_number_input(&self) {
            print!("placeholder")
        }

        fn handle_operator_input(&self) {
            print!("placeholder")
        }
    }
}
