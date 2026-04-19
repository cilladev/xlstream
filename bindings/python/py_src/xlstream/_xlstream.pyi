from typing import Optional, TypedDict

class EvaluateResult(TypedDict):
    rows_processed: int
    formulas_evaluated: int
    duration_ms: int

def evaluate(
    input_path: str,
    output_path: str,
    *,
    workers: Optional[int] = None,
) -> EvaluateResult:
    """Evaluate formulas in an xlsx workbook and write the results.

    Args:
        input_path: Path to the source xlsx file.
        output_path: Path where the evaluated xlsx is written.
        workers: Number of parallel worker threads. None = auto-detect.

    Returns:
        Dict with rows_processed, formulas_evaluated, and duration_ms.

    Raises:
        OSError: If input file cannot be read or output cannot be written.
        UnsupportedFormula: If a formula cannot be evaluated in streaming mode.
        FormulaParseError: If a formula is malformed.
        ClassificationError: If a formula cannot be classified.
        CircularReferenceError: If formula columns form a dependency cycle.
    """
    ...

class XlStreamError(Exception):
    """Base class for all xlstream errors."""

    ...

class UnsupportedFormula(XlStreamError):
    """A formula cannot be evaluated in streaming mode."""

    ...

class FormulaParseError(XlStreamError):
    """A formula could not be parsed."""

    ...

class ClassificationError(XlStreamError):
    """A formula could not be classified."""

    ...

class CircularReferenceError(XlStreamError):
    """Formula columns form a dependency cycle."""

    ...
