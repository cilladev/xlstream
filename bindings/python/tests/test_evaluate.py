"""End-to-end evaluation tests for xlstream Python bindings."""

import os

import pytest

import xlstream


class TestEvaluateBasic:
    def test_evaluate_returns_dict(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output)
        assert isinstance(result, dict)

    def test_evaluate_dict_keys(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output)
        assert "rows_processed" in result
        assert "formulas_evaluated" in result
        assert "duration_ms" in result

    def test_evaluate_dict_types(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output)
        assert isinstance(result["rows_processed"], int)
        assert isinstance(result["formulas_evaluated"], int)
        assert isinstance(result["duration_ms"], int)

    def test_evaluate_row_count(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output)
        assert result["rows_processed"] == 6

    def test_evaluate_formula_count(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output)
        assert result["formulas_evaluated"] == 5

    def test_evaluate_creates_output_file(self, simple_fixture, tmp_output):
        xlstream.evaluate(simple_fixture, tmp_output)
        assert os.path.exists(tmp_output)

    def test_evaluate_output_is_valid_xlsx(self, simple_fixture, tmp_output):
        xlstream.evaluate(simple_fixture, tmp_output)
        assert os.path.getsize(tmp_output) > 0


class TestEvaluateWorkers:
    def test_workers_keyword_only(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output, workers=1)
        assert result["rows_processed"] > 0

    def test_workers_none_uses_auto(self, simple_fixture, tmp_output):
        result = xlstream.evaluate(simple_fixture, tmp_output, workers=None)
        assert result["rows_processed"] > 0

    def test_workers_positional_raises(self, simple_fixture, tmp_output):
        with pytest.raises(TypeError):
            xlstream.evaluate(simple_fixture, tmp_output, 4)


class TestEvaluateErrors:
    def test_missing_input_raises_oserror(self, tmp_output):
        with pytest.raises(OSError):
            xlstream.evaluate("nonexistent_file.xlsx", tmp_output)

    def test_unsupported_formula_raises(self, unsupported_fixture, tmp_output):
        with pytest.raises(xlstream.UnsupportedFormula):
            xlstream.evaluate(unsupported_fixture, tmp_output)
