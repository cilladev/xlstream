use super::conformance::run_conformance;

#[test]
fn stdev_var() {
    run_conformance("statistical/stdev_var.xlsx");
}

#[test]
fn skew_kurt() {
    run_conformance("statistical/skew_kurt.xlsx");
}

#[test]
fn avedev() {
    run_conformance("statistical/avedev.xlsx");
}

#[test]
fn mode_sngl() {
    run_conformance("statistical/mode_sngl.xlsx");
}

#[test]
fn percentile() {
    run_conformance("statistical/percentile.xlsx");
}

#[test]
fn quartile() {
    run_conformance("statistical/quartile.xlsx");
}

#[test]
fn large_small() {
    run_conformance("statistical/large_small.xlsx");
}

#[test]
fn rank() {
    run_conformance("statistical/rank.xlsx");
}

#[test]
fn expon_dist() {
    run_conformance("statistical/expon_dist.xlsx");
}
