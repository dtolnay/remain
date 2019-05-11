#[remain::check]
fn main() {
    let value = 0;

    #[sorted]
    match value {
        0..=20 => {}
        _ => {}
    }
}

#[remain::sorted]
struct TestUnnamedStruct(usize, usize, usize, usize);

#[remain::sorted]
struct TestUnitStruct;
