#![allow(dead_code)]

// use core::num;
use std::io;

fn main() {
    println!("**************          Welcome to R- Calculator!! (Made by RUST)           **************");

    loop {
        println!("Please select an operatioon:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Modulus (%)");
        println!("6. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read your input");

        let choice: u32 =  match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if choice == 6 {
            println!("||      Thanks for using R-Calculator!      ||");
            println!("You are exiting now!");
            break;
        }

        println!("Enter first number:");
        let mut num1: String = String::new();

        io::stdin().read_line(&mut num1).expect("Failed to read num 1");

        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("Error: Cannot divide by zero. Inter the number again!");
                    continue;
                }
                num1 / num2
            }
            
            5 => num1 % num2,
            _ => {
                println!("Invalid choice. Please select a valid operation.");
                continue;
            }
        };
        println!("Result: {}", result);
    }
}
