use error::maybe;

fn main() {
    let e = maybe::convert(None);
    assert!(e.is_err())
}
