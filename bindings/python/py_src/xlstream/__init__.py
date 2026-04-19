"""xlstream -- streaming Excel formula evaluation engine."""

from importlib.metadata import version as _version

from ._xlstream import (
    evaluate,
    XlStreamError,
    UnsupportedFormula,
    FormulaParseError,
    ClassificationError,
    CircularReferenceError,
)

__all__ = [
    "evaluate",
    "XlStreamError",
    "UnsupportedFormula",
    "FormulaParseError",
    "ClassificationError",
    "CircularReferenceError",
]

__version__ = _version("xlstream")
