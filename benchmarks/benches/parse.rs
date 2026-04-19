use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_parse::parse;

const FORMULAS: &[&str] = &[
    "A1+B1",
    "C1-D1",
    "E1*F1",
    "G1/H1",
    "I1^2",
    "-J1",
    "K1%",
    "L1&M1",
    "N1>O1",
    "P1=Q1",
    "IF(A1>0,B1,C1)",
    "IFS(A1>100,\"P\",TRUE,\"B\")",
    "AND(A1>0,B1>0)",
    "IFERROR(G1/H1,0)",
    "SUM(A:A)",
    "AVERAGE(B:B)",
    "SUMIF(R:R,\"EMEA\",B:B)",
    "COUNTIF(B:B,\">500\")",
    "VLOOKUP(A1,Sheet2!A:C,2,FALSE)",
    "INDEX(Sheet2!B:B,MATCH(A1,Sheet2!A:A,0))",
    "LEFT(L1,3)",
    "UPPER(M1)",
    "ROUND(E1,2)",
    "MOD(F1,G1)",
    "YEAR(S1)",
    "EDATE(S1,3)",
    "ISNUMBER(A1)",
    "TYPE(B1)",
    "TEXT(E1,\"0.00\")",
    "VALUE(\"123\")",
];

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse_30_formulas", |b| {
        b.iter(|| {
            for formula in FORMULAS {
                let _ = parse(formula).unwrap();
            }
        });
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
