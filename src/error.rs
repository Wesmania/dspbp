#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    E(String),
}

impl<T: Into<String>> From<T> for Error {
    fn from(s: T) -> Self {
        Self::E(s.into())
    }
}

#[cfg(feature = "dump")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::E(msg.to_string())
    }
}

#[cfg(feature = "dump")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::E(msg.to_string())
    }
}

pub fn some_error<T: Into<String>>(s: T) -> anyhow::Error {
    Error::from(s).into()
}
