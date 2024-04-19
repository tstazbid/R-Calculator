#![allow(dead_code)]
use std::io;

fn warnings() {
    print!("!!         ")
}

fn main() {
    println!("\n\n**************                             Welcome to R- Calculator!! (Made by RUST)                             **************");
    println!("Hi, you are now in our monitor\n");

    loop {
        println!("Please input an arithmetic expression:");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        input.retain(|c| !c.is_whitespace());

        let input: &str = input.trim();

        // println!("{}ok", input);

        if input.is_empty() {
            warnings();
            println!("Input cannot be empty. Please try again.\n");
            continue;
        }

        if !input
            .chars()
            .all(|c: char| c.is_digit(10) || "+-*/% ".contains(c))
        {
            warnings();
            println!("Invalid input: Input contains invalid characters.\n");
            continue;
        }

        let mut parts: Vec<String> = Vec::new();

        let mut current_part: String = String::new();
        for c in input.chars() {
            if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' {
                if !current_part.is_empty() {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                }
                parts.push(c.to_string());
            } else {
                current_part.push(c);
            }
        }

        if !current_part.is_empty() {
            parts.push(current_part.trim().to_string());
        }

        // for part in &parts {
        //     println!("Part: {}", part);
        // }

        // print!("hi");

        let mut result: f64;
        let mut operator: &str = "*";
        let mut chk: bool = false;

        if parts[0].chars().all(|c: char| c.is_numeric()) || parts[0] == "+" || parts[0] == "-" {
            if parts[0] == "-" {
                result = -1.0;
                chk = true;
                // println!("ok line");
            } else if parts[0] == "+" {
                result = 1.0;
                chk = true;
                // println!("ok line");
            } else {
                result = parts[0].parse().expect("Failed to parse first operand");
            }

            for part in parts.iter().skip(1) {
                if ["+", "-", "*", "/", "%"].contains(&&part[..]) {
                    if chk {
                        warnings();
                        println!("Invalid expression: Two operators encountered consecutively");
                        break;
                    } else {
                        operator = part;
                        chk = true;
                    }
                } else {
                    let num: f64 = part.parse().expect("Failed to parse operand");
                    match operator {
                        "+" => result += num,
                        "-" => result -= num,
                        "*" => result *= num,
                        "/" => result /= num,
                        "%" => result %= num,
                        _ => panic!("Invalid operator: '{}'", operator),
                    }
                    chk = false;
                }
            }

            println!("Result: {}", result);

            println!("Do you want to continue? (yes/no)");

            let mut choice: String = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read the line");
            let choice: String = choice.trim().to_lowercase();

            if choice == "no" {
                println!("||         Thanks for using R-Calculator  :)          ||\n");
                break;
            }
        } else {
            warnings();
            println!("Invalid expression: First part must be a number or '+' or '-'");
        }
    }
}
