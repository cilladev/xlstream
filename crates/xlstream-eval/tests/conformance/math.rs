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

#[test]
fn combin_combina() {
    run_conformance("math/combin.xlsx");
}

#[test]
fn fact_factdouble() {
    run_conformance("math/fact.xlsx");
}

#[test]
fn even_odd() {
    run_conformance("math/even_odd.xlsx");
}

#[test]
fn trunc() {
    run_conformance("math/trunc.xlsx");
}

#[test]
fn mround() {
    run_conformance("math/mround.xlsx");
}

#[test]
fn ceiling_floor_variants() {
    run_conformance("math/ceiling_floor_variants.xlsx");
}

#[test]
fn gcd_lcm() {
    run_conformance("math/gcd_lcm.xlsx");
}

#[test]
fn roman_arabic() {
    run_conformance("math/roman_arabic.xlsx");
}

#[test]
fn inverse_hyperbolic() {
    run_conformance("math/inverse_hyperbolic.xlsx");
}

#[test]
fn hyperbolic() {
    run_conformance("math/hyperbolic.xlsx");
}

#[test]
fn reciprocal_trig() {
    run_conformance("math/reciprocal_trig.xlsx");
}

#[test]
fn degrees_radians() {
    run_conformance("math/degrees_radians.xlsx");
}

#[test]
fn subtotal_aggregate() {
    run_conformance("math/subtotal_aggregate.xlsx");
}
