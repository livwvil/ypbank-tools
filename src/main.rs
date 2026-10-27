use std::{
    env, fs,
    io::{self, Read},
};

use ypbank::{
    InMemoryOperationStorage,
    error::LibError,
    model::{Format, OperationStorage},
};

fn show_usage() {
    eprintln!("Использование:");
    eprintln!("  cargo run --bin converter -- <path/to/file_in> <format_in> <format_out>");
    eprintln!("  cargo run --bin converter -- <format_in> <format_out> < path/to/file_in");
    eprintln!("Поддерживаемые форматы:");
    eprintln!("  - txt");
    eprintln!("  - csv");
    eprintln!("  - bin");
}

fn run() -> Result<(), LibError> {
    let mut args: Vec<String> = env::args().collect();

    let use_stdin = args.len() == 3;
    let use_file = args.len() == 4;

    if !use_stdin && !use_file {
        show_usage();
        std::process::exit(1);
    }

    let format_out: Format = args.pop().unwrap().parse()?;
    let format_in: Format = args.pop().unwrap().parse()?;

    let file_in: Box<dyn Read> = if use_stdin {
        Box::new(io::stdin().lock())
    } else {
        let file_in_path = args.pop().unwrap();
        Box::new(fs::File::open(&file_in_path).unwrap_or_else(|e| {
            eprintln!("Ошибка открытия входного файла '{file_in_path}': {}", e);
            std::process::exit(1);
        }))
    };

    let mut storage = InMemoryOperationStorage::new();
    storage.load(format_in, file_in)?;
    storage.save(format_out, &io::stdout())?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
