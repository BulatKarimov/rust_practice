pub fn run() {
    for element in 1..=100 {
        println!("{}: {}", element, match_fizz_buzz(element));
    }
}

fn match_fizz_buzz(number: i32) -> String {
    match number {
        _ if number % 3 == 0 && number % 5 == 0 => "FizzBuzz".to_string(),
        _ if number % 3 == 0 => "Fizz".to_string(),
        _ if number % 5 == 0 => "Buzz".to_string(),
        _ => number.to_string(),
    }
}
