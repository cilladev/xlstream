use super::conformance::run_conformance;

#[test]
fn hex2dec_dec2hex() {
    run_conformance("engineering/hex2dec_dec2hex.xlsx");
}

#[test]
fn complex() {
    run_conformance("engineering/complex.xlsx");
}

#[test]
fn bitwise() {
    run_conformance("engineering/bitwise.xlsx");
}

#[test]
fn bin2dec_dec2bin() {
    run_conformance("engineering/bin2dec_dec2bin.xlsx");
}

#[test]
fn oct2dec_dec2oct() {
    run_conformance("engineering/oct2dec_dec2oct.xlsx");
}

#[test]
fn cross_base() {
    run_conformance("engineering/cross_base.xlsx");
}

#[test]
fn base() {
    run_conformance("engineering/base.xlsx");
}

#[test]
fn delta_gestep() {
    run_conformance("engineering/delta_gestep.xlsx");
}

#[test]
fn erf_erfc() {
    run_conformance("engineering/erf_erfc.xlsx");
}
