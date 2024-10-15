pub mod args;
pub mod lexer;
pub mod syntax;
pub mod threaded;

use lexer::Scanner;
use std::io::{self, Write};

pub fn repl() -> ! {
    let mut line = 1_u128;

    loop {
        let mut input = String::new();

        print!("({line})> ");

        io::stdout().lock().flush().unwrap_or_default();

        let _ = io::stdin().read_line(&mut input).unwrap_or_else(|error| {
            eprintln!("{error}");
            return 0;
        });

        let mut scanner = Scanner::new(input.trim());

        match scanner.scan() {
            Ok(result) => println!("{result:#?}"),
            Err(error) => eprintln!("{error}"),
        }

        line += 1;
    }
}

fn main() {
    repl();
}
