pub enum MyError {
    JustWrong
}

// You can turn an Option into a Result
pub fn convert(maybe: Option<()>) -> Result<(), MyError> {
    maybe.ok_or(MyError::JustWrong)
}
