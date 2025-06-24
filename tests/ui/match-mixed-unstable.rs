#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use remain::sorted;

fn main() {
    let value = "trivial_regex";

    #[sorted]
    match value {
        "cognitive_complexity" => {}
        "hello world" => {}
        "implicit_hasher" => {}
        "inefficient_to_string" => {}
        "integer_division" => {}
        "large_digit_groups" => {}
        let_it_be if false => {}
        "let_unit_value" => {}
        "manual_map" => {}
        "match_bool" => {}
        mixed_in if false => {}
        "needless_pass_by_value" => {}
        "new_ret_no_self" => {}
        "nonstandard_macro_braces" => {}
        "option_if_let_else" => {}
        "option_option" => {}
        "rc_buffer" => {}
        "string_lit_as_bytes" => {}
        "trivial_regex" => {}
        "useless_let_if_seq" => {}
        "trivially_copy_pass_by_ref" => {}
        "unnested_or_patterns" => {}
        "unreadable_literal" => {}
        "unsafe_vector_initialization" => {}
        _ => {}
    }
}
