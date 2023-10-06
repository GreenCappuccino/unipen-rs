extern crate unipen;

use std::env;
use std::path::Path;

use unipen::statements;

fn main() -> Result<(), &'static str> {
    // Get the command line arguments.
    let mut args = env::args();

    // Skip the first argument, which is the program name.
    args.next();

    // Get the path to the file to parse.
    let Some(path_str) = args.next() else {
        return Err("Usage: unipen <file> [include-dir]");
    };
    let path = Path::new(path_str.as_str());

    // Get the path to the include directory.
    let include_arg = args.next();
    let include = include_arg.as_deref().map(Path::new);

    // Parse the file.
    let statements = match statements::parse(path, include) {
        Ok(statements) => statements,
        Err(error) => {
            eprintln!("{error}");
            return Err("Error occured during parsing.");
        }
    };

    // Print the statements.
    serde_json::to_writer(std::io::stdout(), &statements).expect("Failed to write to stdout.");

    Ok(())
}
