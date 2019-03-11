#![feature(proc_macro_hygiene, stmt_expr_attributes)]

#[remain::sorted]
pub enum TestEnum {
    A,
    B,
    C,
    D,
}

#[test]
fn test_match() {
    let value = TestEnum::A;

    #[remain::sorted]
    let _ = match value {
        TestEnum::A => {}
        TestEnum::B => {}
        TestEnum::C => {}
        _ => {}
    };
}

#[test]
fn test_let() {
    let value = TestEnum::A;

    #[remain::sorted]
    match value {
        TestEnum::A => {}
        TestEnum::B => {}
        TestEnum::C => {}
        _ => {}
    }
}
