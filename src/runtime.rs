// use crate::lexer::Scanner;
// use std::fs;
// use std::io::{self, Write};

// pub fn read_file(path: &str) -> io::Result<()> {
//     let buffer = fs::read_to_string(path)?;
//     let mut scanner = Scanner::new(&buffer);
//     let scanned = scanner.scan_vector();

//     match scanned {
//         Ok(success) => println!("{success:#?}"),
//         Err(syn_err) => eprintln!("{syn_err}"),
//     }

//     Ok(())
// }

// pub fn repl() -> ! {
//     let mut line = 1_u128;

//     loop {
//         let mut input = String::new();

//         print!("({line})> ");

//         io::stdout().lock().flush().unwrap_or_default();

//         let _ = io::stdin().read_line(&mut input).unwrap_or_else(|error| {
//             eprintln!("{error}");
//             return 0;
//         });

//         let mut scanner = Scanner::new(input.trim());

//         match scanner.scan_vector() {
//             Ok(result) => println!("{result:#?}"),
//             Err(error) => eprintln!("{error}"),
//         }

//         line += 1;
//     }
// }
