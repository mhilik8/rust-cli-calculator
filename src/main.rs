pub trait Math {
    fn value(&self) -> i32;
}

enum Operation {
    Add,
    Sub
}

struct BinaryOperation {
    left_operand: i32,
    right_operand: i32,
    action: Operation
}

impl Math for BinaryOperation{
    fn value(&self) -> i32 {
        match self.action {
            Operation::Add => self.left_operand + self.right_operand,
            Operation::Sub => self.left_operand - self.right_operand,
        }
    }
}

fn main() {
    println!("Hello to Rust CLI Calculator");
    let input_str = "5+6";
    let parsed = parse_math(input_str);
    let my_binary_operation = BinaryOperation{
        left_operand:parsed.0,
        right_operand:parsed.1,
        action: Operation::Add
    };
    let results = my_binary_operation.value();
        println!("results: {}", results);
}

fn parse_math(to_parse: &str) -> (i32, i32){
    let parts: Vec<&str> = to_parse.split("+").collect();
    let left: i32 = parts[0].parse().unwrap();
    let right: i32 = parts[1].parse().unwrap();
    println!("left: {}", left);
    println!("right: {}", right);
    (left, right)
}
