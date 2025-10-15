use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run_auto() {
    println!("Guest the number!");

    let number = rand::rng().random_range(1..=100);
    println!("The number is {}", number);

    auto(&number)
}

pub fn run_manual() {
    println!("Guest the number!");

    let number = rand::rng().random_range(1..=100);
    println!("The number is {}", number);

    manual(&number);
}

fn manual(number: &u32) {
    let mut attempts = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        attempts += 1;

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!. Attempts: {attempts}");
                break;
            }
        }
    }
}

fn auto(number: &u32) {
    let mut attempts = 0;
    let mut range = 1..=4294967295;

    loop {
        println!("Please input your guess.");

        let min = *range.start();
        let max = *range.end();

        println!("min: {min}, max: {max}");

        let shift = (max - min + 1) / 2;
        let guess = shift + min;

        println!("You guessed: {guess}");
        attempts += 1;

        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Too small!");
                range = guess..=max;
            }
            Ordering::Greater => {
                println!("Too big!");
                range = min..=guess;
            }
            Ordering::Equal => {
                println!("You win!. Attempts: {attempts}");
                break;
            }
        }
    }
}
