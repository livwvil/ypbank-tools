use std::{
    fmt,
    io::{Read, Write},
    str::FromStr,
};

use crate::error::{CompareError, LibError, ParseFormatTypeError, ParseOperationTypeError};

/// Формат файла с транзакциями
#[derive(Debug)]
pub enum Format {
    Text,
    Csv,
    Bin,
}

impl FromStr for Format {
    type Err = ParseFormatTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "txt" => Ok(Format::Text),
            "csv" => Ok(Format::Csv),
            "bin" => Ok(Format::Bin),
            s => Err(ParseFormatTypeError(s.into())),
        }
    }
}

/// Тип транзакции
#[derive(Debug, PartialEq)]
pub enum OperationType {
    Income,
    Expense,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperationType::Income => "income",
            OperationType::Expense => "expense",
        };
        write!(f, "{s}")
    }
}

impl FromStr for OperationType {
    type Err = ParseOperationTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(OperationType::Income),
            "expense" => Ok(OperationType::Expense),
            s => Err(ParseOperationTypeError(s.into())),
        }
    }
}

/// Представление транзакции
#[derive(Debug, PartialEq)]
pub struct Operation {
    date: String,
    category: String,
    operation_type: OperationType,
    amount: u64,
}

impl Operation {
    /// Создаёт транзакцию
    pub fn new(date: String, category: String, operation_type: OperationType, amount: u64) -> Self {
        Operation {
            date,
            category,
            operation_type,
            amount,
        }
    }

    /// Возвращает значение поля `date`
    pub fn get_date(&self) -> &String {
        &self.date
    }

    /// Возвращает значение поля `category`
    pub fn get_category(&self) -> &String {
        &self.category
    }

    /// Возвращает значение поля `operation_type`
    pub fn get_operation_type(&self) -> &OperationType {
        &self.operation_type
    }

    /// Возвращает значение поля `amount`
    pub fn get_amount(&self) -> u64 {
        self.amount
    }
}

/// Хранилище транзакций
pub trait OperationStorage {
    /// Загружает транзакции из источника
    fn load(&mut self, format: Format, source: impl Read) -> Result<(), LibError>;
    /// Сохраняет транзакции в приёмник
    fn save(&self, format: Format, dst: impl Write) -> Result<(), LibError>;
    /// Возвращает все транзакции хранилища
    fn get_all(&self) -> Vec<&Operation>;
    /// Сравнивает два хранилища транзакций
    fn compare(&self, other: &impl OperationStorage) -> Result<(), CompareError>;
}

/// Реализация хранилища транзакций в памяти
pub struct InMemoryOperationStorage {
    operations: Vec<Operation>,
}

impl InMemoryOperationStorage {
    /// Создаёт пустое хранилище транзакций
    pub fn new() -> Self {
        Self { operations: vec![] }
    }
}

macro_rules! nameof {
    ($ty:ident . $field:ident) => {{
        let _ = |x: &$ty| {
            let _ = &x.$field;
        };
        stringify!($field)
    }};
}

impl OperationStorage for InMemoryOperationStorage {
    fn load(&mut self, format: Format, source: impl Read) -> Result<(), LibError> {
        let operations = match format {
            Format::Text => crate::text_format::read(source),
            Format::Csv => crate::csv_format::read(source),
            Format::Bin => crate::bin_format::read(source),
        }?;
        self.operations.extend(operations);
        Ok(())
    }

    fn save(&self, format: Format, dst: impl Write) -> Result<(), LibError> {
        match format {
            Format::Text => crate::text_format::write(&self.operations, dst),
            Format::Csv => crate::csv_format::write(&self.operations, dst),
            Format::Bin => crate::bin_format::write(&self.operations, dst),
        }?;
        Ok(())
    }

    fn get_all(&self) -> Vec<&Operation> {
        self.operations.iter().map(|op| op).collect()
    }

    fn compare(&self, other: &impl OperationStorage) -> Result<(), CompareError> {
        let left = self.get_all();
        let right = other.get_all();

        if left.len() != right.len() {
            return Err(CompareError::DifferentLength {
                this: left.len(),
                other: right.len(),
            });
        }

        for (pos, (&l, r)) in left.iter().zip(right).enumerate() {
            if l.date != r.date {
                return Err(CompareError::DifferentFieldValues {
                    pos,
                    field: nameof!(Operation.date),
                    this: format!("'{}'", l.date),
                    other: format!("'{}'", r.date),
                });
            }
            if l.category != r.category {
                return Err(CompareError::DifferentFieldValues {
                    pos,
                    field: nameof!(Operation.category),
                    this: format!("'{}'", l.category),
                    other: format!("'{}'", r.category),
                });
            }
            if l.operation_type != r.operation_type {
                return Err(CompareError::DifferentFieldValues {
                    pos,
                    field: nameof!(Operation.operation_type),
                    this: format!("'{}'", l.operation_type.to_string()),
                    other: format!("'{}'", r.operation_type.to_string()),
                });
            }
            if l.amount != r.amount {
                return Err(CompareError::DifferentFieldValues {
                    pos,
                    field: nameof!(Operation.amount),
                    this: format!("{}", l.amount),
                    other: format!("{}", r.amount),
                });
            }
        }

        Ok(())
    }
}
