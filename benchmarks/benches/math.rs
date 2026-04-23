use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_math(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Number(123.456), // A1
        Value::Number(7.0),     // B1
        Value::Number(144.0),   // C1
    ];

    let formulas: &[(&str, &str)] = &[
        ("round", "ROUND(A1,2)"),
        ("mod", "MOD(A1,B1)"),
        ("sqrt", "SQRT(C1)"),
        ("abs", "ABS(-A1)"),
        ("int", "INT(A1)"),
        ("power", "POWER(A1,2)"),
    ];

    let mut group = c.benchmark_group("math");
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

criterion_group!(benches, bench_math);
criterion_main!(benches);
