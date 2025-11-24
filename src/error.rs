#[derive(Debug)]
pub enum CalcError {
    InvalidChar(char),
    InvalidToken(String),
    ParseError(String),
    DivideByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    ArityMismatch { name: String, expected: usize, found: usize },
}
