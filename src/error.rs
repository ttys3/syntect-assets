use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] ::std::io::Error),
    #[error(transparent)]
    SyntectError(#[from] ::syntect::Error),
    #[error(transparent)]
    SyntectLoadingError(#[from] ::syntect::LoadingError),
    #[error(transparent)]
    ParseIntError(#[from] ::std::num::ParseIntError),
    #[error(transparent)]
    GlobParsingError(#[from] ::globset::Error),
    #[error(transparent)]
    SerdeYamlError(#[from] ::serde_yaml::Error),
    #[error("unable to detect syntax for {0}")]
    UndetectedSyntax(String),
    #[error("unknown syntax: '{0}'")]
    UnknownSyntax(String),
    #[error("Unknown style '{0}'")]
    UnknownStyle(String),
    #[error("Use of bat as a pager is disallowed in order to avoid infinite recursion problems")]
    InvalidPagerValueBat,
    #[error("{0}")]
    Msg(String),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Msg(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
