use super::conformance::run_conformance;

#[test]
fn hex2dec_dec2hex() {
    run_conformance("engineering/hex2dec_dec2hex.xlsx");
}

#[test]
fn complex() {
    run_conformance("engineering/complex.xlsx");
}
