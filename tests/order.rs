#![allow(dead_code, non_camel_case_types)]
#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]

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

#[remain::sorted]
enum AtomOrder {
    A,
    A_,
    A0,
    AA,
    Aa,
    under_0core,
    under_Score,
    under_score,
    under__0core,
    under__Score,
    under__score,
    underscore,
}

#[remain::sorted]
enum LargeNumber {
    E1,
    E99999999999999999999999,
}
