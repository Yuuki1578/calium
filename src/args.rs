pub use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "Calium",
    version,
    about = "Calium, the calculator language implemented in Rust",
    long_about = "The Calium Language, is a calculator-like language that can parse and evaluate a simple mathematical expression",
    next_line_help = true
)]
/// Calium, the calculator language implementation in Rust
pub struct ArgsHandler {
    #[arg(short = 'r', long = "run", default_value = None, value_name = "FILE")]
    /// Optional file to be interpreted.
    /// If not provided, run REPL instead
    pub run: Option<String>,

    /// Optional expression to be evaluated.
    /// If not provided, run REPL instead
    #[arg(short = 'e', long = "eval", default_value = None, value_name = "EXPR")]
    pub eval: Option<Vec<String>>,
}

impl ArgsHandler {
    pub fn tuple(&self) -> (Option<&String>, Option<&Vec<String>>) {
        (self.run.as_ref(), self.eval.as_ref())
    }
}
