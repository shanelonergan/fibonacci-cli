use std::io;
extern crate colour;
use console::style;

fn main() {
    println!("\n\n🐚  {} 🐚", style("Welcome to the Fibonacci Generator!").cyan());

    loop {
        println!("\nPlease enter the {} of the sequence you would like to see:\n", style("number").yellow().bold());

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                colour::red_ln!("\nPlease enter a valid number");
                continue;
            },
        };

        let output = fib(input);
        println!("\nThe {}{} number of the Fibonacci sequence is {}\n", style(input).yellow().bold(), style("th").yellow().bold(), style(output).cyan().bold());
        break;
    };

}

fn fib(num: u32) -> u32 {
    if num < 2 {
        return num;
    }

    fib(num - 1) + fib(num - 2)
}
