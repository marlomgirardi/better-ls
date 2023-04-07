use colored::Colorize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

#[derive(Debug)]
#[non_exhaustive]
pub enum BetterLsError {
    NotFound(String),
    Unauthorized(String),
    Unknown(io::Error),
}

// Display requires manual implementation
impl Display for BetterLsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use BetterLsError::*;
        let msg = match self {
            NotFound(context) => format!("Entry not found: {}", context),
            Unauthorized(context) => format!("Unauthorized access: {}", context),
            Unknown(ref error) => format!("Unknown error! {}", error),
        };

        write!(f, "{}", msg.red())
    }
}

impl Error for BetterLsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use BetterLsError::*;
        match self {
            NotFound(_) => None,
            Unauthorized(_) => None,
            Unknown(ref error) => Some(error),
        }
    }
}

/// io::Error is not that relevant on its own for us, a generic mapping does best.
/// And can be reused in all places instead of unwrapping even if the error is unlikely.
pub fn exhaustive_io_error_mapping(err: io::Error, context: String) -> BetterLsError {
    use BetterLsError::*;
    match err.kind() {
        io::ErrorKind::NotFound => NotFound(context),
        io::ErrorKind::PermissionDenied => Unauthorized(context),
        _ => Unknown(err),
    }
}
