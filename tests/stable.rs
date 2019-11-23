#![allow(dead_code)]

#[remain::sorted]
pub enum TestEnum {
    A,
    B,
    #[remain::unsorted]
    Ignored,
    C,
    #[unsorted]
    AlsoIgnored,
    D,
}

#[remain::sorted]
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
#[remain::check]
fn test_let() {
    let value = TestEnum::A;

    #[sorted]
    let _ = match value {
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
