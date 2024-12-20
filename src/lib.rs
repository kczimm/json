pub mod error;
pub mod parser;
pub mod tokenizer;
pub mod value;

pub type Result<T> = std::result::Result<T, error::JsonError>;
