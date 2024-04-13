fn _sometimes_you_dont_have_to_unwrap() {
    // when you want to do nothing if is's wrong or empty
    let e: Option<()> = None;
    e.map(|_x| {
        // do something here
    });
}

fn _flatten() {
    let e: Option<i32> = Some(42);
    let list = vec![e];
    // you can do this:
    let _ret = list.iter().filter(|x| x.is_some()).map(|x| x.unwrap());
    // but this will be better:
    let _anothor_ret = list.iter().flatten();
}
