use std::io;

const AVAILABLE_OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub fn run() {
    loop {
        println!();

        let first_number: f64 = read_number("Enter first number:");
        let operator: char = read_operator("Enter operator: (+, -, /, *):");
        let second_number: f64 = read_number("Enter second number:");

        let result = match calculate(first_number, operator, second_number) {
            Some(number) => number,
            None => {
                println!("Error: Division by zero!\n");
                continue;
            }
        };

        println!("{first_number} {operator} {second_number} = {result:.2}",);

        if !need_restart() {
            break;
        }
    }
}

fn read_number(title: &str) -> f64 {
    loop {
        match read_input(title).parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("Entered invalid number!"),
        }
    }
}

fn read_operator(title: &str) -> char {
    loop {
        match read_input(title).chars().next() {
            Some(operator) if AVAILABLE_OPERATORS.contains(&operator) => return operator,
            Some(_) => println!("Unknown operator!"),
            None => println!("Empty input!"),
        }
    }
}

fn read_input(title: &str) -> String {
    println!("{title}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn calculate(first_number: f64, operator: char, second_number: f64) -> Option<f64> {
    match operator {
        '+' => Some(first_number + second_number),
        '-' => Some(first_number - second_number),
        '*' => Some(first_number * second_number),
        '/' => {
            if second_number != 0.0 {
                Some(first_number / second_number)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn need_restart() -> bool {
    read_input("Press 'y' to restart").eq_ignore_ascii_case("y")
}
