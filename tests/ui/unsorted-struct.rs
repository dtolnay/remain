use remain::sorted;

#[sorted]
struct TestStruct {
    d: usize,
    #[unsorted]
    c: usize,
    a: usize,
    b: usize,
}

fn main() {}
