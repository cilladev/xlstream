#![allow(clippy::cast_precision_loss)]

use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_financial(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);

    // Row for PMT/FV/RATE: rate, nper, pv, fv
    let tvm_row = vec![
        Value::Number(0.05),      // A1: annual rate
        Value::Number(360.0),     // B1: nper (30yr mortgage)
        Value::Number(200_000.0), // C1: present value
        Value::Number(0.0),       // D1: future value
    ];

    // Row for IRR/NPV: cash flows as individual cells
    let cf_row = vec![
        Value::Number(-1000.0), // A1: initial investment
        Value::Number(200.0),   // B1
        Value::Number(300.0),   // C1
        Value::Number(400.0),   // D1
        Value::Number(500.0),   // E1
        Value::Number(600.0),   // F1
    ];

    let mut group = c.benchmark_group("financial");

    let tvm_formulas: &[(&str, &str)] = &[
        ("pmt", "PMT(A1/12,B1,C1)"),
        ("fv", "FV(A1/12,B1,-1000)"),
        ("rate", "RATE(B1,-1073.64,C1)"),
    ];
    for (name, formula) in tvm_formulas {
        let ast = parse(formula).unwrap();
        group.bench_function(*name, |b| {
            b.iter(|| {
                let scope = RowScope::new(&tvm_row, 1);
                interp.eval(ast.root(), &scope)
            });
        });
    }

    let cf_formulas: &[(&str, &str)] =
        &[("npv", "NPV(0.1,A1,B1,C1,D1,E1,F1)"), ("irr", "IRR(A1,B1,C1,D1,E1,F1)")];
    for (name, formula) in cf_formulas {
        let ast = parse(formula).unwrap();
        group.bench_function(*name, |b| {
            b.iter(|| {
                let scope = RowScope::new(&cf_row, 1);
                interp.eval(ast.root(), &scope)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_financial);
criterion_main!(benches);
