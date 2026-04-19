//! Deterministic benchmark fixture generator.
//!
//! Generates xlsx workbooks with seeded data for reproducible benchmarks.
//!
//! Usage:
//!   generate-fixtures --tier small|medium|large|all [--output-dir DIR]

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::similar_names)]

use std::path::{Path, PathBuf};

use rust_xlsxwriter::{Formula, Workbook, Worksheet};

// ---------------------------------------------------------------------------
// Seeded RNG (LCG — no external crate)
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 =
            self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// Uniform f64 in `[lo, hi)`.
    fn f64_range(&mut self, lo: f64, hi: f64) -> f64 {
        let t = (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64);
        lo + t * (hi - lo)
    }

    /// Uniform integer in `[lo, hi]` (inclusive).
    fn int_range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }

    /// Random string of length in `[min_len, max_len]` from the given alphabet.
    fn text(&mut self, alphabet: &[u8], min_len: usize, max_len: usize) -> String {
        let len = self.int_range(min_len as u64, max_len as u64) as usize;
        (0..len)
            .map(|_| alphabet[self.int_range(0, alphabet.len() as u64 - 1) as usize] as char)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tier definitions
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Tier {
    Small,
    Medium,
    Large,
}

impl Tier {
    fn rows(self) -> u32 {
        match self {
            Self::Small => 10_000,
            Self::Medium => 100_000,
            Self::Large => 1_000_000,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const REGIONS: &[&str] = &["EMEA", "APAC", "AMER", "LATAM"];
const LOOKUP_REGIONS: &[&str] = &["East", "West", "North", "South"];
const CATEGORIES: &[&str] = &["Electronics", "Clothing", "Food", "Tools", "Books"];
const ALPHABET: &[u8] = b"abcdefghij";

// ---------------------------------------------------------------------------
// Lookup sheets
// ---------------------------------------------------------------------------

fn write_lookup1(ws: &mut Worksheet, rng: &mut Rng) {
    ws.set_name("Lookup1").expect("set_name Lookup1");
    for r in 0u32..1000 {
        ws.write_number(r, 0, f64::from(r + 1)).expect("L1 A");
        ws.write_string(r, 1, LOOKUP_REGIONS[(r as usize) % LOOKUP_REGIONS.len()]).expect("L1 B");
        ws.write_number(r, 2, rng.f64_range(0.01, 0.99)).expect("L1 C");
        ws.write_string(r, 3, format!("key_{:03}", r + 1)).expect("L1 D");
    }
}

fn write_lookup2(ws: &mut Worksheet, rng: &mut Rng) {
    ws.set_name("Lookup2").expect("set_name Lookup2");
    for r in 0u32..10_000 {
        ws.write_string(r, 0, format!("P{:04}", r + 1)).expect("L2 A");
        ws.write_string(r, 1, CATEGORIES[(r as usize) % CATEGORIES.len()]).expect("L2 B");
        ws.write_number(r, 2, rng.f64_range(1.0, 999.99)).expect("L2 C");
    }
}

fn write_holidays(ws: &mut Worksheet) {
    ws.set_name("Holidays").expect("set_name Holidays");
    for r in 0u32..30 {
        let serial = 45292.0 + f64::from(r) * (364.0 / 29.0);
        ws.write_number(r, 0, serial.round()).expect("Holidays A");
    }
}

// ---------------------------------------------------------------------------
// Main sheet data row
// ---------------------------------------------------------------------------

fn write_data_row(ws: &mut Worksheet, row: u32, rng: &mut Rng) {
    ws.write_number(row, 0, f64::from(row + 1)).expect("A");
    ws.write_number(row, 1, rng.f64_range(0.0, 1000.0)).expect("B");
    ws.write_number(row, 2, rng.f64_range(0.0, 1000.0)).expect("C");
    ws.write_number(row, 3, rng.f64_range(1.0, 100.0)).expect("D");
    ws.write_number(row, 4, rng.f64_range(0.01, 999.99)).expect("E");
    ws.write_number(row, 5, rng.int_range(1, 50) as f64).expect("F");
    ws.write_number(row, 6, rng.f64_range(1.0, 100.0)).expect("G");
    ws.write_number(row, 7, rng.f64_range(1.0, 100.0)).expect("H");
    ws.write_number(row, 8, rng.f64_range(1.0, 10.0)).expect("I");
    ws.write_number(row, 9, rng.f64_range(-100.0, 100.0)).expect("J");
    ws.write_number(row, 10, rng.int_range(1, 100) as f64).expect("K");
    let txt_l = rng.text(ALPHABET, 3, 8);
    ws.write_string(row, 11, &txt_l).expect("L");
    let txt_m = rng.text(ALPHABET, 3, 8);
    ws.write_string(row, 12, &txt_m).expect("M");
    ws.write_number(row, 13, rng.f64_range(0.0, 100.0)).expect("N");
    ws.write_number(row, 14, rng.f64_range(0.0, 100.0)).expect("O");
    ws.write_number(row, 15, rng.int_range(1, 10) as f64).expect("P");
    ws.write_number(row, 16, rng.int_range(1, 10) as f64).expect("Q");
    ws.write_string(row, 17, REGIONS[(row as usize) % REGIONS.len()]).expect("R");
    ws.write_number(row, 18, rng.f64_range(45292.0, 46023.0).round()).expect("S");
    ws.write_boolean(row, 19, row % 2 == 0).expect("T");
}

// ---------------------------------------------------------------------------
// Main sheet formula row
// ---------------------------------------------------------------------------

fn write_formula_row(ws: &mut Worksheet, row: u32) {
    let r = row + 1; // 1-based Excel row number
    let formulas: [String; 30] = [
        format!("=A{r}+B{r}"),
        format!("=C{r}-D{r}"),
        format!("=E{r}*F{r}"),
        format!("=G{r}/H{r}"),
        format!("=I{r}^2"),
        format!("=-J{r}"),
        format!("=K{r}%"),
        format!("=L{r}&M{r}"),
        format!("=N{r}>O{r}"),
        format!("=P{r}=Q{r}"),
        format!("=IF(A{r}>5000,B{r},C{r})"),
        format!("=IFS(A{r}>7500,\"P\",A{r}>5000,\"G\",TRUE,\"B\")"),
        format!("=AND(A{r}>0,B{r}>0)"),
        format!("=IFERROR(G{r}/H{r},0)"),
        format!("=B{r}/SUM(B:B)*100"),
        "=AVERAGE(C:C)".to_string(),
        "=SUMIF(R:R,\"EMEA\",B:B)".to_string(),
        "=COUNTIF(B:B,\">500\")".to_string(),
        format!("=VLOOKUP(MOD(A{r},1000)+1,Lookup1!A:D,2,FALSE)"),
        format!("=INDEX(Lookup1!C:C,MATCH(MOD(A{r},1000)+1,Lookup1!A:A,0))"),
        format!("=LEFT(L{r},3)"),
        format!("=UPPER(M{r})"),
        format!("=ROUND(E{r},2)"),
        format!("=MOD(F{r},G{r})"),
        format!("=YEAR(S{r})"),
        format!("=EDATE(S{r},3)"),
        format!("=ISNUMBER(A{r})"),
        format!("=TYPE(B{r})"),
        format!("=TEXT(E{r},\"0.00\")"),
        "=VALUE(\"123\")".to_string(),
    ];

    for (i, f) in formulas.iter().enumerate() {
        let col = 20 + i as u16;
        ws.write_formula(row, col, Formula::new(f).set_result("0")).expect("formula");
    }
}

// ---------------------------------------------------------------------------
// Workbook generation
// ---------------------------------------------------------------------------

fn generate(tier: Tier, output_dir: &Path) {
    let n_rows = tier.rows();
    let path = output_dir.join(format!("bench_{}.xlsx", tier.name()));
    eprintln!("generating {} ({n_rows} rows)...", tier.name());

    let mut wb = Workbook::new();
    let mut rng = Rng::new(42);

    write_lookup1(wb.add_worksheet(), &mut rng);
    write_lookup2(wb.add_worksheet(), &mut rng);
    write_holidays(wb.add_worksheet());

    let ws = wb.add_worksheet();
    ws.set_name("Main").expect("set_name Main");
    for row in 0..n_rows {
        write_data_row(ws, row, &mut rng);
        write_formula_row(ws, row);
    }

    wb.save(&path).expect("save workbook");
    eprintln!("done: {}", path.display());
}

// ---------------------------------------------------------------------------
// CLI arg parsing
// ---------------------------------------------------------------------------

fn parse_args() -> (Vec<Tier>, PathBuf) {
    let args: Vec<String> = std::env::args().collect();
    let mut tier_str: Option<String> = None;
    let mut output_dir = PathBuf::from("benchmarks/fixtures/");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tier" => {
                i += 1;
                tier_str = Some(args.get(i).expect("--tier requires a value").clone());
            }
            "--output-dir" => {
                i += 1;
                output_dir = PathBuf::from(args.get(i).expect("--output-dir requires a value"));
            }
            other => {
                eprintln!("unknown arg: {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let tier_str = tier_str.unwrap_or_else(|| {
        eprintln!("usage: generate-fixtures --tier small|medium|large|all [--output-dir DIR]");
        std::process::exit(1);
    });

    let tiers = match tier_str.as_str() {
        "small" => vec![Tier::Small],
        "medium" => vec![Tier::Medium],
        "large" => vec![Tier::Large],
        "all" => vec![Tier::Small, Tier::Medium, Tier::Large],
        other => {
            eprintln!("unknown tier: {other}");
            std::process::exit(1);
        }
    };

    (tiers, output_dir)
}

fn main() {
    let (tiers, output_dir) = parse_args();
    std::fs::create_dir_all(&output_dir).expect("create output dir");

    for tier in tiers {
        generate(tier, &output_dir);
    }
}
