pub enum AppError{
    IoError(String),
    DecodeError(String),
    InvalidCommand,
    InvalidInput(String),
    KeyNotFound(String),
    InternalError(String),
}