use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_date(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Number(45000.0), // A1: date serial (~2023-03-15)
        Value::Number(45090.0), // B1: date serial (~2023-06-13)
        Value::Number(3.0),     // C1: months offset
    ];

    let formulas: &[(&str, &str)] = &[
        ("year", "YEAR(A1)"),
        ("edate", "EDATE(A1,C1)"),
        ("networkdays", "NETWORKDAYS(A1,B1)"),
        ("datedif", "DATEDIF(A1,B1,\"D\")"),
    ];

    let mut group = c.benchmark_group("date");
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

criterion_group!(benches, bench_date);
criterion_main!(benches);
