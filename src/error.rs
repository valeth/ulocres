use std::io;


pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid version {0}")]
    InvalidVersion(u8),

    #[error("Invalid localized string index {0}")]
    InvalidLocalizedStringIndex(usize),

    #[error("Failed to parse string")]
    StringParsingFailed(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Io(#[from] io::Error),
}
