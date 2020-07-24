mod sys;

pub use crate::sys::*;

use std::fmt;

// pub struct Weebview<'a>(WebView<'a>);

// impl<'a> Weebview<'a> {
//     pub fn new() -> Result<Self, Error> {
//         Ok(Self(WebView::new(true)?))
//     }

//     pub fn navigate(&self, url: &str) -> Result<(), Error> {
//         self.0.navigate(url)
//     }

//     pub fn init(&self, js: &str) -> Result<(), Error> {
//         self.0.init(js)
//     }

//     pub fn eval(&self, js: &str) -> Result<(), Error> {
//         self.0.eval(js)
//     }

//     pub fn bind<F>(&mut self, name: &'a str, fn_: F) -> Result<(), Error>
//         where F: Fn(usize, serde_json::Value) -> usize + 'static
//     {
//         self.0.bind(name, fn_)
//     }

//     pub fn run(self) {
//         self.0.run();
//     }
// }



#[derive(Debug)]
pub enum Error {
    InitError,
    NulError(std::ffi::NulError),
    #[cfg(target_os = "windows")]
    WinrtError(winrt::Error),
    #[cfg(not(target_os = "linux"))]
    OsError(winit::error::OsError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitError => "Fail to initialize instance".fmt(f),
            Error::NulError(e) => e.fmt(f),
            #[cfg(target_os = "windows")]
            Error::WinrtError(e) => format!("{:?}", e).fmt(f),
            #[cfg(not(target_os = "linux"))]
            Error::OsError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(target_os = "windows")]
impl From<winrt::Error> for Error {
    fn from(error: winrt::Error) -> Self {
        Error::WinrtError(error)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

impl From<winit::error::OsError> for Error {
    fn from(error: winit::error::OsError) -> Self {
        Error::OsError(error)
    }
}