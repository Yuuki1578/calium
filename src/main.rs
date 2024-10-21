#[macro_use]
pub mod args;

#[macro_use]
pub mod lexer;

pub mod rules;
pub mod runtime;
pub mod syntax;
pub mod tests;

use args::{ArgsHandler, Parser};
use std::process::ExitCode;

fn main() -> ExitCode {
    let parsed = ArgsHandler::parse();

    match parsed.tuple() {
        (Some(run), None) => {
            if let Err(error) = runtime::read_file(run) {
                let err = error.raw_os_error().unwrap_or(1);

                return ExitCode::from(err as u8);
            } else {
                return ExitCode::SUCCESS;
            }
        }

        (None, Some(eval)) => {
            if let Err(syntax_err) = runtime::eval(eval) {
                eprintln!("{syntax_err}");

                return ExitCode::FAILURE;
            } else {
                return ExitCode::SUCCESS;
            }
        }

        (None, None) | (Some(_), Some(_)) => {
            runtime::repl();
        }
    }
}
