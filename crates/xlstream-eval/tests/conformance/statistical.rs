use super::conformance::run_conformance;

#[test]
fn stdev_var() {
    run_conformance("statistical/stdev_var.xlsx");
}

#[test]
fn skew_kurt() {
    run_conformance("statistical/skew_kurt.xlsx");
}
