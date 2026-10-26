use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::error::LibError;
use crate::model::Operation;

pub fn read(source: impl Read) -> Result<Vec<Operation>, LibError> {
    let reader = BufReader::new(source);
    let mut lines_iterator = reader.lines();
    let mut result: Vec<Operation> = vec![];

    let mut first_line = true;
    loop {
        let date = match lines_iterator.next() {
            Some(r) => r?.to_string(),
            None => {
                if first_line {
                    return Err(LibError::EmptyInput);
                }
                break;
            }
        };
        let category = match lines_iterator.next() {
            Some(r) => r?.to_string(),
            None => return Err(LibError::BadContent),
        };
        let operation_type = match lines_iterator.next() {
            Some(r) => r?.parse()?,
            None => return Err(LibError::BadContent),
        };
        let amount = match lines_iterator.next() {
            Some(r) => r?.parse()?,
            None => return Err(LibError::BadContent),
        };

        result.push(Operation::new(date, category, operation_type, amount));
        first_line = false;
    }

    Ok(result)
}

pub fn write<'a>(
    operations: impl IntoIterator<Item = &'a Operation>,
    dst: &mut impl Write,
) -> Result<(), LibError> {
    let mut writer = BufWriter::new(dst);
    for operation in operations.into_iter() {
        writeln!(writer, "{}", operation.get_date())?;
        writeln!(writer, "{}", operation.get_category())?;
        writeln!(writer, "{}", operation.get_operation_type().to_string())?;
        writeln!(writer, "{}", operation.get_amount())?;
    }
    writer.flush()?;
    Ok(())
}
