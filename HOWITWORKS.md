# How xlstream works

A streaming Excel formula evaluation engine. Reads `.xlsx` row-by-row, evaluates formulas in bounded memory, writes `.xlsx` out.

## The big picture

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  .xlsx   │────>│  PARSE   │────>│ CLASSIFY  │────>│ PRELUDE  │────>│  STREAM  │────> .xlsx
│  input   │     │ formulas │     │ formulas  │     │ (pass 1) │     │ (pass 2) │      output
└──────────┘     └──────────┘     └──────────┘     └──────────┘     └──────────┘
```

---

## Step 1: Read the file and find formulas

Open the workbook, scan every sheet for formula cells. The first sheet that contains formulas becomes the "main sheet."

```
Input workbook:
┌─────────────────────────────────────┐
│ Sheet1 (main - first with formulas) │
│                                     │
│     A        B            C         │
│ 1 | Name   | Revenue   | Tax       │
│ 2 | Alice  | 5000      | =B2*0.1   │  <-- formula found
│ 3 | Bob    | 3000      | =B3*0.1   │  <-- formula found
│ 4 | Carol  | 7000      | =B4*0.1   │  <-- formula found
│ ...                                 │
│ 700k| Zara | 2000      | =B700000*0.1
└─────────────────────────────────────┘

Result: col C has formulas
```

## Step 2: Parse formulas into ASTs

Each formula string is parsed into an abstract syntax tree. If all formulas in a column have the same structure (differing only in same-sheet row numbers), one AST represents the entire column.

```
"=B2*0.1"  --parse-->    *
                        / \
                   CellRef  Number
                   (col B)  (0.1)

All formulas in col C have the same structure
--> ONE AST stored for the entire column
```

## Step 3: Classify each formula

Determines whether a formula can be evaluated in a streaming pass, needs prelude data, or must be rejected.

```
                     ┌─────────┐
                     │ Formula │
                     └────┬────┘
                          │
                    ┌─────▼─────┐
                    │ Row-local? │---- YES --> RowLocal (can stream)
                    │ (no future │              =B2*0.1, =IF(A2>0,...)
                    │  row refs) │
                    └─────┬─────┘
                          │ NO
                    ┌─────▼──────┐
                    │ Aggregate? │---- YES --> PreludeAggregate (pass 1 computes)
                    │ (SUM, AVG  │              =SUMIF(A:A,">0"), =AVERAGE(B:B)
                    │  whole col)│
                    └─────┬──────┘
                          │ NO
                    ┌─────▼──────┐
                    │ Lookup?    │---- YES --> Lookup (prelude loads sheet)
                    │ (VLOOKUP,  │              =VLOOKUP(A2,Sheet2!A:C,3,FALSE)
                    │  XLOOKUP)  │
                    └─────┬──────┘
                          │ NO
                    ┌─────▼──────┐
                    │ REJECT     │  =OFFSET(...), =INDIRECT(...), =FILTER(...)
                    │ Unsupported│  --> XlStreamError with doc link
                    └────────────┘
```

## Step 4: Prelude (pass 1) - read the whole file once

Computes everything that can't be done row-by-row. Reads the file front-to-back, then discards the data.

```
┌─────────────────────────────────────────────────────┐
│                    PRELUDE                           │
│                                                     │
│  Aggregates (computed by scanning all rows once):   │
│  ┌──────────────────────────────┐                   │
│  │ SUM(Revenue:Revenue) = 15000 │                   │
│  │ COUNT(A:A)           = 3     │                   │
│  │ SUMIF(A:A,">0")     = 15000  │                   │
│  └──────────────────────────────┘                   │
│                                                     │
│  Lookup sheets (loaded fully into memory):          │
│  ┌────────────────────────────┐                     │
│  │ Sheet2 "Thresholds"       │                      │
│  │   A      B                │                      │
│  │ 1| Low  | 1000            │  <-- hash-indexed    │
│  │ 2| Mid  | 5000            │      for O(1) lookup │
│  │ 3| High | 10000           │                      │
│  └────────────────────────────┘                     │
└─────────────────────────────────────────────────────┘
```

## Step 5: Stream (pass 2) - evaluate row by row

The core of the engine. Bounded memory regardless of file size.

```
                    ┌──────────┐
                    │ .xlsx    │
                    │ reader   │
                    └────┬─────┘
                         │ one row at a time
                         ▼
        ┌────────────────────────────────────┐
        │           ROW EVALUATOR            │
        │                                    │
        │  Inputs available:                 │
        │  |-- this row's cell values        │
        │  |-- prelude scalars (SUM, COUNT)  │
        │  '-- prelude lookup sheets         │
        │                                    │
        │  NOT available:                    │
        │  '-- any future row <-- THE RULE   │
        │                                    │
        └────────────┬───────────────────────┘
                     │ evaluated row
                     ▼
                ┌──────────┐
                │  .xlsx   │
                │  writer  │  <-- constant_memory mode
                └──────────┘      (flushes to disk, never buffers)
```

Row by row in detail:

```
Row 2 arrives: [Alice, 5000, <formula>]

  AST for col C:    *
                   / \
             CellRef  0.1
             (col B)
                |
                v
         scope.get(B) = 5000    <-- reads from CURRENT row
                |
         5000 * 0.1 = 500
                |
                v
  Row 2 written: [Alice, 5000, 500]
  Row 2 DISCARDED from memory


Row 3 arrives: [Bob, 3000, <formula>]

  Same AST reused:  *
                   / \
             CellRef  0.1
             (col B)
                |
                v
         scope.get(B) = 3000    <-- reads from THIS row now
                |
         3000 * 0.1 = 300
                |
                v
  Row 3 written: [Bob, 3000, 300]
  Row 3 DISCARDED from memory

  ...repeat 700k times...
```

## Why one AST works for same-sheet refs

The interpreter ignores the row number in same-sheet cell references. It always reads from the current streaming row.

```
Excel formulas:          What the interpreter actually does:

Row 2: =B2*0.1    --+
Row 3: =B3*0.1      +-->  All become: scope.get(col_B) * 0.1
Row 4: =B4*0.1    --+
                           The row number in the AST is IGNORED.
                           scope.get() always reads the current
                           streaming row.
```

## Why cross-sheet refs are different

Cross-sheet references use the literal row/col from the AST. They look up a fixed cell in a prelude-loaded sheet, not the current row.

```
Row 2: =Sheet1!A2*2       interpreter does:
                           prelude.lookup_sheet("Sheet1").cell(row=2, col=A)
                                                              ^
                                                              |
                                                         LITERAL from AST
                                                         (not current row)

Row 3: =Sheet1!A3*2       interpreter does:
                           prelude.lookup_sheet("Sheet1").cell(row=3, col=A)
                                                              ^
                                                              |
                                                         DIFFERENT row
                                                         = DIFFERENT AST needed
```

## Mixed-column handling

When a column has formulas with different AST structures (e.g., some rows use same-sheet refs, others use cross-sheet refs), the engine stores per-row overrides for the exceptions.

```
BEFORE (broken):                    AFTER (fixed):

col_formulas:                       col_asts (default):
  B: AST("=A2*2")  <-- first wins    B: AST("=A2*2")

  Row 4's =Sheet1!A2*2             row_overrides:
  is THROWN AWAY                      B: { row 4: AST("=Sheet1!A2*2") }


During streaming:                   During streaming:

Row 2: use B's AST --> correct      Row 2: no override --> use default --> correct
Row 3: use B's AST --> correct      Row 3: no override --> use default --> correct
Row 4: use B's AST --> WRONG        Row 4: override found --> use it --> CORRECT
       (reads A4=999, not                  (reads Sheet1!A2=5000)
        Sheet1!A2=5000)
```

## Parallel mode (>10k rows)

When the main sheet exceeds 10,000 data rows, the engine splits work across threads. Each worker opens its own reader seeked to its chunk. Results are reassembled in row order.

```
                    ┌──────────┐
                    │  Input   │
                    └────┬─────┘
                         │
              ┌──────────┼──────────┐
              v          v          v
         ┌────────┐ ┌────────┐ ┌────────┐
         │Worker 1│ │Worker 2│ │Worker 3│
         │row 1-  │ │row 3k- │ │row 6k- │
         │  3000  │ │  6000  │ │  10000 │
         └───┬────┘ └───┬────┘ └───┬────┘
             │          │          │
             └──────┬───┘──────────┘
                    v
              ┌───────────┐
              │  Ordered  │  <-- BTreeMap reassembles
              │  Writer   │      row order
              └───────────┘

Each worker has:
|-- its own calamine reader (seeked to its chunk)
|-- shared prelude (Arc, read-only)
|-- shared ASTs + overrides (Arc, read-only)
'-- sends results through bounded channel
```

## Memory model

```
┌─────────────────────────────────────────┐
│              MEMORY AT ANY POINT        │
│                                         │
│  Fixed (loaded once, kept forever):     │
│  |-- Prelude aggregates    ~few KB      │
│  |-- Lookup sheets         ~varies      │
│  |-- Column ASTs           ~few KB      │
│  '-- Row overrides         ~per-exception
│                                         │
│  Transient (one row at a time):         │
│  |-- Current row values    ~few KB      │
│  '-- Writer buffer         ~few KB      │
│                                         │
│  NOT in memory:                         │
│  '-- All other rows        <-- streaming│
│                                         │
│  formualizer (comparison):              │
│  '-- ALL rows as graph vertices = 3.3 GB│
└─────────────────────────────────────────┘
```

---

## Where each step lives in the code

### Entry point

The entire pipeline is driven by one function:

| What | File | Line | Function |
|------|------|------|----------|
| Public entry point | `crates/xlstream-eval/src/evaluate.rs` | 95 | `evaluate()` |

`evaluate()` calls `build_plan()` then either `stream_single_threaded()` or `stream_parallel()`.

### Step 1: Read the file and find formulas

| What | File | Line | Function |
|------|------|------|----------|
| Open xlsx | `crates/xlstream-io/src/reader.rs` | 66 | `Reader::open()` |
| List sheet names | `crates/xlstream-io/src/reader.rs` | 86 | `Reader::sheet_names()` |
| Extract named ranges | `crates/xlstream-io/src/reader.rs` | 107 | `Reader::defined_names()` |
| Collect formulas per sheet | `crates/xlstream-io/src/reader.rs` | 181 | `Reader::formulas()` |
| Orchestration (find main sheet) | `crates/xlstream-eval/src/evaluate.rs` | 195 | `build_plan()` |

### Step 2: Parse formulas into ASTs

| What | File | Line | Function |
|------|------|------|----------|
| Parse formula string to AST | `crates/xlstream-parse/src/parser.rs` | 28 | `parse()` |
| Resolve named ranges in AST | `crates/xlstream-parse/src/resolve.rs` | 31 | `resolve_named_ranges()` |
| Build per-column AST map | `crates/xlstream-eval/src/evaluate.rs` | 711 | `build_eval_plan()` |
| AST node types | `crates/xlstream-parse/src/view.rs` | 24 | `enum NodeView` |
| AST traversal handle | `crates/xlstream-parse/src/view.rs` | 111 | `struct NodeRef` |

### Step 3: Classify formulas

| What | File | Line | Function |
|------|------|------|----------|
| Classify a formula | `crates/xlstream-parse/src/classify.rs` | 338 | `classify()` |
| Rewrite AST (aggregate -> PreludeRef) | `crates/xlstream-parse/src/rewrite.rs` | 148 | `rewrite()` |
| Collect lookup keys from AST | `crates/xlstream-parse/src/rewrite.rs` | 319 | `collect_lookup_keys()` |
| Extract cell/range references | `crates/xlstream-parse/src/references.rs` | 109 | `extract_references()` |
| Build topo order for column deps | `crates/xlstream-eval/src/topo.rs` | 33 | `topo_sort()` |

### Step 4: Prelude (pass 1)

| What | File | Line | Function |
|------|------|------|----------|
| Collect aggregate keys from ASTs | `crates/xlstream-eval/src/prelude_plan.rs` | 217 | `collect_aggregate_keys()` |
| Collect bounded range keys | `crates/xlstream-eval/src/prelude_plan.rs` | 509 | `collect_bounded_range_keys()` |
| Execute prelude (scan + compute) | `crates/xlstream-eval/src/prelude_plan.rs` | 594 | `execute_prelude()` |
| Load lookup sheets into memory | `crates/xlstream-eval/src/lookup/loader.rs` | 37 | `load_lookup_sheets()` |

### Step 5: Stream (pass 2)

| What | File | Line | Function |
|------|------|------|----------|
| Single-threaded streaming | `crates/xlstream-eval/src/evaluate.rs` | 462 | `stream_single_threaded()` |
| Parallel streaming (coordinator) | `crates/xlstream-eval/src/evaluate.rs` | 541 | `stream_parallel()` |
| Parallel worker (per-thread) | `crates/xlstream-eval/src/evaluate.rs` | 655 | `run_worker()` |
| Stream rows from xlsx | `crates/xlstream-io/src/stream.rs` | 95 | `CellStream::next_row()` |
| Seek to row (parallel workers) | `crates/xlstream-io/src/stream.rs` | 122 | `CellStream::seek_to_row()` |

### Row evaluation (inside the streaming loop)

| What | File | Line | Function |
|------|------|------|----------|
| Interpreter (evaluates one AST node) | `crates/xlstream-eval/src/interp.rs` | 73 | `Interpreter::eval()` |
| Same-sheet cell ref resolution | `crates/xlstream-eval/src/interp.rs` | 90 | `scope.get(col)` |
| Cross-sheet cell ref resolution | `crates/xlstream-eval/src/interp.rs` | 81 | `prelude.lookup_sheet().cell()` |
| Builtin function dispatch | `crates/xlstream-eval/src/builtins/mod.rs` | 83 | `dispatch()` |

### Output

| What | File | Line | Function |
|------|------|------|----------|
| Create output xlsx | `crates/xlstream-io/src/writer.rs` | 59 | `Writer::create()` |
| Add sheet to output | `crates/xlstream-io/src/writer.rs` | 84 | `Writer::add_sheet()` |
| Write one row | `crates/xlstream-io/src/sheet_handle.rs` | 63 | `SheetHandle::write_row()` |
| Finalize and close | `crates/xlstream-io/src/writer.rs` | 106 | `Writer::finish()` |

### Call graph

```
evaluate()                                    evaluate.rs:95
  |
  |-- build_plan()                            evaluate.rs:195
  |     |-- Reader::open()                    reader.rs:66
  |     |-- Reader::formulas()                reader.rs:181
  |     |-- build_eval_plan()                 evaluate.rs:711
  |     |     |-- parse()                     parser.rs:28
  |     |     |-- resolve_named_ranges()      resolve.rs:31
  |     |     |-- classify()                  classify.rs:338
  |     |     |-- rewrite()                   rewrite.rs:148
  |     |     '-- topo_sort()                 topo.rs:33
  |     |-- execute_prelude()                 prelude_plan.rs:594
  |     '-- load_lookup_sheets()              lookup/loader.rs:37
  |
  |-- stream_single_threaded()                evaluate.rs:462
  |     |-- CellStream::next_row()            stream.rs:95
  |     |-- Interpreter::eval()               interp.rs:73
  |     |     |-- scope.get(col)              (same-sheet ref)
  |     |     |-- prelude.lookup_sheet()      (cross-sheet ref)
  |     |     '-- dispatch()                  builtins/mod.rs:83
  |     '-- SheetHandle::write_row()          sheet_handle.rs:63
  |
  '-- stream_parallel()                       evaluate.rs:541
        |-- spawn run_worker() * N            evaluate.rs:655
        |     |-- Reader::open() (per worker)
        |     |-- CellStream::seek_to_row()   stream.rs:122
        |     |-- CellStream::next_row()
        |     |-- Interpreter::eval()
        |     '-- tx.send(row)
        '-- drain rx, write in order
              '-- SheetHandle::write_row()
```
