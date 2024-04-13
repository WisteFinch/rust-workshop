use std::{error::{self, Error}, fmt};

#[derive(Debug, thiserror::Error)]
// Complete this struct defination to build your own error type
pub enum MyError<F: Error> {
    #[error("just wrong")]
    JustWrong,
    #[error("wrong with source: {0}")]
    WrongWithSource(
        #[from]
        F
    ),
}

// impl<F> fmt::Display for MyError<F> 
// where
//     F: Error
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::JustWrong => write!(f, "just wrong"),
//             Self::WrongWithSource(e) => write!(f, "wrong with source: {e}"),
//         }
//     }
// }

// impl<F> Error for MyError<F> 
// where
//     F: Error + 'static 
// {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             Self::JustWrong => None,
//             Self::WrongWithSource(e) => Some(e),
//         }
//     }
// }

// impl<F> From<F> for MyError<F>
// where
//     F: Error
// {
//     fn from(value: F) -> Self {
//         Self::WrongWithSource(value)
//     }
// }