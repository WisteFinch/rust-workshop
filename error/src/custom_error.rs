use std::error::Error;

// Complete this struct defination to build your own error type
pub enum MyError<F: Error> {
    JustWrong,
    WrongWithSource(F),
}
