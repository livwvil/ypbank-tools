use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::error::LibError;
use crate::model::{Operation, OperationType};

/// Читает транзакции `Operation` из `impl Read` в формате `txt`
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

/// Записывает транзакции `Operation` в `impl Write` в формате `txt`
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
