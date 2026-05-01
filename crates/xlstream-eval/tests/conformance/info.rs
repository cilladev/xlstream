use super::conformance::run_conformance;

#[test]
fn isblank_isnumber_istext_islogical_isnontext_iserror_isna_isref_type_na() {
    run_conformance("info/type_checks.xlsx");
}

#[test]
fn rows_columns() {
    run_conformance("info/rows_columns.xlsx");
}
