use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_core::Value;
use xlstream_eval::{Interpreter, Prelude, RowScope};
use xlstream_parse::parse;

fn bench_string(c: &mut Criterion) {
    let prelude = Prelude::empty();
    let interp = Interpreter::new(&prelude);
    let row = vec![
        Value::Text("Hello World".into()),   // A1
        Value::Text("fox".into()),           // B1
        Value::Number(12345.6789),           // C1
        Value::Text("one-two-three".into()), // D1
    ];

    let formulas: &[(&str, &str)] = &[
        ("left", "LEFT(A1,5)"),
        ("concat", "CONCATENATE(A1,B1)"),
        ("text", "TEXT(C1,\"#,##0.00\")"),
        ("substitute", "SUBSTITUTE(D1,\"-\",\" \")"),
        ("find", "FIND(\"World\",A1)"),
        ("textjoin", "TEXTJOIN(\"-\",TRUE,A1,B1)"),
    ];

    let mut group = c.benchmark_group("string");
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

criterion_group!(benches, bench_string);
criterion_main!(benches);
