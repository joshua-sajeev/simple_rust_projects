use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        let (low_range, high_range) = get_ranges(&mut input);
        let random_number = rand::thread_rng().gen_range(low_range..=high_range);
        play_game(&mut input, random_number);

        if !play_again(&mut input) {
            break;
        }
    }
}

fn get_ranges(input: &mut String) -> (i32, i32) {
    let mut low_range: i32;
    let mut high_range: i32;

    loop {
        input.clear();
        loop {
            input.clear();
            print!("Enter the lower range: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(input).expect("Failed to read line");
            match input.trim().parse() {
                Ok(num) => {
                    low_range = num;
                    break;
                }
                Err(_) => println!("Invalid input, please enter a number."),
            }
        }

        input.clear();
        loop {
            input.clear();
            print!("Enter the higher range: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(input).expect("Failed to read line");
            match input.trim().parse() {
                Ok(num) => {
                    high_range = num;
                    break;
                }
                Err(_) => println!("Invalid input, please enter a number."),
            }
        }

        if low_range < high_range {
            return (low_range, high_range);
        } else {
            println!("\nLower Range should be less than Higher Range\n");
        }
    }
}

fn play_game(input: &mut String, random_number: i32) {
    loop {
        input.clear();
        print!("Enter your guess: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(input).expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(num) => {
                if num < random_number {
                    println!("Too Low");
                } else if num > random_number {
                    println!("Too High");
                } else {
                    println!("Hurray");
                    break;
                }
            }
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}

fn play_again(input: &mut String) -> bool {
    print!("1: New Game\nAny other key: Exit\n");
    let _ = io::stdout().flush();
    input.clear();
    io::stdin().read_line(input).expect("Failed to read line");

    match input.trim().parse::<u8>() {
        Ok(1) => true,
        _ => false,
    }
}
