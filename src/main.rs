mod logic;
mod token;

use std::{env, io::Write};

fn main() {
    let verbose = parse_args();

    println!("Command Line Interface Calculator\n");
    println!("Enter `exit` to close the program");
    let mut input = String::new();

    loop {
        input.clear();

        print!("\n> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.to_lowercase() == String::from("exit\r\n") {
            break;
        }

        let tokens = logic::parse(&input);
        let postfix_tokens = logic::infix_to_postfix(&tokens);
        let result = logic::evaluate_postfix(&postfix_tokens);

        if verbose {
            print!("      User input: {}", input);
            std::io::stdout().flush().unwrap();
            println!("          Tokens: {:?}", tokens);
            println!("Postfix notation: {:?}", postfix_tokens);
            println!("          Result: {}", result);
        } else {
            println!("{}", result)
        }
    }
}

fn parse_args() -> bool {
    let args: Vec<String> = env::args().collect();
    // println!("cli args: {:?}", args);

    if args.len() >= 2 {
        args[1].to_lowercase() == String::from("verbose")
    } else {
        false
    }
}
