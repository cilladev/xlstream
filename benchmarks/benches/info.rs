use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_info(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Empty,             // A1 (blank)
        Value::Number(42.0),      // B1
        Value::Text("hi".into()), // C1
        Value::Bool(true),        // D1
    ];

    let formulas: &[(&str, &str)] = &[
        ("isblank", "ISBLANK(A1)"),
        ("isnumber", "ISNUMBER(B1)"),
        ("type", "TYPE(B1)"),
        ("istext", "ISTEXT(C1)"),
        ("iserror", "ISERROR(A1)"),
        ("row_ref", "ROW(A5)"),
        ("column_ref", "COLUMN(C3)"),
        ("row_no_arg", "ROW()"),
        ("column_no_arg", "COLUMN()"),
        ("rows_range", "ROWS(A1:A10)"),
        ("columns_range", "COLUMNS(A1:C5)"),
    ];

    let mut group = c.benchmark_group("info");
    for (name, formula) in formulas {
        let ast = parse(formula).unwrap();
        group.bench_function(*name, |b| {
            b.iter(|| {
                let scope = RowScope::new(&row, 1);
                interp.eval(ast.root(), &scope)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_info);
criterion_main!(benches);
