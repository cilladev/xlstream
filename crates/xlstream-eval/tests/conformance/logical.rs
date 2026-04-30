use super::conformance::run_conformance;

#[test]
fn if_ifs_switch() {
    run_conformance("logical/if_ifs_switch.xlsx");
}

#[test]
fn iferror_ifna() {
    run_conformance("logical/iferror_ifna.xlsx");
}

#[test]
fn boolean_logic() {
    run_conformance("logical/boolean_logic.xlsx");
}
