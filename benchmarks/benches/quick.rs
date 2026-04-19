#![allow(clippy::unwrap_used, clippy::cast_precision_loss, clippy::cast_lossless)]

use criterion::{criterion_group, criterion_main, Criterion};
use xlstream_eval::evaluate;

fn generate_quick_fixture() -> tempfile::NamedTempFile {
    use rust_xlsxwriter::{Formula, Workbook};

    let tmp = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    for c in 0u16..5 {
        ws.write_string(0, c, format!("Col{c}")).unwrap();
    }
    for r in 1u32..=5_000 {
        let excel_row = r + 1;
        ws.write_number(r, 0, r as f64).unwrap();
        ws.write_number(r, 1, (r as f64) * 10.0).unwrap();
        ws.write_number(r, 2, (r as f64) * 0.5).unwrap();
        ws.write_formula(
            r,
            3,
            Formula::new(format!("=A{excel_row}+B{excel_row}"))
                .set_result(((r as f64) + (r as f64) * 10.0).to_string()),
        )
        .unwrap();
        ws.write_formula(
            r,
            4,
            Formula::new(format!("=C{excel_row}*2")).set_result((r as f64).to_string()),
        )
        .unwrap();
    }

    wb.save(tmp.path()).unwrap();
    tmp
}

fn bench_quick(c: &mut Criterion) {
    let input = generate_quick_fixture();
    let mut group = c.benchmark_group("quick");
    group.sample_size(10);

    group.bench_function("eval_5k_rows", |b| {
        let output = tempfile::NamedTempFile::with_suffix(".xlsx").unwrap();
        b.iter(|| evaluate(input.path(), output.path(), Some(1)).unwrap());
    });

    group.finish();
}

criterion_group!(benches, bench_quick);
criterion_main!(benches);
