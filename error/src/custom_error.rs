use std::error::Error;

pub enum MyError<F: Error> {
    JustWrong,
    WrongWithSource(F),
}
