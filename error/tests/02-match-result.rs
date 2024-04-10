use error::basic;

// Match the Result, and do something

fn main() {
    let ok: Result<i32, String> = Ok(41);
    let e: Result<i32, String> = Err("Error".into());

    assert_eq!(42, basic::match_result(ok));
    assert_eq!(0, basic::match_result(e));
}
