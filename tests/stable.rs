#[remain::sorted]
pub enum TestEnum {
    A,
    B,
    C,
}

#[test]
#[remain::check]
fn test_match() {
    let value = TestEnum::A;

    #[sorted]
    let _ = match value {
        TestEnum::A => {}
        TestEnum::B => {}
        TestEnum::C => {}
    };
}

#[test]
#[remain::check]
fn test_let() {
    let value = TestEnum::A;

    #[sorted]
    match value {
        TestEnum::A => {}
        TestEnum::B => {}
        TestEnum::C => {}
    }
}
