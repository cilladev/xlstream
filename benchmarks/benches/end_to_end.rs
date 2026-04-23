#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use criterion::{criterion_group, criterion_main, Criterion};
use rust_xlsxwriter::{Formula, Workbook};
use std::path::Path;
use tempfile::TempDir;
use xlstream_eval::evaluate;

fn generate_bench_fixture(path: &Path, n_rows: usize) {
    let mut wb = Workbook::new();

    // Lookup sheet: 100 rows, 3 columns (ID, Region, Rate)
    let lookup = wb.add_worksheet();
    lookup.set_name("Lookup1").unwrap();
    lookup.write_string(0, 0, "ID").unwrap();
    lookup.write_string(0, 1, "Region").unwrap();
    lookup.write_string(0, 2, "Rate").unwrap();
    let regions = ["EMEA", "APAC", "AMER", "LATAM"];
    for i in 0u32..100 {
        let row = i + 1;
        lookup.write_number(row, 0, f64::from(row)).unwrap();
        lookup.write_string(row, 1, regions[(i as usize) % 4]).unwrap();
        lookup.write_number(row, 2, f64::from(row) * 0.01).unwrap();
    }

    // Main sheet
    let main = wb.add_worksheet();
    main.set_name("Main").unwrap();

    let headers = ["A", "B", "C", "D", "E", "SumAB", "Product", "Lookup"];
    for (c, h) in headers.iter().enumerate() {
        main.write_string(0, c as u16, *h).unwrap();
    }

    for i in 0..n_rows {
        let row = (i + 1) as u32;
        let excel_row = row + 1;
        let a = (i % 1000 + 1) as f64;
        let b = (i % 500) as f64 * 0.1;

        main.write_number(row, 0, a).unwrap();
        main.write_number(row, 1, b).unwrap();
        main.write_number(row, 2, (i % 100 + 1) as f64).unwrap();
        main.write_string(row, 3, regions[i % 4]).unwrap();
        main.write_number(row, 4, (i * 7 % 1000) as f64).unwrap();

        let f_sum = format!("=A{excel_row}+B{excel_row}");
        main.write_formula(row, 5, Formula::new(&f_sum).set_result("0")).unwrap();

        let f_prod = format!("=A{excel_row}*B{excel_row}");
        main.write_formula(row, 6, Formula::new(&f_prod).set_result("0")).unwrap();

        let f_lookup = format!("=VLOOKUP(C{excel_row},Lookup1!A:C,2,FALSE)");
        main.write_formula(row, 7, Formula::new(&f_lookup).set_result("0")).unwrap();
    }

    wb.save(path).unwrap();
}

fn bench_end_to_end(c: &mut Criterion) {
    let dir = TempDir::new().unwrap();
    let input_path = dir.path().join("bench_e2e_input.xlsx");
    let output_path = dir.path().join("bench_e2e_output.xlsx");

    generate_bench_fixture(&input_path, 10_000);

    let mut group = c.benchmark_group("end_to_end");
    group.sample_size(10);
    group.bench_function("evaluate_10k_rows", |b| {
        b.iter(|| {
            // Single-threaded for deterministic benchmarking
            evaluate(&input_path, &output_path, Some(1)).unwrap();
        });
    });
    group.finish();
}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
