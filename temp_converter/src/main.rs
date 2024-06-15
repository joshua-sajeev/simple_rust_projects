use std::io::{self, Write};

fn get_choice() -> u8 {
    let mut input = String::new();
    loop {
        input.clear();
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("3: Exit");
        print!("Enter your choice: ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}

fn main() {
    let mut result: f64 ;
    loop{
        match get_choice() {
            1 => {
                let celsius = get_temperature();
                result = celsius_to_fahrenheit(celsius);
                println!("{celsius}°C is {result}°F\n");
            },
            2 => {
                let fahrenheit = get_temperature();
                result = fahrenheit_to_celsius(fahrenheit);
                println!("{fahrenheit}°F is {result}°C\n");
            },
            3 => {
                println!("Exiting....");
                break;
            },
            _ => println!("Invalid Choice, Input 1, 2 or 3\n"),
        }
    }
}

fn get_temperature() -> f64{
    let mut input = String::new();
    loop{
        input.clear();
        print!("Enter your temperature: ");

        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a valid input"),
        }
    }
}
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
