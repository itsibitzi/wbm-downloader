use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid date")]
    InvalidDate,
    #[error("Output directory does not exist")]
    OutputDirectoryDoesNotExist,
    #[error("From date is after until date")]
    DateRangeBackwards,
}
