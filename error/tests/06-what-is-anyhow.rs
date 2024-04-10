use error::questionmark;
use thiserror::Error;

#[derive(Error, Debug)]
enum ForeignError {
    #[error("Just Wrong")]
    JustWrong,
}

fn main() {
    let _ = questionmark::questionmark(Err(ForeignError::JustWrong));
}
