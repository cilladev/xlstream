#![allow(clippy::cast_precision_loss)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use xlstream_core::Value;
use xlstream_eval::lookup::LookupSheet;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn build_lookup_sheet(n_rows: usize) -> LookupSheet {
    let rows: Vec<Vec<Value>> = (0..n_rows)
        .map(|i| {
            vec![
                Value::Number((i + 1) as f64),
                Value::Text(format!("region_{}", i % 4).into()),
                Value::Number((i as f64) * 0.01),
            ]
        })
        .collect();
    let mut sheet = LookupSheet::new(rows);
    sheet.build_col_index(0);
    sheet
}

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup");

    for size in [1_000, 10_000] {
        let sheet = build_lookup_sheet(size);
        let mut sheets = HashMap::new();
        sheets.insert("lookup1".to_string(), sheet);
        let prelude = Prelude::empty().with_lookup_sheets(sheets);
        let interp = Interpreter::new(&prelude);
        let ast = parse("VLOOKUP(A1,Lookup1!A:C,2,FALSE)").unwrap();

        group.bench_with_input(BenchmarkId::new("vlookup_exact", size), &size, |b, _| {
            let row = vec![Value::Number(500.0)];
            b.iter(|| {
                let scope = RowScope::new(&row, 1);
                interp.eval(ast.root(), &scope)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
