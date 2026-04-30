use super::conformance::run_conformance;

#[test]
fn left_right_mid_len() {
    run_conformance("text/substring.xlsx");
}

#[test]
fn upper_lower_proper_trim_clean() {
    run_conformance("text/case.xlsx");
}

#[test]
fn concat_concatenate_textjoin() {
    run_conformance("text/concat.xlsx");
}

#[test]
fn find_search_substitute_replace() {
    run_conformance("text/find_search.xlsx");
}

#[test]
fn text_value_exact() {
    run_conformance("text/text_value.xlsx");
}
