use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::error::LibError;
use crate::model::{Operation, OperationType};

/// Читает транзакции `Operation` из `impl Read` в формате `text`
pub fn read(source: impl Read) -> Result<Vec<Operation>, LibError> {
    let reader = BufReader::new(source);

    let mut result: Vec<Operation> = vec![];

    for res in reader.lines() {
        let line = res?;

        let parts: Vec<&str> = line.trim().split(';').collect();
        if parts.len() == 4 {
            let date = parts[0].to_string();
            let category = parts[1].to_string();
            let operation_type: OperationType = parts[2].parse()?;
            let amount: u64 = parts[3].parse()?;

            result.push(Operation::new(date, category, operation_type, amount));
        } else {
            return Err(LibError::BadContent);
        }
    }

    Ok(result)
}

/// Записывает транзакции `Operation` в `impl Write` в формате `text`
pub fn write<'a>(
    operations: impl IntoIterator<Item = &'a Operation>,
    dst: impl Write,
) -> Result<(), LibError> {
    let mut writer = BufWriter::new(dst);
    for operation in operations.into_iter() {
        writeln!(
            writer,
            "{};{};{};{}",
            operation.get_date(),
            operation.get_category(),
            operation.get_operation_type().to_string(),
            operation.get_amount()
        )?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::model::OperationType;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_text_round_trip_success() {
        let op = Operation::new(
            "2026-04-01".into(),
            "salary".into(),
            OperationType::Income,
            120000,
        );

        let mut cursor = Cursor::new(Vec::new());
        write(&vec![op.clone()], &mut cursor).unwrap();
        cursor.set_position(0);
        let res = read(cursor).unwrap();

        assert_eq!(vec![op], res);
    }

    #[test]
    fn test_text_parse_error() {
        let data = b"2026-04-01,salary,income,120000\n";
        let cursor = Cursor::new(&data[..]);

        assert!(read(cursor).is_err());
    }
}
