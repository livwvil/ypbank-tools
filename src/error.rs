use std::fmt;

#[derive(Debug)]
pub struct ParseFormatTypeError(pub String);

impl fmt::Display for ParseFormatTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неподдерживаемый формат файла: '{}'", self.0)
    }
}

impl std::error::Error for ParseFormatTypeError {}

#[derive(Debug)]
pub struct ParseOperationTypeError(pub String);

impl fmt::Display for ParseOperationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неизвестный статус: {}", self.0)
    }
}

impl std::error::Error for ParseOperationTypeError {}

#[derive(Debug)]
pub enum CompareError {
    DifferentLength {
        this: usize,
        other: usize,
    },
    DifferentFieldValues {
        pos: usize,
        field: &'static str,
        this: String,
        other: String,
    },
}

#[derive(Debug)]
pub enum LibError {
    EmptyInput,
    Format,
    Io(std::io::Error),
    ParseAmount(std::num::ParseIntError),
    ParseOperationType(ParseOperationTypeError),
}

impl From<std::io::Error> for LibError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::num::ParseIntError> for LibError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseAmount(err)
    }
}

impl From<ParseOperationTypeError> for LibError {
    fn from(err: ParseOperationTypeError) -> Self {
        Self::ParseOperationType(err)
    }
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for LibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LibError::Io(error) => Some(error),
            LibError::ParseAmount(error) => Some(error),
            _ => None,
        }
    }
}
