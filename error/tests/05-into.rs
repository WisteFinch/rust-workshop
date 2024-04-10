use std::{
    error::Error,
    fmt::{write, Display},
};

use error::custom_error::MyError;

#[derive(Debug, PartialEq, Eq)]
enum ForeignError {
    This,
}

impl Display for ForeignError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("Message"))
    }
}

impl Error for ForeignError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::This => None,
        }
    }
}

fn helper() -> ForeignError {
    ForeignError::This
}

fn something() -> Result<(), MyError<ForeignError>> {
    Err(helper().into())
}

fn main() {}
