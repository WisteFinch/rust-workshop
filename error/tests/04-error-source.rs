use std::{
    error::Error,
    fmt::{write, Display},
};

use error::custom_error::MyError;

// This test is almost the same as the last, 
// but this time you should be able to convert a foreign error type into your type
// using trait `From<T>`

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

fn main() {
    let e = MyError::WrongWithSource(ForeignError::This);
    let _ = e.source();
}
