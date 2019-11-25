#![allow(dead_code)]
#![cfg(not(remain_stable_testing))]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]

#[remain::sorted]
#[derive(PartialEq)]
pub enum TestEnum {
    A,
    #[remain::unsorted]
    Ignored,
    B,
    C,
    D,
    __Nonexhaustive,
}

#[remain::sorted]
#[derive(PartialEq)]
pub struct TestStruct {
    a: usize,
    b: usize,
    c: usize,
    #[unsorted]
    ignored: usize,
    d: usize,
}

#[test]
fn test_attrs() {
    fn is_partial_eq<T: PartialEq>() -> bool {
        true
    }

    assert!(is_partial_eq::<TestEnum>());
    assert!(is_partial_eq::<TestStruct>());
}

#[test]
fn test_let() {
    let value = TestEnum::A;

    #[remain::sorted]
    let _ = match value {
        TestEnum::A => {}
        #[remain::unsorted]
        TestEnum::Ignored => {}
        TestEnum::B => {}
        TestEnum::C => {}
        _ => {}
    };
}

#[test]
fn test_match() {
    let value = TestEnum::A;

    #[remain::sorted]
    match value {
        TestEnum::A => {}
        TestEnum::B => {}
        #[unsorted]
        TestEnum::Ignored => {}
        TestEnum::C => {}
        _ => {}
    }
}
