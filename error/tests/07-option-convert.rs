use error::maybe;

// This test looks for a function that converts a `Option` into a `Result`

fn main() {
    let e = maybe::convert(None);
    assert!(e.is_err())
}
