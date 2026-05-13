use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_statistical(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Number(3.0),   // A1 — x
        Value::Number(5.0),   // B1 — mean
        Value::Number(100.0), // C1 — large mean
    ];

    let formulas: &[(&str, &str)] = &[
        ("poisson_pmf", "POISSON.DIST(A1,B1,FALSE)"),
        ("poisson_cdf", "POISSON.DIST(A1,B1,TRUE)"),
        ("poisson_large_cdf", "POISSON.DIST(C1,C1,TRUE)"),
    ];

    let mut group = c.benchmark_group("statistical");
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

criterion_group!(benches, bench_statistical);
criterion_main!(benches);
