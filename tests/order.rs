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

#[remain::sorted]
enum NumberingSimple {
    E1,
    E9,
    E10,
}

#[remain::sorted]
enum NumberingComplex {
    E1_Aaa,
    E9_Aaa,
    E10_Aaa,
}
