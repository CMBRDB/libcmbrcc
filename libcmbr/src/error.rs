#![allow(dead_code)]

use std::error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
/// An enum denoting each error libcmbr can give
pub enum LibCmbrErrorType {
    #[default]
    Ok = 0,
    ShouldBeUnreachable,
    CrazyHouseNotSupported,
}

// A struct with libcmbr reports errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LibCmbrError {
    kind: LibCmbrErrorType,
}

impl fmt::Display for LibCmbrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error_string = "[LIBCMBR ERROR] ".to_string();

        error_string.push_str(match self.kind {
            LibCmbrErrorType::ShouldBeUnreachable => "This should be unreachable",
            LibCmbrErrorType::CrazyHouseNotSupported => "Crazyhouse is not supported yet",
            LibCmbrErrorType::Ok => "Ok. (This should be generally unreachable, and if you're seeing this, something probably went very wrong)"
        });

        return write!(f, "{}", error_string);
    }
}

impl error::Error for LibCmbrError {}

impl LibCmbrError {
    pub fn new(kind: LibCmbrErrorType) -> Self {
        return Self { kind };
    }

    pub fn ok() -> Self {
        return Self::default();
    }
}
