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

#[test]
fn poisson_dist() {
    run_conformance("statistical/poisson_dist.xlsx");
}

#[test]
fn t_dist() {
    run_conformance("statistical/t_dist.xlsx");
}

#[test]
fn binom_dist() {
    run_conformance("statistical/binom_dist.xlsx");
}

#[test]
fn norm_dist() {
    run_conformance("statistical/norm_dist.xlsx");
}

#[test]
fn norm_s_dist() {
    run_conformance("statistical/norm_s_dist.xlsx");
}

#[test]
fn correl() {
    run_conformance("statistical/correl.xlsx");
}

#[test]
fn covariance() {
    run_conformance("statistical/covariance.xlsx");
}

#[test]
fn slope_intercept_rsq() {
    run_conformance("statistical/slope_intercept_rsq.xlsx");
}

#[test]
fn forecast_linear() {
    run_conformance("statistical/forecast_linear.xlsx");
}

#[test]
fn permut_permutationa() {
    run_conformance("statistical/permut.xlsx");
}
