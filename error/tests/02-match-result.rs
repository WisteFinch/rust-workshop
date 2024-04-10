use error::basic::match_result;

fn main() {
    let ok: Result<i32, String> = Ok(41);
    let e: Result<i32, String> = Err("Error".into());

    assert_eq!(42, match_result(ok));
    assert_eq!(0, match_result(e));
}