#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParserError(#[from] nom::error::Error<String>),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("None returned")]
    None
}

pub type Result<T> = std::result::Result<T, Error>;
