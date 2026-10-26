use std::{env, fs, io};

use ypbank::{
    InMemoryOperationStorage,
    error::LibError,
    model::{Format, OperationStorage},
};

fn run() -> Result<(), LibError> {
    let args: Vec<String> = env::args().collect();

    let mut storage = InMemoryOperationStorage::new();
    match args.len() {
        4 => {
            let stdin = io::stdin().lock();
            let format_in: Format = args[1].parse()?;
            let mut file_out = fs::File::create(&args[2]).unwrap();
            let format_out: Format = args[3].parse()?;

            storage.load(format_in, stdin)?;
            storage.save(format_out, &mut file_out)?;
        }
        5 => {
            let file_in = fs::File::open(&args[1]).unwrap();
            let format_in: Format = args[2].parse()?;
            let mut file_out = fs::File::create(&args[3]).unwrap();
            let format_out: Format = args[4].parse()?;

            storage.load(format_in, file_in)?;
            storage.save(format_out, &mut file_out)?;
        }
        _ => {
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
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
