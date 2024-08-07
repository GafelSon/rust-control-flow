use std::io::{self, Write};

fn main() {
    println!(":::Temperatures Converter:::");

    loop {
        // Prompt for temperature
        let temperature = prompt("-> Enter the temperature: ");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature input. Please enter a valid number.");
                continue;
            }
        };

        // Prompt for conversion type
        println!("-> Select your quantity: ");
        println!("  1.\tCelsius to Fahrenheit");
        println!("  2.\tFahrenheit to Celsius");

        let conversion_type = prompt("");
        let conversion_type: i8 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection. Please enter 1 or 2.");
                continue;
            }
        };

        match conversion_type {
            1 => {
                let converted = celsius_to_fahrenheit(temperature);
                println!("Converted! It's {:.2}°F", converted);
                break;
            }
            2 => {
                let converted = fahrenheit_to_celsius(temperature);
                println!("Converted! It's {:.2}°C", converted);
                break;
            }
            _ => println!("Invalid selection. Please enter 1 or 2."),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // Ensure the message is printed immediately
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
