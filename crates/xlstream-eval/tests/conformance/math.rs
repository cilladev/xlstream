use super::conformance::run_conformance;

#[test]
fn round_roundup_rounddown_int() {
    run_conformance("math/round.xlsx");
}

#[test]
fn mod_abs_sign_sqrt_power() {
    run_conformance("math/arithmetic_math.xlsx");
}

#[test]
fn ceiling_floor() {
    run_conformance("math/ceiling_floor.xlsx");
}

#[test]
fn ln_log_exp() {
    run_conformance("math/logarithms.xlsx");
}

#[test]
fn trigonometry() {
    run_conformance("math/trigonometry.xlsx");
}
