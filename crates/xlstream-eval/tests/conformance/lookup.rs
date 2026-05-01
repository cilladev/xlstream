use super::conformance::run_conformance;

#[test]
fn vlookup() {
    run_conformance("lookup/vlookup.xlsx");
}

#[test]
fn xlookup() {
    run_conformance("lookup/xlookup.xlsx");
}

#[test]
fn index_match_xmatch() {
    run_conformance("lookup/index_match.xlsx");
}

#[test]
fn hlookup() {
    run_conformance("lookup/hlookup.xlsx");
}

#[test]
fn choose() {
    run_conformance("lookup/choose.xlsx");
}

#[test]
fn table_references() {
    run_conformance("lookup/table_references.xlsx");
}
