#[cfg(not(target_os = "windows"))]
mod posix;
#[cfg(not(target_os = "windows"))]
pub use posix::Clipboard;
pub type Result<T, E = Error> = std::result::Result<T, E>;

// Implement windows someday, wouldja?

use std::string::FromUtf8Error;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to initialize clipboard")]
    Init,

    #[error("found non-utf8 data on clipboard")]
    BadUtf8,

    #[error("unable to initialize clipboard")]
    X11Init,
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Error::BadUtf8
    }
}

impl From<x11_clipboard::error::Error> for Error {
    fn from(_: x11_clipboard::error::Error) -> Self {
        Error::X11Init
    }
}


/// get clipboard data as a utf8 string
pub fn get() -> Result<String> {
    Clipboard::default().get()
}

/// Fill the clipboard with the given string
pub fn set(text: impl AsRef<str>) -> Result<()> {
    Clipboard::default().set(text)
}
