pub mod args;
pub mod lexer;
pub mod runtime;
pub mod syntax;
pub mod threaded;

use args::Args;

fn main() {
    let args = Args::new(1);

    if args.len() == 1 {
        let bait = String::new();
        let get_inner = args.get(0).unwrap_or(&bait);

        if runtime::read_file(get_inner)
            .inspect_err(|error| {
                eprintln!("{error}");
            })
            .is_err()
        {
            return;
        }

        return;
    } else {
        runtime::repl();
    }
}
