use super::conformance::run_conformance;

#[test]
fn date_year_month_day_weekday() {
    run_conformance("date/date_parts.xlsx");
}

#[test]
fn edate_eomonth_datedif() {
    run_conformance("date/date_math.xlsx");
}

#[test]
fn networkdays_workday() {
    run_conformance("date/workdays.xlsx");
}
