
use testing;

mod common;


#[test]
fn test_greeting() {
    common::setup();
    let hello = testing::greeting("Hello");
    assert_eq!(hello, "Hello Hello");
}
