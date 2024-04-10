use error::basic::simply_unwrap;

fn main() {
    let ok: Result<String, String> = Ok("Ok".into());

    let answer = simply_unwrap(ok);
    assert_eq!(answer, "Ok");
}
