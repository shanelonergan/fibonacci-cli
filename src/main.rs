use std::io;
extern crate colour;

fn main() {
    colour::cyan_ln!("\n🐚  Welcome to the Fibonacci Generator! 🐚");

    loop {
        colour::cyan_ln!("\nPlease enter the number of the sequence you would like to see:\n");

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
        println!("\nThe {}th number of the Fibonacci sequence is {}", input, output);
        break;
    };

}

fn fib(num: u32) -> u32 {
    if num < 2 {
        return num;
    }

    fib(num - 1) + fib(num - 2)
}
