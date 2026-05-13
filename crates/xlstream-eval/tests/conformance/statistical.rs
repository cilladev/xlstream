use super::conformance::run_conformance;

#[test]
fn stdev_var() {
    run_conformance("statistical/stdev_var.xlsx");
}
