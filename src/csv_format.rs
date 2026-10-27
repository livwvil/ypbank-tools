use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::error::LibError;
use crate::model::{Operation, OperationType};

pub fn read(source: impl Read) -> Result<Vec<Operation>, LibError> {
    let reader = BufReader::new(source);
    let mut lines_iterator = reader.lines();

    match lines_iterator.next() {
        Some(res) => {
            let first_line = res?;
            let mut parts = first_line.trim().split(',');
            let Some("date") = parts.next() else {
                return Err(LibError::BadContent);
            };
            let Some("category") = parts.next() else {
                return Err(LibError::BadContent);
            };
            let Some("kind") = parts.next() else {
                return Err(LibError::BadContent);
            };
            let Some("amount") = parts.next() else {
                return Err(LibError::BadContent);
            };
            let None = parts.next() else {
                return Err(LibError::BadContent);
            };
        }
        None => {
            return Err(LibError::EmptyInput);
        }
    }

    let mut result: Vec<Operation> = vec![];

    for res in lines_iterator {
        let line = res?;

        let parts: Vec<&str> = line.trim().split(',').collect();
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

pub fn write<'a>(
    operations: impl IntoIterator<Item = &'a Operation>,
    dst: impl Write,
) -> Result<(), LibError> {
    let mut writer = BufWriter::new(dst);
    writeln!(writer, "date,category,kind,amount")?;
    for operation in operations.into_iter() {
        writeln!(
            writer,
            "{},{},{},{}",
            operation.get_date(),
            operation.get_category(),
            operation.get_operation_type().to_string(),
            operation.get_amount()
        )?;
    }
    writer.flush()?;
    Ok(())
}
