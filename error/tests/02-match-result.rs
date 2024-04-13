use error::basic;

// This test looks for a function that takes a `Result<i32, _>`.
// The return value will be:
// - Ok(number) => number + 1
// - Err(_) => 0

fn main() {
    let ok: Result<i32, String> = Ok(41);
    let e: Result<i32, String> = Err("Error".into());

    assert_eq!(42, basic::match_result(ok));
    assert_eq!(0, basic::match_result(e));
}
