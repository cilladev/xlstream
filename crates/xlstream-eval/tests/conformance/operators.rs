use super::conformance::run_conformance;

#[test]
fn arithmetic_operators() {
    run_conformance("operators/arithmetic.xlsx");
}

#[test]
fn comparison_operators() {
    run_conformance("operators/comparison.xlsx");
}

#[test]
fn concatenation_operator() {
    run_conformance("operators/concatenation.xlsx");
}
