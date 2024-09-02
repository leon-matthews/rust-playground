
use super::*;

#[test]
fn greeting_contains_name() {
    let result = greeting("Leon");
    assert!(
        result.contains("Leon"),
        "Greeting does not contain name: {result:?}"
    );
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_can_not_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };
    assert!(!smaller.can_hold(&larger));
}
