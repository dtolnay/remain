#![allow(dead_code, non_camel_case_types)]

#[remain::sorted]
enum UnderscoresFirst {
    __Nonexhaustive,
    Aaa,
    Bbb,
}

#[remain::sorted]
enum UnderscoresLast {
    Aaa,
    Bbb,
    __Nonexhaustive,
}

#[remain::sorted]
enum SnakeCase {
    under_score,
    underscore,
}
