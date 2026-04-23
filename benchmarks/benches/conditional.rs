use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_conditional(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Number(150.0), // A1
        Value::Number(200.0), // B1
        Value::Number(0.0),   // C1
        Value::Bool(true),    // D1
    ];

    let formulas: &[(&str, &str)] = &[
        ("if", "IF(A1>100,B1,C1)"),
        ("ifs", "IFS(A1>200,\"high\",A1>100,\"mid\",TRUE,\"low\")"),
        ("switch", "SWITCH(A1,100,\"exact\",150,\"match\",\"none\")"),
        ("iferror", "IFERROR(B1/C1,0)"),
        ("and", "AND(A1>0,B1>0,D1)"),
        ("or", "OR(A1>200,B1>200,C1>0)"),
    ];

    let mut group = c.benchmark_group("conditional");
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

criterion_group!(benches, bench_conditional);
criterion_main!(benches);
