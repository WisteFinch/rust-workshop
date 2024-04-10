use error::custom_error::MyError;
use std::error::Error;
use thiserror::Error;

// Define your own error type

struct Wrapper<E: Error> {
    inner: E,
}

#[derive(Debug, Error)]
enum ForeignError {
    #[error("Just Wrong.")]
    JustWrong,
}

fn main() {
    let ok: Result<(), MyError<ForeignError>> = Ok(());
    let e: Result<(), MyError<ForeignError>> = Err(MyError::JustWrong);
    assert!(ok.is_ok());
    assert!(e.is_err());

    if let Err(e) = e {
        let _ = Wrapper { inner: e };
    }
}
