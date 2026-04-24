//! Golden-file regression test suite.
//!
//! Compares xlstream evaluation output against Excel-computed cached values.
//!
//! ## Workflow
//!
//! 1. Generate the fixture (once, or after adding formulas):
//!    ```sh
//!    cargo test -p xlstream-eval --test regression_base_base -- generate_fixture --ignored --nocapture
//!    ```
//! 2. Open `crates/xlstream-eval/tests/fixtures/regression.xlsx` in Excel.
//! 3. Let formulas compute, then **Save**. This populates cached values.
//! 4. Commit the saved file.
//! 5. Run: `cargo test -p xlstream-eval --test regression_base`
//!
//! The comparison test reads Excel's cached values as ground truth and
//! compares them cell-by-cell against xlstream's output.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::needless_range_loop
)]

use std::path::{Path, PathBuf};

use calamine::{open_workbook, Data, Reader as CalReader, Xlsx};
use rust_xlsxwriter::{Format, Formula, Workbook};
use tempfile::NamedTempFile;
use xlstream_eval::evaluate;

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

fn regression_path() -> PathBuf {
    fixtures_dir().join("regression.xlsx")
}

// ---------------------------------------------------------------------------
// Data constants
// ---------------------------------------------------------------------------

const N_ROWS: usize = 10;
const DATA_COLS: u16 = 8;

const DATA_A: [f64; N_ROWS] = [10.0, 25.0, 0.0, 100.0, -5.0, 50.0, 75.0, 1.0, 999.0, 42.0];
const DATA_B: [f64; N_ROWS] = [3.5, 7.2, 0.0, 15.8, -2.3, 8.1, 4.6, 0.5, 12.4, 6.7];
const DATA_C: [f64; N_ROWS] =
    [1000.0, 2500.0, 0.0, 5000.0, 150.0, 3000.0, 7500.0, 250.0, 4000.0, 1200.0];
const DATA_D: [bool; N_ROWS] = [true, false, true, true, false, false, true, false, true, true];
const DATA_E: [&str; N_ROWS] =
    ["EMEA", "APAC", "AMER", "EMEA", "APAC", "AMER", "EMEA", "APAC", "AMER", "EMEA"];
const DATA_F: [&str; N_ROWS] = [
    "hello",
    "WORLD",
    "  spaces  here  ",
    "abc",
    "short",
    "test",
    "ProPer Case",
    "num123",
    "foo bar",
    "x",
];
const DATA_G: [f64; N_ROWS] =
    [45000.0, 45031.0, 45061.0, 44927.0, 45200.0, 45300.0, 44800.0, 45100.0, 45400.0, 45500.0];

// Column H (mixed): indices 0,3,6,9 are empty; 1,8 are numbers; 2 is text;
// 4 is bool; 5 is zero; 7 is text.
// Written inline in the generator.

// ---------------------------------------------------------------------------
// Formula specifications
// ---------------------------------------------------------------------------

struct Spec {
    header: &'static str,
    template: &'static str,
    volatile: bool,
}

const fn f(header: &'static str, template: &'static str) -> Spec {
    Spec { header, template, volatile: false }
}

const fn fv(header: &'static str, template: &'static str) -> Spec {
    Spec { header, template, volatile: true }
}

#[rustfmt::skip]
const FORMULAS: &[Spec] = &[
    // ---- Operators (15) ----
    f("op_add",    "=A{r}+B{r}"),
    f("op_sub",    "=A{r}-B{r}"),
    f("op_mul",    "=A{r}*B{r}"),
    f("op_div",    "=A{r}/B{r}"),
    f("op_exp",    "=A{r}^2"),
    f("op_cat",    "=E{r}&F{r}"),
    f("op_pct",    "=C{r}%"),
    f("op_eq",     "=(A{r}=B{r})"),
    f("op_neq",    "=(A{r}<>B{r})"),
    f("op_lt",     "=(A{r}<B{r})"),
    f("op_gt",     "=(A{r}>B{r})"),
    f("op_lte",    "=(A{r}<=B{r})"),
    f("op_gte",    "=(A{r}>=B{r})"),
    f("op_neg",    "=-A{r}"),
    f("op_pos",    "=+A{r}"),

    // ---- Logical (11) ----
    f("fn_true",    "=TRUE()"),
    f("fn_false",   "=FALSE()"),
    f("fn_if",      "=IF(A{r}>50,\"high\",\"low\")"),
    f("fn_ifs",     "=IFS(A{r}>100,\"A\",A{r}>50,\"B\",A{r}>0,\"C\",TRUE,\"D\")"),
    f("fn_switch",  "=SWITCH(E{r},\"EMEA\",1,\"APAC\",2,\"AMER\",3,0)"),
    f("fn_iferror", "=IFERROR(A{r}/B{r},-1)"),
    f("fn_ifna",    "=IFNA(VLOOKUP(E{r},Lookup1!A:C,2,FALSE),\"missing\")"),
    f("fn_and",     "=AND(A{r}>0,B{r}>0)"),
    f("fn_or",      "=OR(A{r}>100,B{r}>10)"),
    f("fn_not",     "=NOT(D{r})"),
    f("fn_xor",     "=XOR(A{r}>50,B{r}>5)"),

    // ---- Aggregates (15) ----
    f("agg_sum",      "=SUM(C:C)"),
    f("agg_sumif",    "=SUMIF(AggSource!A:A,\"EMEA\",AggSource!B:B)"),
    f("agg_sumifs",   "=SUMIFS(AggSource!B:B,AggSource!A:A,\"EMEA\",AggSource!C:C,\"Sales\")"),
    f("agg_product",  "=PRODUCT(2,3,4)"),
    f("agg_count",    "=COUNT(C:C)"),
    f("agg_counta",   "=COUNTA(H:H)"),
    f("agg_cntblk",   "=COUNTBLANK(H:H)"),
    f("agg_countif",  "=COUNTIF(AggSource!A:A,\"EMEA\")"),
    f("agg_cntifs",   "=COUNTIFS(AggSource!A:A,\"EMEA\",AggSource!C:C,\"Sales\")"),
    f("agg_avg",      "=AVERAGE(C:C)"),
    f("agg_avgif",    "=AVERAGEIF(AggSource!A:A,\"APAC\",AggSource!B:B)"),
    f("agg_avgifs",   "=AVERAGEIFS(AggSource!B:B,AggSource!A:A,\"EMEA\",AggSource!C:C,\"Sales\")"),
    f("agg_min",      "=MIN(C:C)"),
    f("agg_max",      "=MAX(C:C)"),
    f("agg_median",   "=MEDIAN(C:C)"),

    // ---- Lookup (7) ----
    f("lk_vlookup",   "=VLOOKUP(E{r},Lookup1!A:C,2,FALSE)"),
    f("lk_hlookup",   "=HLOOKUP(\"Q2\",HLookup!A1:D3,2,FALSE)"),
    f("lk_xlookup",   "=XLOOKUP(E{r},Lookup1!A:A,Lookup1!B:B,\"N/F\")"),
    f("lk_match",     "=MATCH(E{r},Lookup1!A:A,0)"),
    f("lk_xmatch",    "=XMATCH(E{r},Lookup1!A:A,0)"),
    f("lk_idx_match", "=INDEX(Lookup1!B:B,MATCH(E{r},Lookup1!A:A,0))"),
    f("lk_choose",    "=CHOOSE(2,\"alpha\",\"beta\",\"gamma\")"),

    // ---- Text (19) ----
    f("tx_left",     "=LEFT(F{r},3)"),
    f("tx_right",    "=RIGHT(F{r},3)"),
    f("tx_mid",      "=MID(F{r},2,3)"),
    f("tx_len",      "=LEN(F{r})"),
    f("tx_upper",    "=UPPER(F{r})"),
    f("tx_lower",    "=LOWER(F{r})"),
    f("tx_proper",   "=PROPER(F{r})"),
    f("tx_trim",     "=TRIM(F{r})"),
    f("tx_clean",    "=CLEAN(F{r})"),
    f("tx_concat",   "=CONCAT(E{r},\"-\",F{r})"),
    f("tx_concat2",  "=CONCATENATE(E{r},\"-\",F{r})"),
    f("tx_textjoin", "=TEXTJOIN(\"-\",TRUE,E{r},F{r})"),
    f("tx_find",     "=IFERROR(FIND(\"l\",F{r}),0)"),
    f("tx_search",   "=IFERROR(SEARCH(\"L\",F{r}),0)"),
    f("tx_subst",    "=SUBSTITUTE(F{r},\"o\",\"0\")"),
    f("tx_replace",  "=REPLACE(F{r},1,1,\"X\")"),
    f("tx_text",     "=TEXT(C{r},\"0.00\")"),
    f("tx_value",    "=VALUE(\"123.45\")"),
    f("tx_exact",    "=EXACT(F{r},\"hello\")"),

    // ---- Math (23) ----
    f("m_round",    "=ROUND(B{r},1)"),
    f("m_roundup",  "=ROUNDUP(B{r},0)"),
    f("m_rounddn",  "=ROUNDDOWN(B{r},0)"),
    f("m_int",      "=INT(B{r})"),
    f("m_mod",      "=MOD(A{r},3)"),
    f("m_abs",      "=ABS(A{r})"),
    f("m_sign",     "=SIGN(A{r})"),
    f("m_sqrt",     "=SQRT(ABS(A{r}))"),
    f("m_power",    "=POWER(ABS(A{r}),2)"),
    f("m_ceiling",  "=CEILING(B{r},1)"),
    f("m_floor",    "=FLOOR(B{r},1)"),
    f("m_ln",       "=LN(ABS(A{r})+1)"),
    f("m_log",      "=LOG(ABS(A{r})+1,10)"),
    f("m_log10",    "=LOG10(ABS(A{r})+1)"),
    f("m_exp",      "=EXP(1)"),
    f("m_sin",      "=SIN(B{r})"),
    f("m_cos",      "=COS(B{r})"),
    f("m_tan",      "=TAN(B{r})"),
    f("m_asin",     "=ASIN(B{r}/100)"),
    f("m_acos",     "=ACOS(B{r}/100)"),
    f("m_atan",     "=ATAN(B{r})"),
    f("m_atan2",    "=ATAN2(A{r},B{r})"),
    f("m_pi",       "=PI()"),

    // ---- Date/Time (12) ----
    fv("dt_today",   "=TODAY()"),
    fv("dt_now",     "=NOW()"),
    f("dt_date",     "=DATE(2026,4,20)"),
    f("dt_year",     "=YEAR(G{r})"),
    f("dt_month",    "=MONTH(G{r})"),
    f("dt_day",      "=DAY(G{r})"),
    f("dt_weekday",  "=WEEKDAY(G{r})"),
    f("dt_edate",    "=EDATE(G{r},3)"),
    f("dt_eomonth",  "=EOMONTH(G{r},0)"),
    f("dt_datedif",  "=DATEDIF(DATE(2020,1,1),G{r},\"d\")"),
    f("dt_netdays",  "=NETWORKDAYS(DATE(2026,1,1),DATE(2026,1,31))"),
    f("dt_workday",  "=WORKDAY(DATE(2026,1,1),10)"),

    // ---- Info (10) ----
    f("inf_isblank",  "=ISBLANK(H{r})"),
    f("inf_isnum",    "=ISNUMBER(A{r})"),
    f("inf_istext",   "=ISTEXT(F{r})"),
    f("inf_islog",    "=ISLOGICAL(D{r})"),
    f("inf_isnontx",  "=ISNONTEXT(A{r})"),
    f("inf_iserr",    "=ISERROR(1/0)"),
    f("inf_isna",     "=ISNA(VLOOKUP(\"ZZZ\",Lookup1!A:C,2,FALSE))"),
    f("inf_isref",    "=ISREF(A{r})"),
    f("inf_na",       "=NA()"),
    f("inf_type",     "=TYPE(A{r})"),

    // ---- Financial (5; IRR skipped — needs range arg) ----
    f("fin_pmt",   "=PMT(0.05/12,360,-200000)"),
    f("fin_pv",    "=PV(0.08/12,120,-500)"),
    f("fin_fv",    "=FV(0.06/12,240,-200)"),
    f("fin_npv",   "=NPV(0.1,A{r},B{r},C{r})"),
    f("fin_rate",  "=RATE(120,-500,50000)"),
];

// ---------------------------------------------------------------------------
// Support sheet builders
// ---------------------------------------------------------------------------

fn write_lookup1(wb: &mut Workbook) {
    let ws = wb.add_worksheet();
    ws.set_name("Lookup1").unwrap();
    let data: &[(&str, &str, f64)] = &[
        ("EMEA", "Europe", 100.0),
        ("APAC", "Asia", 200.0),
        ("AMER", "Americas", 300.0),
        ("LATAM", "LatAm", 50.0),
        ("MEA", "MidEast", 75.0),
    ];
    for (i, &(key, name, val)) in data.iter().enumerate() {
        let r = i as u32;
        ws.write_string(r, 0, key).unwrap();
        ws.write_string(r, 1, name).unwrap();
        ws.write_number(r, 2, val).unwrap();
    }
}

fn write_hlookup(wb: &mut Workbook) {
    let ws = wb.add_worksheet();
    ws.set_name("HLookup").unwrap();
    // Row 0: quarter labels
    for (c, label) in ["Q1", "Q2", "Q3", "Q4"].iter().enumerate() {
        ws.write_string(0, c as u16, *label).unwrap();
    }
    // Row 1: values
    for (c, val) in [100.0, 200.0, 300.0, 400.0].iter().enumerate() {
        ws.write_number(1, c as u16, *val).unwrap();
    }
    // Row 2: secondary values
    for (c, val) in [10.0, 20.0, 30.0, 40.0].iter().enumerate() {
        ws.write_number(2, c as u16, *val).unwrap();
    }
}

fn write_aggsource(wb: &mut Workbook) {
    let ws = wb.add_worksheet();
    ws.set_name("AggSource").unwrap();
    let data: &[(&str, f64, &str)] = &[
        ("EMEA", 500.0, "Sales"),
        ("APAC", 300.0, "Marketing"),
        ("AMER", 700.0, "Sales"),
        ("EMEA", 200.0, "Marketing"),
        ("APAC", 400.0, "Sales"),
        ("EMEA", 600.0, "Sales"),
        ("AMER", 100.0, "Marketing"),
        ("APAC", 800.0, "Sales"),
        ("EMEA", 350.0, "Marketing"),
        ("AMER", 450.0, "Sales"),
    ];
    for (i, &(region, amount, category)) in data.iter().enumerate() {
        let r = i as u32;
        ws.write_string(r, 0, region).unwrap();
        ws.write_number(r, 1, amount).unwrap();
        ws.write_string(r, 2, category).unwrap();
    }
}

fn write_holidays(wb: &mut Workbook) {
    let ws = wb.add_worksheet();
    ws.set_name("Holidays").unwrap();
    let date_fmt = Format::new().set_num_format("yyyy-mm-dd");
    for (i, serial) in [46023.0, 46054.0, 46388.0].iter().enumerate() {
        ws.write_number_with_format(i as u32, 0, *serial, &date_fmt).unwrap();
    }
}

fn write_data_sheet(wb: &mut Workbook) {
    let ws = wb.add_worksheet();
    ws.set_name("Data").unwrap();

    // Headers: data columns
    let headers = ["id", "val", "amt", "flag", "region", "text", "date", "mixed"];
    for (c, h) in headers.iter().enumerate() {
        ws.write_string(0, c as u16, *h).unwrap();
    }
    // Headers: formula columns
    for (i, spec) in FORMULAS.iter().enumerate() {
        ws.write_string(0, DATA_COLS + i as u16, spec.header).unwrap();
    }

    let date_fmt = Format::new().set_num_format("yyyy-mm-dd");

    for row_idx in 0..N_ROWS {
        let row = (row_idx + 1) as u32;
        let excel_row = row + 1;

        // A: id
        ws.write_number(row, 0, DATA_A[row_idx]).unwrap();
        // B: val
        ws.write_number(row, 1, DATA_B[row_idx]).unwrap();
        // C: amt
        ws.write_number(row, 2, DATA_C[row_idx]).unwrap();
        // D: flag
        ws.write_boolean(row, 3, DATA_D[row_idx]).unwrap();
        // E: region
        ws.write_string(row, 4, DATA_E[row_idx]).unwrap();
        // F: text
        ws.write_string(row, 5, DATA_F[row_idx]).unwrap();
        // G: date serial
        ws.write_number_with_format(row, 6, DATA_G[row_idx], &date_fmt).unwrap();
        // H: mixed (some empty, some numbers, some text, some bool)
        match row_idx {
            1 => {
                ws.write_number(row, 7, 42.0).unwrap();
            }
            2 => {
                ws.write_string(row, 7, "text").unwrap();
            }
            4 => {
                ws.write_boolean(row, 7, true).unwrap();
            }
            5 => {
                ws.write_number(row, 7, 0.0).unwrap();
            }
            7 => {
                ws.write_string(row, 7, "error").unwrap();
            }
            8 => {
                ws.write_number(row, 7, 100.0).unwrap();
            }
            _ => {} // 0, 3, 6, 9 left empty
        }

        // Formula columns
        for (i, spec) in FORMULAS.iter().enumerate() {
            let col = DATA_COLS + i as u16;
            let formula_text = spec.template.replace("{r}", &excel_row.to_string());
            ws.write_formula(row, col, Formula::new(&formula_text)).unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// Workbook generator
// ---------------------------------------------------------------------------

fn generate_regression_workbook(path: &Path) {
    let mut wb = Workbook::new();
    write_lookup1(&mut wb);
    write_hlookup(&mut wb);
    write_aggsource(&mut wb);
    write_holidays(&mut wb);
    write_data_sheet(&mut wb);
    wb.save(path).unwrap();
}

// ---------------------------------------------------------------------------
// Comparison helpers
// ---------------------------------------------------------------------------

const EPSILON: f64 = 1e-6;

fn col_letter(col: usize) -> String {
    let mut s = String::new();
    let mut c = col;
    loop {
        s.insert(0, (b'A' + (c % 26) as u8) as char);
        if c < 26 {
            break;
        }
        c = c / 26 - 1;
    }
    s
}

fn is_volatile(col_idx: usize) -> bool {
    if col_idx < DATA_COLS as usize {
        return false;
    }
    let spec_idx = col_idx - DATA_COLS as usize;
    FORMULAS.get(spec_idx).is_some_and(|s| s.volatile)
}

// TODO: remove when calamine merges https://github.com/tafia/calamine/pull/645
fn is_calamine_corrupted(col_idx: usize) -> bool {
    if col_idx < DATA_COLS as usize {
        return false;
    }
    let spec_idx = col_idx - DATA_COLS as usize;
    FORMULAS.get(spec_idx).is_some_and(|s| s.header == "m_log10")
}

fn error_to_string(e: &calamine::CellErrorType) -> &'static str {
    use calamine::CellErrorType;
    match e {
        CellErrorType::Div0 => "#DIV/0!",
        CellErrorType::NA => "#N/A",
        CellErrorType::Name => "#NAME?",
        CellErrorType::Null => "#NULL!",
        CellErrorType::Num => "#NUM!",
        CellErrorType::Ref => "#REF!",
        CellErrorType::Value | CellErrorType::GettingData => "#VALUE!",
    }
}

fn data_as_f64(d: &Data) -> Option<f64> {
    match d {
        Data::Float(f) => Some(*f),
        Data::Int(i) => Some(*i as f64),
        Data::DateTime(dt) => Some(dt.as_f64()),
        _ => None,
    }
}

fn values_match(expected: &Data, actual: &Data) -> bool {
    match (expected, actual) {
        // Numeric (Float, Int, DateTime all compare as f64)
        (a, b) if data_as_f64(a).is_some() && data_as_f64(b).is_some() => {
            let av = data_as_f64(a).unwrap();
            let bv = data_as_f64(b).unwrap();
            (av - bv).abs() < EPSILON || (av == 0.0 && bv == 0.0)
        }
        // Strings
        (Data::String(a), Data::String(b)) => a == b,
        // Booleans
        (Data::Bool(a), Data::Bool(b)) => a == b,
        // Bool vs Int (Excel sometimes stores TRUE as 1), or both empty
        (Data::Bool(true), Data::Int(1))
        | (Data::Int(1), Data::Bool(true))
        | (Data::Bool(false), Data::Int(0))
        | (Data::Int(0), Data::Bool(false))
        | (Data::Empty, Data::Empty) => true,
        // Empty string ≈ Empty (rust_xlsxwriter discards empty strings)
        (Data::String(s), Data::Empty) | (Data::Empty, Data::String(s)) if s.is_empty() => true,
        // Error (Excel) vs String (xlstream writes errors as text)
        (Data::Error(e), Data::String(s)) | (Data::String(s), Data::Error(e)) => {
            s == error_to_string(e)
        }
        // Error vs Error
        (Data::Error(a), Data::Error(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
#[ignore = "run manually to generate the regression fixture for Excel verification"]
fn generate_fixture() {
    let path = regression_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    generate_regression_workbook(&path);
    eprintln!("wrote {}", path.display());
    eprintln!("open in Excel, save (populates cached values), then commit.");
}

#[test]
fn golden_file_regression() {
    let fixture = regression_path();
    if !fixture.exists() {
        eprintln!("skip: fixture not found at {}. Run generate_fixture first.", fixture.display());
        return;
    }

    // Read Excel cached values as ground truth.
    let mut expected_wb: Xlsx<_> = open_workbook(&fixture).unwrap();
    let expected_range = expected_wb.worksheet_range("Data").unwrap();

    // Check if the file has been opened in Excel (cached values populated).
    // Without Excel, rust_xlsxwriter caches all formulas as 0.0.
    // TRUE() can never be 0 — if it is, the file hasn't been Excel-saved.
    let fn_true_idx = FORMULAS.iter().position(|s| s.header == "fn_true").unwrap();
    let fn_true_col = DATA_COLS as usize + fn_true_idx;
    let first_data_row = expected_range.rows().nth(1);
    if let Some(row) = first_data_row {
        if row.len() > fn_true_col && matches!(&row[fn_true_col], Data::Float(f) if *f == 0.0) {
            eprintln!(
                "skip: fixture has default cached values (not Excel-saved). \
                 Open in Excel, save, then re-run."
            );
            return;
        }
    }

    // Evaluate through xlstream.
    let output = NamedTempFile::with_suffix(".xlsx").unwrap();
    evaluate(&fixture, output.path(), None).unwrap();

    // Read xlstream output.
    let mut actual_wb: Xlsx<_> = open_workbook(output.path()).unwrap();
    let actual_range = actual_wb.worksheet_range("Data").unwrap();

    // Compare cell-by-cell (skip header row).
    let mut mismatches = Vec::new();
    let expected_rows: Vec<_> = expected_range.rows().collect();
    let actual_rows: Vec<_> = actual_range.rows().collect();

    let row_count = expected_rows.len().min(actual_rows.len());
    if expected_rows.len() != actual_rows.len() {
        mismatches.push(format!(
            "row count mismatch: expected {} actual {}",
            expected_rows.len(),
            actual_rows.len()
        ));
    }

    for row_idx in 1..row_count {
        let exp_row = expected_rows[row_idx];
        let act_row = actual_rows[row_idx];

        let col_count = exp_row.len().min(act_row.len());
        for col_idx in 0..col_count {
            if is_volatile(col_idx) || is_calamine_corrupted(col_idx) {
                continue;
            }
            if !values_match(&exp_row[col_idx], &act_row[col_idx]) {
                let header = if col_idx < DATA_COLS as usize {
                    ["id", "val", "amt", "flag", "region", "text", "date", "mixed"][col_idx]
                        .to_string()
                } else {
                    let spec_idx = col_idx - DATA_COLS as usize;
                    FORMULAS
                        .get(spec_idx)
                        .map_or_else(|| col_letter(col_idx), |s| s.header.to_string())
                };
                mismatches.push(format!(
                    "  {}{} [{}]: expected {:?}  actual {:?}",
                    col_letter(col_idx),
                    row_idx + 1,
                    header,
                    exp_row[col_idx],
                    act_row[col_idx],
                ));
            }
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} cell mismatches:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}
