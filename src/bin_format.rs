use std::io::{self, Read, Write};
use std::mem::size_of_val;

use crate::error::LibError;
use crate::model::Operation;

pub fn read(mut source: impl Read) -> Result<Vec<Operation>, LibError> {
    let mut result: Vec<Operation> = vec![];

    let mut first_item = true;
    loop {
        let mut size_buf = [0u8; size_of::<usize>()];

        // date
        let date_len = match source.read_exact(&mut size_buf) {
            Ok(()) => usize::from_le_bytes(size_buf),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                if first_item {
                    return Err(LibError::EmptyInput);
                }
                break;
            }
            Err(e) => return Err(e.into()),
        };

        let mut buf = vec![0u8; date_len];
        let date = match source.read_exact(&mut buf) {
            Ok(()) => {
                String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            }
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        //category
        let category_len = match source.read_exact(&mut size_buf) {
            Ok(()) => usize::from_le_bytes(size_buf),
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        let mut buf = vec![0u8; category_len];
        let category = match source.read_exact(&mut buf) {
            Ok(()) => {
                String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            }
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        //operation_type
        let operation_type_len = match source.read_exact(&mut size_buf) {
            Ok(()) => usize::from_le_bytes(size_buf),
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        let mut buf = vec![0u8; operation_type_len];
        let operation_type = match source.read_exact(&mut buf) {
            Ok(()) => String::from_utf8(buf)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                .parse()?,
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        //operation_type
        let mut buf = [0u8; 8];
        let amount = match source.read_exact(&mut buf) {
            Ok(()) => u64::from_le_bytes(buf),
            Err(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => return Err(e.into()),
            Err(_) => return Err(LibError::BadContent),
        };

        result.push(Operation::new(date, category, operation_type, amount));
        first_item = false;
    }

    Ok(result)
}

pub fn write<'a>(
    operations: impl IntoIterator<Item = &'a Operation>,
    dst: &mut impl Write,
) -> Result<(), LibError> {
    for operation in operations.into_iter() {
        dst.write_all(&operation.get_date().as_bytes().len().to_le_bytes())?;
        dst.write_all(operation.get_date().as_bytes())?;

        dst.write_all(&operation.get_category().as_bytes().len().to_le_bytes())?;
        dst.write_all(operation.get_category().as_bytes())?;

        dst.write_all(
            &operation
                .get_operation_type()
                .to_string()
                .as_bytes()
                .len()
                .to_le_bytes(),
        )?;
        dst.write_all(operation.get_operation_type().to_string().as_bytes())?;

        dst.write_all(&operation.get_amount().to_le_bytes())?;
    }
    dst.flush()?;
    Ok(())
}
