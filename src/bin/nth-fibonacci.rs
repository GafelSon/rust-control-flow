use std::io::{self, Write};

fn main() {
    loop {
        let number = prompt("Enter your number -> ");
        let number: i64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong value!");
                continue;
            }
        };

        match gen_fibonacci(number) {
            Ok(result) => println!("Fibonacci number: {}", result),
            Err(e) => println!("Error: {}", e),
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

fn gen_fibonacci(n: i64) -> Result<i64, &'static str> {
    if n <= 0 {
        return Err("Incorrect input");
    }
    if n == 1 {
        return Ok(0);
    }
    if n == 2 {
        return Ok(1);
    }

    // Iterative approach
    let mut a = 0;
    let mut b = 1;
    for _ in 3..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    Ok(b)
}
