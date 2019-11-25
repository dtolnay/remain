use remain::sorted;

#[sorted]
enum E {
    Aaa,
    Ccc(u8),
    #[unsorted]
    Ddd { u: u8 },
    Bbb(u8, u8),
}

fn main() {}
