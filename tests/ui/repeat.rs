enum E {
    Aaa(u8),
    Bbb,
}

#[remain::check]
fn main() {
    #[sorted]
    match E::Bbb {
        E::Aaa(0) => {}
        E::Bbb => {}
        E::Aaa(_) => {}
    }
}
