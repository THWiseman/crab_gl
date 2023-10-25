
pub enum Error {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    CustomError(String)
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

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::CustomError(error)
    }
}