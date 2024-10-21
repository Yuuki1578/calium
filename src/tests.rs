#[test]
#[allow(unused)]
fn macro_test() {
    let list = string!("Hello", "World!");
    let s = string!("Hello World!");

    let tok = scan!("Hello World");
}
