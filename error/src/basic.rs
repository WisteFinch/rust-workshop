use std::fmt::Debug;

// Maybe this is not the best way, this is the easiest way to deal with a Result
pub fn simply_unwrap<T, E: Debug>(result: Result<T, E>) -> T {
    result.unwrap()
}

// Handle the error in your way
pub fn match_result<E>(result: Result<i32, E>) -> i32 {
    match result {
        Ok(value) => value + 1,
        Err(_) => 0
    }
}
