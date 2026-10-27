use std::{env, fs, path::Path};

use ypbank::{
    error::{CompareError, LibError},
    model::{Format, InMemoryOperationStorage, OperationStorage},
};

fn show_usage() {
    eprintln!("Использование:");
    eprintln!("  cargo run --bin comparer -- <path/to/file1> <path/to/file2>");
    eprintln!("Поддерживаемые форматы:");
    eprintln!("  - text");
    eprintln!("  - csv");
    eprintln!("  - bin");
}

fn open_with_format(path: &str) -> (fs::File, Format) {
    let format = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap()
        .parse::<Format>()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

    let file = fs::File::open(path).unwrap_or_else(|e| {
        eprintln!("Ошибка открытия файла '{path}': {}", e);
        std::process::exit(1);
    });

    (file, format)
}

fn run() -> Result<(), LibError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        show_usage();
        std::process::exit(1);
    }

    let (file1, format1) = open_with_format(&args[1]);
    let (file2, format2) = open_with_format(&args[2]);

    let mut storage1 = InMemoryOperationStorage::new();
    storage1.load(format1, file1)?;

    let mut storage2 = InMemoryOperationStorage::new();
    storage2.load(format2, file2)?;

    if let Err(e) = storage1.compare(&storage2) {
        match e {
            CompareError::DifferentFieldValues {
                pos,
                field,
                this,
                other,
            } => {
                println!(
                    "Транзакции отличаются на позиции {}: {}: {} != {}",
                    pos, field, this, other
                );
            }
            CompareError::DifferentLength { this, other } => {
                println!(
                    "Списки транзакций не совпадают по длинне: {} != {}",
                    this, other
                );
            }
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
