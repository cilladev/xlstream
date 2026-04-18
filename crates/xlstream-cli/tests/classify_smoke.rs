//! Smoke tests that run the compiled binary and check stdout.

use std::process::Command;

#[test]
fn classify_smoke_aggregate_writes_to_stdout() {
    let bin = env!("CARGO_BIN_EXE_xlstream");
    let out = Command::new(bin).args(["classify", "SUM(A:A)"]).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("AggregateOnly"), "expected AggregateOnly: {stdout}");
}

#[test]
fn classify_smoke_unsupported_writes_to_stdout() {
    let bin = env!("CARGO_BIN_EXE_xlstream");
    let out = Command::new(bin).args(["classify", "OFFSET(A1, 1, 0)"]).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Unsupported"), "expected Unsupported: {stdout}");
}

#[test]
fn classify_smoke_lookup_with_registered_sheet() {
    let bin = env!("CARGO_BIN_EXE_xlstream");
    let out = Command::new(bin)
        .args([
            "classify",
            "VLOOKUP(A2, 'Region Info'!A:C, 2, FALSE)",
            "--row",
            "2",
            "--col",
            "5",
            "--lookup-sheet",
            "Region Info",
        ])
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("LookupOnly"), "expected LookupOnly: {stdout}");
}

#[test]
fn classify_smoke_row_local_formula() {
    let bin = env!("CARGO_BIN_EXE_xlstream");
    let out = Command::new(bin).args(["classify", "1+2"]).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("RowLocal"), "expected RowLocal: {stdout}");
}
