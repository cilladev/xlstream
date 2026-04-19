use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_arithmetic(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![Value::Number(42.5), Value::Number(17.3)];

    let ops: &[(&str, &str)] = &[
        ("add", "A1+B1"),
        ("subtract", "A1-B1"),
        ("multiply", "A1*B1"),
        ("divide", "A1/B1"),
        ("power", "A1^2"),
        ("negate", "-A1"),
        ("percent", "A1%"),
        ("concat", "A1&B1"),
        ("compare_gt", "A1>B1"),
        ("compare_eq", "A1=B1"),
    ];

    let mut group = c.benchmark_group("arithmetic");
    for (name, formula) in ops {
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

criterion_group!(benches, bench_arithmetic);
criterion_main!(benches);
