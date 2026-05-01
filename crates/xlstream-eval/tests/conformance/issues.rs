// Issue-specific conformance fixtures go here.
// Each test points at a fixture in fixtures/issues/.

#[test]
fn issue_76_self_referential_formulas() {
    super::conformance::run_conformance("issues/issue-76-self-referential-formulas.xlsx");
}

#[test]
fn issue_76_self_referential_max_iter_1() {
    let options = xlstream_eval::EvaluateOptions {
        max_iterations: 1,
        ..xlstream_eval::EvaluateOptions::default()
    };
    super::conformance::run_conformance_with_options(
        "issues/issue-76-self-ref-max-iter-1.xlsx",
        &options,
    );
}
