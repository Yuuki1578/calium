#[test]
#[allow(unused)]
fn macro_test() {
    let scanned = scan!("Hello World!");
    let scanned = scan!("Hello", "World");
}
