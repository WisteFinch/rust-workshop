use error::basic;

// This test looks for a function that unwraps a `Result` and return the inner value.
// You can use `unwrap()` provided by rust stdlib

fn main() {
    let ok: Result<String, String> = Ok("Ok".into());

    let answer = basic::simply_unwrap(ok);
    assert_eq!(answer, "Ok");
}
