use std::{env, fs, path::Path};

use ypbank::{
    error::{CompareError, LibError},
    model::{Format, InMemoryOperationStorage, OperationStorage},
};

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

    let file = fs::File::open(path).unwrap_or_else(|_| {
        eprintln!("Файл не найден: {}", path);
        std::process::exit(1);
    });

    (file, format)
}

fn display_error(err: LibError) {
    match err {
        LibError::EmptyInput => {
            eprintln!("Не удалось прочитать файл - пустой");
        }
        LibError::Format => {
            eprintln!("Содержимое файла не соответствует формату");
        }
        LibError::Io(e) => {
            eprintln!("Ошибка ввода-вывода: {}", e);
        }
        LibError::ParseAmount(e) => {
            eprintln!("Ошибка парсинга суммы: {}", e);
        }
        LibError::ParseOperationType(e) => {
            eprintln!("Ошибка парсинга типа операции: {}", e);
        }
    }
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Использование:");
        eprintln!("  cargo run --bin comparer -- path/to/file1 path/to/file2");
        eprintln!("Поддерживаемые форматы:");
        return;
    }

    let (file1, format1) = open_with_format(&args[1]);
    let (file2, format2) = open_with_format(&args[2]);

    let mut storage1 = InMemoryOperationStorage::new();
    storage1.load(format1, file1).unwrap_or_else(display_error);

    let mut storage2 = InMemoryOperationStorage::new();
    storage2.load(format2, file2).unwrap_or_else(display_error);

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
}
