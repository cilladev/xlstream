use super::conformance::run_conformance;

#[test]
fn sum_count_counta_countblank_average_min_max_median_product() {
    run_conformance("aggregate/basic_aggregates.xlsx");
}

#[test]
fn sumif_countif_averageif() {
    run_conformance("aggregate/sumif.xlsx");
}

#[test]
fn sumifs_countifs_averageifs() {
    run_conformance("aggregate/sumifs.xlsx");
}

#[test]
fn minifs_maxifs() {
    run_conformance("aggregate/minifs_maxifs.xlsx");
}

#[test]
fn sumproduct() {
    run_conformance("aggregate/sumproduct.xlsx");
}
