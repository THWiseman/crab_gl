
#[derive(Error, Debug)]
enum Error {
    #[error("I/O error: {0}")]
    IoError(std::io::Error),

    #[error("Parsing error: {0}")]
    ParseError(std::num::ParseIntError),

    #[error("Custom error: {0}")]
    CustomError(string)
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseError(error)
    }
}