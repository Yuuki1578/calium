#![allow(unused)]
pub mod args;
pub mod ast;
pub mod lexer;
pub mod runtime;
pub mod syntax;

use args::Args;
use std::process::ExitCode;

fn main() {}

// fn main() -> ExitCode {
//     let args = Args::new(1);

//     if args.len() == 1 {
//         let bait = String::new();
//         let get_inner = args.get(0).unwrap_or(&bait);

//         match runtime::read_file(get_inner) {
//             Ok(_) => return ExitCode::SUCCESS,
//             Err(error) => {
//                 let os_err = error.raw_os_error().unwrap_or(1) as u8;

//                 eprintln!("{error}");
//                 return ExitCode::from(os_err);
//             }
//         }
//     }

//     runtime::repl();
// }
