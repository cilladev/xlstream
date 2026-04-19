# xlstream (Python)

Python bindings for the xlstream streaming Excel formula evaluation engine.

## Install (development)

```bash
cd bindings/python
pip install maturin pytest openpyxl
maturin develop --release
```

## Usage

```python
import xlstream

result = xlstream.evaluate("input.xlsx", "output.xlsx")
print(result["rows_processed"])
print(result["formulas_evaluated"])
print(result["duration_ms"])

# With parallel workers
result = xlstream.evaluate("input.xlsx", "output.xlsx", workers=4)
```

## Run tests

```bash
pytest tests/ -v
```

## Build wheel

```bash
maturin build --release
```
