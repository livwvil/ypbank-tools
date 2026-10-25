use std::io::{BufRead, BufReader, Read, Write};

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
                return Err(LibError::Format);
            };
            let Some("category") = parts.next() else {
                return Err(LibError::Format);
            };
            let Some("kind") = parts.next() else {
                return Err(LibError::Format);
            };
            let Some("amount") = parts.next() else {
                return Err(LibError::Format);
            };
            let None = parts.next() else {
                return Err(LibError::Format);
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
            return Err(LibError::Format);
        }
    }

    Ok(result)
}

pub fn write(
    _operations: &impl IntoIterator<Item = Operation>,
    _dst: impl Write,
) -> Result<(), LibError> {
    todo!()
}
