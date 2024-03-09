#![allow(unused)]
// std::borrow::Borrow & BorrowMut example: https://doc.rust-lang.org/1.59.0/core/borrow/trait.BorrowMut.html

// Define a function called "check" that takes a parameter of any type that implements Borrow<str>
fn check<T: std::borrow::Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn check_mut_borrow<T: std::borrow::BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}

// Test function for String type
#[test]
fn test_string_borrow() {
    let s = "Hello".to_string();
    check(s);
}

// Test function for &str type
#[test]
fn test_str_borrow() {
    let s = "Hello";
    check(s);
}

// Test mutable borrow:
#[test]
fn test_mut_borrow() {
    let v = vec![1, 2, 3];
    check_mut_borrow(v);
}