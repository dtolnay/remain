#![allow(dead_code)]
#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::extra_unused_type_parameters,
    clippy::let_underscore_untyped,
    clippy::let_unit_value,
    clippy::match_same_arms
)]

#[remain::sorted]
#[derive(PartialEq)]
pub enum TestEnum {
    A,
    B,
    #[remain::unsorted]
    Ignored,
    C,
    #[unsorted]
    AlsoIgnored,
    D,
    __Nonexhaustive,
}

#[remain::sorted]
#[derive(PartialEq)]
pub struct TestStruct {
    a: usize,
    b: usize,
    #[unsorted]
    ignored: usize,
    c: usize,
    #[remain::unsorted]
    also_ignored: usize,
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
#[remain::check]
fn test_let() {
    let value = TestEnum::A;

    #[sorted]
    let () = match value {
        TestEnum::A => {}
        #[remain::unsorted]
        TestEnum::Ignored => {}
        TestEnum::B => {}
        #[unsorted]
        TestEnum::AlsoIgnored => {}
        TestEnum::C => {}
        _ => {}
    };
}

#[test]
#[remain::check]
fn test_match() {
    let value = TestEnum::A;

    #[sorted]
    match value {
        TestEnum::A => {}
        TestEnum::B => {}
        #[unsorted]
        TestEnum::Ignored => {}
        TestEnum::C => {}
        #[remain::unsorted]
        TestEnum::AlsoIgnored => {}
        _ => {}
    }
}
