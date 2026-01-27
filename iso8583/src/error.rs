use alloc::string::String;
use core::panic::Location;

#[derive(Debug)]
pub enum ErrorKind {
    InsufficientLength { need: usize, got: usize },
}

pub struct Error {
    pub kind: ErrorKind,
    pub reason: String,
    pub location: &'static Location<'static>,
}

impl Error {
    #[track_caller]
    pub fn new(kind: ErrorKind) -> Self {
        Self::with_reason(kind, "")
    }

    #[track_caller]
    pub fn with_reason<T: Into<String>>(
        kind: ErrorKind,
        reason: T,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
            location: Location::caller(),
        }
    }
}

impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(f, "{self}")?;

        Ok(())
    }
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        if self.reason.is_empty() {
            write!(f, "{:?}", self.kind)?;
        } else {
            write!(f, "{:?}: {}", self.kind, self.reason)?;
        }

        write!(f, " (at {}:{})", self.location.file(), self.location.line())?;

        Ok(())
    }
}
