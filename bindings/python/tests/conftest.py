"""Shared test fixtures for xlstream Python tests."""

import os

import pytest

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


def _ensure_fixtures_dir():
    os.makedirs(FIXTURES_DIR, exist_ok=True)


@pytest.fixture
def tmp_output(tmp_path):
    """Temporary output xlsx path."""
    return str(tmp_path / "output.xlsx")


@pytest.fixture(scope="session")
def simple_fixture():
    """5-row fixture: col A (data), col B (data), col C (formula =A+B)."""
    openpyxl = pytest.importorskip("openpyxl")
    _ensure_fixtures_dir()
    fixture_path = os.path.join(FIXTURES_DIR, "simple.xlsx")
    if os.path.exists(fixture_path):
        return fixture_path

    wb = openpyxl.Workbook()
    ws = wb.active
    ws.title = "Sheet1"
    ws.append(["A", "B", "C"])
    for i in range(1, 6):
        row = i + 1
        ws.append([i * 10, i * 20, f"=A{row}+B{row}"])
    wb.save(fixture_path)
    return fixture_path


@pytest.fixture(scope="session")
def unsupported_fixture():
    """Fixture with an unsupported formula (OFFSET)."""
    openpyxl = pytest.importorskip("openpyxl")
    _ensure_fixtures_dir()
    fixture_path = os.path.join(FIXTURES_DIR, "unsupported.xlsx")
    if os.path.exists(fixture_path):
        return fixture_path

    wb = openpyxl.Workbook()
    ws = wb.active
    ws.append(["A", "B"])
    ws.append([10, "=OFFSET(A1,0,0)"])
    wb.save(fixture_path)
    return fixture_path
