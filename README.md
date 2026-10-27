Онлайн-курс Яндекс Практикум `Rust для действующих разработчиков: переход с Python, С++ или C` 25.10.2026.

Проектная работа модуля 1. Чтение, парсинг и анализ данных в Rust.

# ypbank-tools

## Запуск

```sh
cargo run --bin comparer -- path/to/file1 path/to/file2

# Пример
cargo run --bin comparer -- ./test_files/transactions.csv ./test_files/transactions.txt
cargo run --bin comparer -- ./test_files/transactions.csv ./test_files/transactions_another.csv
```

```sh
cargo run --bin converter -- path/to/file_in format_in path/to/file_out format_out
cargo run --bin converter -- format_in path/to/file_out format_out  path/to/file_in

# Пример
cargo run --bin converter -- ./test_files/transactions.csv csv ./test_files/transactions.bin bin
cargo run --bin converter -- txt ./test_files/transactions.bin bin < ./test_files/transactions_another.txt

```

## Устройство бинарного файла

Транзакции хранятся в файле подряд, вплотную. Размер строк ограничен 255 байтами - это осознанный выбор. Ни дата, ни категория, ни тип операции не будут длиннее.

| Размер (байт) | Тип     | Обозначение                 |
| ------------- | ------- | --------------------------- |
| 1             | u8      | Размер `date` - N           |
| N             | UTF-8   | `date`                      |
| 1             | u8      | Размер `category` - M       |
| M             | UTF-8   | `category`                  |
| 1             | u8      | Размер `operation_type` - K |
| K             | UTF-8   | `operation_type`            |
| 8             | u8 (LE) | `amount`                    |
