"""Exception hierarchy tests for xlstream Python bindings."""

import pytest

import xlstream


class TestExceptionImport:
    def test_xlstream_error_importable(self):
        assert xlstream.XlStreamError is not None

    def test_unsupported_formula_importable(self):
        assert xlstream.UnsupportedFormula is not None

    def test_formula_parse_error_importable(self):
        assert xlstream.FormulaParseError is not None

    def test_classification_error_importable(self):
        assert xlstream.ClassificationError is not None

    def test_circular_reference_error_importable(self):
        assert xlstream.CircularReferenceError is not None


class TestExceptionHierarchy:
    def test_xlstream_error_is_exception(self):
        assert issubclass(xlstream.XlStreamError, Exception)

    def test_unsupported_formula_is_xlstream_error(self):
        assert issubclass(xlstream.UnsupportedFormula, xlstream.XlStreamError)

    def test_formula_parse_error_is_xlstream_error(self):
        assert issubclass(xlstream.FormulaParseError, xlstream.XlStreamError)

    def test_classification_error_is_xlstream_error(self):
        assert issubclass(xlstream.ClassificationError, xlstream.XlStreamError)

    def test_circular_reference_error_is_xlstream_error(self):
        assert issubclass(xlstream.CircularReferenceError, xlstream.XlStreamError)


class TestExceptionCatch:
    def test_unsupported_caught_by_xlstream_error(
        self, unsupported_fixture, tmp_output
    ):
        with pytest.raises(xlstream.XlStreamError):
            xlstream.evaluate(unsupported_fixture, tmp_output)

    def test_missing_input_is_oserror_not_xlstream(self, tmp_output):
        with pytest.raises(OSError):
            xlstream.evaluate("nonexistent.xlsx", tmp_output)
        try:
            xlstream.evaluate("nonexistent.xlsx", tmp_output)
        except xlstream.XlStreamError:
            pytest.fail("I/O error should be OSError, not XlStreamError")
        except OSError:
            pass
