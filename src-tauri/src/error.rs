pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CtxFail,

    XValueNotOfType(&'static str),

    XPropertyNotFound(String),

    StoreFailToCreate(String),

    JsonSerde(serde_json::Error),

    ModqlOperatorNotSupported(String),

    IO(std::io::Error),

    Reqwest(reqwest::Error),
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Error::JsonSerde(val)
    }
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}

impl From<reqwest::Error> for Error {
    fn from(val: reqwest::Error) -> Self {
        Error::Reqwest(val)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
