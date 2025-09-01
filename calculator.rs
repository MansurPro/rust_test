use std::io;

fn main() {
    println!("Simple Rust Calculator");
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    println!("Enter operator (+, -, *, /):");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read input");
    let op = op.trim();

    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero");
                return;
            } else {
                num1 / num2
            }
        },
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, op, num2, result);
}
