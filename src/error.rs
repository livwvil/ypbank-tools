use std::fmt;

/// Ошибка парсинга типа формата
#[derive(Debug)]
pub struct ParseFormatTypeError(pub String);

impl fmt::Display for ParseFormatTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неподдерживаемый формат файла: '{}'", self.0)
    }
}

impl std::error::Error for ParseFormatTypeError {}

/// Ошибка парсинга типа операции
#[derive(Debug)]
pub struct ParseOperationTypeError(pub String);

impl fmt::Display for ParseOperationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "неизвестный статус: {}", self.0)
    }
}

impl std::error::Error for ParseOperationTypeError {}

/// Ошибка сравнения списков транзакций
#[derive(Debug)]
pub enum CompareError {
    /// Списки не совпадают по количеству транзакций
    DifferentLength { this: usize, other: usize },
    /// Обнаружено различие в значении поля транзакции
    DifferentFieldValues {
        pos: usize,
        field: &'static str,
        this: String,
        other: String,
    },
}

/// Ошибка чтения\записи транзакций из\в файл
#[derive(Debug)]
pub enum LibError {
    /// Пустой файл
    EmptyInput,
    /// Контент файла не соответствует формату
    BadContent,
    /// Ошибка парсинга формата файла
    BadFormat(ParseFormatTypeError),
    /// Ошибка ввода вывода при чтении\записи файла
    Io(std::io::Error),
    /// Ошибка парсинга суммы
    ParseAmount(std::num::ParseIntError),
    /// Ошибка парсинга типа операции
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

impl From<ParseFormatTypeError> for LibError {
    fn from(err: ParseFormatTypeError) -> Self {
        Self::BadFormat(err)
    }
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LibError::EmptyInput => write!(f, "Не удалось прочитать файл - пустой"),
            LibError::BadContent => write!(f, "Содержимое файла не соответствует формату"),
            LibError::BadFormat(e) => write!(f, "Ошибка парсинга формата файла: {e}"),
            LibError::Io(e) => write!(f, "Ошибка ввода-вывода: {e}"),
            LibError::ParseAmount(e) => write!(f, "Ошибка парсинга суммы: {e}"),
            LibError::ParseOperationType(e) => write!(f, "Ошибка парсинга типа операции: {e}"),
        }
    }
}

impl std::error::Error for LibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LibError::BadFormat(error) => Some(error),
            LibError::Io(error) => Some(error),
            LibError::ParseAmount(error) => Some(error),
            LibError::ParseOperationType(error) => Some(error),
            _ => None,
        }
    }
}
