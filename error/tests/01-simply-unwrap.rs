use error::basic;

// Unwrap the Result and return the inner value

fn main() {
    let ok: Result<String, String> = Ok("Ok".into());

    let answer = basic::simply_unwrap(ok);
    assert_eq!(answer, "Ok");
}
