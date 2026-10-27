use std::{
    env, fs,
    io::{self, Read},
};

use ypbank::{
    InMemoryOperationStorage,
    error::LibError,
    model::{Format, OperationStorage},
};

fn run() -> Result<(), LibError> {
    let mut args: Vec<String> = env::args().collect();

    let use_stdin = args.len() == 4;
    let use_file = args.len() == 5;

    if !use_stdin && !use_file {
        eprintln!("Использование:");
        eprintln!(
            "  cargo run --bin converter -- <path/to/file_in> <format_in> <path/to/file_out> <format_out>"
        );
        eprintln!(
            "  cargo run --bin converter -- <format_in> <path/to/file_out> <format_out> < path/to/file_in"
        );
        eprintln!("Поддерживаемые форматы:");
        eprintln!("  - txt");
        eprintln!("  - csv");
        eprintln!("  - bin");
        std::process::exit(1);
    }

    let format_out: Format = args.pop().unwrap().parse()?;
    let mut file_out = fs::File::create(&args.pop().unwrap()).unwrap();
    let format_in: Format = args.pop().unwrap().parse()?;

    let file_in: Box<dyn Read> = if use_stdin {
        Box::new(io::stdin().lock())
    } else {
        Box::new(fs::File::open(&args.pop().unwrap()).unwrap())
    };

    let mut storage = InMemoryOperationStorage::new();
    storage.load(format_in, file_in)?;
    storage.save(format_out, &mut file_out)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
