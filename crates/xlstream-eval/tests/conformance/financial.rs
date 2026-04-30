use super::conformance::run_conformance;

#[test]
fn pmt_pv_fv_rate() {
    run_conformance("financial/basic_financial.xlsx");
}

#[test]
fn npv() {
    run_conformance("financial/npv.xlsx");
}
