use error::custom_error::MyError;
use std::error::Error;
use thiserror::Error;

// This test needs a type that look like this:
// pub enum MyError<F: Error> {
//     JustWrong,
//     WrongWithSource(F)
// }
// The `F` type is an error type defined by rust or other libs.
// The `MyError` type should derive trait `Error`, `Display` & `Debug`

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
