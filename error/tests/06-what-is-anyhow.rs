use error::questionmark;
use thiserror::Error;

// This test is prrtty simple.
// It looks for a function that returns an `anyhow::Result`

#[derive(Error, Debug)]
enum ForeignError {
    #[error("Just Wrong")]
    JustWrong,
}

fn main() {
    let _ = questionmark::questionmark(Err(ForeignError::JustWrong));
}
