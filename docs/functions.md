# Supported Excel functions

Every Excel function (~493), organized by category. Cross-referenced against xlstream support status, streaming compatibility, target version, and competitor engines.

## Legend

| Symbol | Meaning |
|---|---|
| x | Implemented and shipped |
| . | Planned (streaming-compatible, not yet built) |
| - | Permanently excluded (incompatible with streaming) |

## Logical

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| TRUE | x | yes | v0.1 | x | x | |
| FALSE | x | yes | v0.1 | x | x | |
| IF | x | yes | v0.1 | x | x | |
| IFS | x | yes | v0.1 | x | | |
| SWITCH | x | yes | v0.1 | x | | |
| IFERROR | x | yes | v0.1 | x | x | |
| IFNA | x | yes | v0.1 | x | | |
| AND | x | yes | v0.1 | x | x | |
| OR | x | yes | v0.1 | x | x | |
| NOT | x | yes | v0.1 | x | x | |
| XOR | x | yes | v0.1 | x | | |
| LET | . | yes | v0.4 | x | | Scoped variable binding |
| LAMBDA | - | no | - | x | | Closures, recursion |
| MAP | - | no | - | | | LAMBDA + spill |
| REDUCE | - | no | - | | | LAMBDA + spill |
| SCAN | - | no | - | | | LAMBDA + spill |
| BYROW | - | no | - | | | LAMBDA + spill |
| BYCOL | - | no | - | | | LAMBDA + spill |
| MAKEARRAY | - | no | - | | | LAMBDA + spill |

## Math & Trigonometry

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| ABS | x | yes | v0.1 | x | x | |
| ACOS | x | yes | v0.1 | x | | |
| ACOSH | x | yes | v0.3 | x | | |
| ACOT | . | yes | v0.4 | | | |
| ACOTH | . | yes | v0.4 | | | |
| AGGREGATE | x | yes | v0.3 | x | | Multi-mode aggregate (fn 1-13, options 0-7; hidden-row/nested-SUBTOTAL ignoring deferred) |
| ARABIC | x | yes | v0.3 | | | |
| ASIN | x | yes | v0.1 | x | | |
| ASINH | x | yes | v0.3 | x | | |
| ATAN | x | yes | v0.1 | x | | |
| ATAN2 | x | yes | v0.1 | x | | |
| ATANH | x | yes | v0.3 | x | | |
| BASE | x | yes | v0.3 | | | |
| CEILING | x | yes | v0.1 | x | x | |
| CEILING.MATH | x | yes | v0.3 | x | x | |
| CEILING.PRECISE | x | yes | v0.3 | x | x | |
| COMBIN | x | yes | v0.3 | | | |
| COMBINA | x | yes | v0.3 | | | |
| COS | x | yes | v0.1 | x | | |
| COSH | x | yes | v0.3 | x | | |
| COT | x | yes | v0.3 | | | |
| COTH | x | yes | v0.3 | | | |
| CSC | x | yes | v0.3 | | | |
| CSCH | x | yes | v0.3 | | | |
| DECIMAL | . | yes | v0.4 | | | |
| DEGREES | x | yes | v0.3 | x | | |
| EVEN | x | yes | v0.3 | x | x | |
| EXP | x | yes | v0.1 | x | | |
| FACT | x | yes | v0.3 | | | |
| FACTDOUBLE | x | yes | v0.3 | | | |
| FLOOR | x | yes | v0.1 | x | x | |
| FLOOR.MATH | x | yes | v0.3 | x | x | |
| FLOOR.PRECISE | x | yes | v0.3 | x | x | |
| GCD | x | yes | v0.3 | | | |
| INT | x | yes | v0.1 | x | x | |
| ISO.CEILING | x | yes | v0.3 | x | x | |
| LCM | x | yes | v0.3 | | | |
| LN | x | yes | v0.1 | x | x | |
| LOG | x | yes | v0.1 | x | x | |
| LOG10 | x | yes | v0.1 | x | | |
| MDETERM | . | yes | v0.4 | | | |
| MINVERSE | . | yes | v0.4 | | | |
| MMULT | . | yes | v0.4 | | | |
| MOD | x | yes | v0.1 | x | x | |
| MROUND | x | yes | v0.3 | x | x | |
| MULTINOMIAL | . | yes | v0.4 | | | |
| MUNIT | . | yes | v0.4 | | | |
| ODD | x | yes | v0.3 | x | x | |
| PI | x | yes | v0.1 | x | x | |
| POWER | x | yes | v0.1 | x | x | |
| PRODUCT | x | yes | v0.1 | x | | |
| QUOTIENT | . | yes | v0.4 | | | |
| RADIANS | x | yes | v0.3 | x | | |
| RAND | - | no | - | x | x | Volatile |
| RANDARRAY | - | no | - | | | Volatile + spill |
| RANDBETWEEN | - | no | - | x | x | Volatile |
| ROMAN | x | yes | v0.3 | | | |
| ROUND | x | yes | v0.1 | x | x | |
| ROUNDDOWN | x | yes | v0.1 | x | x | |
| ROUNDUP | x | yes | v0.1 | x | x | |
| SEC | x | yes | v0.3 | | | |
| SECH | x | yes | v0.3 | | | |
| SEQUENCE | - | no | - | | | Spill |
| SERIESSUM | . | yes | v0.4 | | | |
| SIGN | x | yes | v0.1 | x | | |
| SIN | x | yes | v0.1 | x | | |
| SINH | x | yes | v0.3 | x | | |
| SQRT | x | yes | v0.1 | x | x | |
| SQRTPI | . | yes | v0.4 | | | |
| SUBTOTAL | x | yes | v0.3 | x | | fn 1-11/101-111; hidden-row ignoring deferred |
| SUM | x | yes | v0.1 | x | x | |
| SUMIF | x | yes | v0.1 | x | x | Offset sum range (`B5:B14` vs `A2:A11`) returns #VALUE!; Excel resizes |
| SUMIFS | x | yes | v0.1 | x | | |
| SUMPRODUCT | x | yes | v0.2 | x | x | |
| SUMSQ | . | yes | v0.4 | | | |
| SUMX2MY2 | . | yes | v0.4 | | | |
| SUMX2PY2 | . | yes | v0.4 | | | |
| SUMXMY2 | . | yes | v0.4 | | | |
| TAN | x | yes | v0.1 | x | | |
| TANH | x | yes | v0.3 | x | | |
| TRUNC | x | yes | v0.3 | x | x | |

## Statistical

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| AVERAGE | x | yes | v0.1 | x | x | |
| AVERAGEA | . | yes | v0.4 | | | |
| AVERAGEIF | x | yes | v0.1 | x | | Offset avg range returns #VALUE!; Excel resizes |
| AVERAGEIFS | x | yes | v0.1 | x | | |
| AVEDEV | x | yes | v0.3 | | | |
| BETA.DIST | . | yes | v0.4 | | | |
| BETA.INV | . | yes | v0.4 | | | |
| BINOM.DIST | x | yes | v0.3 | | | |
| BINOM.DIST.RANGE | . | yes | v0.4 | | | |
| BINOM.INV | x | yes | v0.3 | | | |
| CHISQ.DIST | . | yes | v0.4 | | | |
| CHISQ.DIST.RT | . | yes | v0.4 | | | |
| CHISQ.INV | . | yes | v0.4 | | | |
| CHISQ.INV.RT | . | yes | v0.4 | | | |
| CHISQ.TEST | . | yes | v0.4 | | | |
| CONFIDENCE.NORM | . | yes | v0.4 | | | |
| CONFIDENCE.T | . | yes | v0.4 | | | |
| CORREL | x | yes | v0.3 | | | |
| COUNT | x | yes | v0.1 | x | x | |
| COUNTA | x | yes | v0.1 | x | x | |
| COUNTBLANK | x | yes | v0.1 | x | | |
| COUNTIF | x | yes | v0.1 | x | x | |
| COUNTIFS | x | yes | v0.1 | x | | |
| COVARIANCE.P | x | yes | v0.3 | | | |
| COVARIANCE.S | x | yes | v0.3 | | | |
| DEVSQ | . | yes | v0.4 | | | |
| EXPON.DIST | x | yes | v0.3 | | | |
| F.DIST | . | yes | v0.4 | | | |
| F.DIST.RT | . | yes | v0.4 | | | |
| F.INV | . | yes | v0.4 | | | |
| F.INV.RT | . | yes | v0.4 | | | |
| F.TEST | . | yes | v0.4 | | | |
| FISHER | . | yes | v0.4 | | | |
| FISHERINV | . | yes | v0.4 | | | |
| FORECAST.LINEAR | x | yes | v0.3 | | | |
| FREQUENCY | . | yes | v0.4 | | | |
| GAMMA | . | yes | v0.4 | | | |
| GAMMA.DIST | . | yes | v0.4 | | | |
| GAMMA.INV | . | yes | v0.4 | | | |
| GAMMALN | . | yes | v0.4 | | | |
| GAMMALN.PRECISE | . | yes | v0.4 | | | |
| GAUSS | . | yes | v0.4 | | | |
| GEOMEAN | . | yes | v0.4 | | | |
| GROWTH | . | yes | v0.4 | | | |
| HARMEAN | . | yes | v0.4 | | | |
| HYPGEOM.DIST | . | yes | v0.4 | | | |
| INTERCEPT | x | yes | v0.3 | | | |
| KURT | x | yes | v0.3 | | | |
| LARGE | x | yes | v0.3 | | | |
| LINEST | . | yes | v0.4 | | | |
| LOGEST | . | yes | v0.4 | | | |
| LOGNORM.DIST | . | yes | v0.4 | | | |
| LOGNORM.INV | . | yes | v0.4 | | | |
| MAX | x | yes | v0.1 | x | x | |
| MAXA | . | yes | v0.4 | | | |
| MAXIFS | x | yes | v0.2 | x | x | |
| MEDIAN | x | yes | v0.1 | x | | |
| MIN | x | yes | v0.1 | x | x | |
| MINA | . | yes | v0.4 | | | |
| MINIFS | x | yes | v0.2 | x | x | |
| MODE.SNGL | x | yes | v0.3 | x | x | |
| MODE.MULT | . | yes | v0.4 | | | |
| NEGBINOM.DIST | . | yes | v0.4 | | | |
| NORM.DIST | x | yes | v0.3 | x | x | |
| NORM.INV | x | yes | v0.3 | x | x | |
| NORM.S.DIST | x | yes | v0.3 | | | |
| NORM.S.INV | x | yes | v0.3 | | | |
| PEARSON | . | yes | v0.4 | | | |
| PERCENTILE.EXC | x | yes | v0.3 | | | |
| PERCENTILE.INC | x | yes | v0.3 | | | |
| PERCENTRANK.EXC | . | yes | v0.4 | | | |
| PERCENTRANK.INC | . | yes | v0.4 | | | |
| PERMUT | x | yes | v0.3 | | | |
| PERMUTATIONA | x | yes | v0.3 | | | |
| PHI | . | yes | v0.4 | | | |
| POISSON.DIST | x | yes | v0.3 | | | |
| PROB | . | yes | v0.4 | | | |
| QUARTILE.EXC | x | yes | v0.3 | | | |
| QUARTILE.INC | x | yes | v0.3 | | | |
| RANK.AVG | x | yes | v0.3 | | | |
| RANK.EQ | x | yes | v0.3 | | | |
| RSQ | x | yes | v0.3 | | | |
| SKEW | x | yes | v0.3 | | | |
| SKEW.P | x | yes | v0.3 | | | |
| SLOPE | x | yes | v0.3 | | | |
| SMALL | x | yes | v0.3 | | | |
| STANDARDIZE | . | yes | v0.4 | | | |
| STDEV.P | x | yes | v0.3 | | | |
| STDEV.S | x | yes | v0.3 | | | |
| STDEVA | . | yes | v0.4 | | | |
| STDEVPA | . | yes | v0.4 | | | |
| STEYX | . | yes | v0.4 | | | |
| T.DIST | x | yes | v0.3 | | | |
| T.DIST.2T | x | yes | v0.3 | | | |
| T.DIST.RT | x | yes | v0.3 | | | |
| T.INV | x | yes | v0.3 | | | |
| T.INV.2T | x | yes | v0.3 | | | |
| T.TEST | . | yes | v0.4 | | | |
| TREND | . | yes | v0.4 | | | |
| TRIMMEAN | . | yes | v0.4 | | | |
| VAR.P | x | yes | v0.3 | | | |
| VAR.S | x | yes | v0.3 | | | |
| VARA | . | yes | v0.4 | | | |
| VARPA | . | yes | v0.4 | | | |
| WEIBULL.DIST | . | yes | v0.4 | | | |
| Z.TEST | . | yes | v0.4 | | | |

## Text

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| CHAR | . | yes | v0.4 | | | |
| CLEAN | x | yes | v0.1 | x | | |
| CODE | . | yes | v0.4 | | | |
| CONCAT | x | yes | v0.1 | x | x | |
| CONCATENATE | x | yes | v0.1 | x | x | |
| DOLLAR | . | yes | v0.4 | | | |
| EXACT | x | yes | v0.1 | x | | |
| FIND | x | yes | v0.1 | x | x | |
| FINDB | . | yes | v0.4 | | | |
| FIXED | . | yes | v0.4 | | | |
| LEFT | x | yes | v0.1 | x | x | |
| LEFTB | . | yes | v0.4 | | | |
| LEN | x | yes | v0.1 | x | x | |
| LENB | . | yes | v0.4 | | | |
| LOWER | x | yes | v0.1 | x | x | |
| MID | x | yes | v0.1 | x | x | |
| MIDB | . | yes | v0.4 | | | |
| NUMBERVALUE | . | yes | v0.4 | | | |
| PROPER | x | yes | v0.1 | x | | |
| REPLACE | x | yes | v0.1 | x | | |
| REPLACEB | . | yes | v0.4 | | | |
| REPT | . | yes | v0.4 | | | |
| RIGHT | x | yes | v0.1 | x | x | |
| RIGHTB | . | yes | v0.4 | | | |
| SEARCH | x | yes | v0.1 | x | | |
| SEARCHB | . | yes | v0.4 | | | |
| SUBSTITUTE | x | yes | v0.1 | x | | |
| T | . | yes | v0.4 | | | |
| TEXT | x | yes | v0.1 | x | | |
| TEXTAFTER | . | yes | v0.4 | | | |
| TEXTBEFORE | . | yes | v0.4 | | | |
| TEXTJOIN | x | yes | v0.1 | x | | |
| TEXTSPLIT | - | no | - | | | Spill |
| TRIM | x | yes | v0.1 | x | x | |
| UNICHAR | . | yes | v0.4 | | | |
| UNICODE | . | yes | v0.4 | | | |
| UPPER | x | yes | v0.1 | x | x | |
| VALUE | x | yes | v0.1 | x | x | |
| VALUETOTEXT | . | yes | v0.4 | | | |

## Date & Time

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| DATE | x | yes | v0.1 | x | x | |
| DATEDIF | x | yes | v0.1 | x | | |
| DATEVALUE | . | yes | v0.4 | | | |
| DAY | x | yes | v0.1 | x | x | |
| DAYS | . | yes | v0.4 | | | |
| DAYS360 | . | yes | v0.4 | | | |
| EDATE | x | yes | v0.1 | x | x | |
| EOMONTH | x | yes | v0.1 | x | x | |
| HOUR | . | yes | v0.4 | | | |
| ISOWEEKNUM | . | yes | v0.4 | x | | |
| MINUTE | . | yes | v0.4 | | | |
| MONTH | x | yes | v0.1 | x | x | |
| NETWORKDAYS | x | yes | v0.1 | x | | |
| NETWORKDAYS.INTL | . | yes | v0.4 | | | |
| NOW | x | yes | v0.1 | x | x | Once per run |
| SECOND | . | yes | v0.4 | | | |
| TIME | . | yes | v0.4 | | | |
| TIMEVALUE | . | yes | v0.4 | | | |
| TODAY | x | yes | v0.1 | x | x | Once per run |
| WEEKDAY | x | yes | v0.1 | x | x | |
| WEEKNUM | . | yes | v0.4 | | | |
| WORKDAY | x | yes | v0.1 | x | | |
| WORKDAY.INTL | . | yes | v0.4 | | | |
| YEAR | x | yes | v0.1 | x | x | |
| YEARFRAC | . | yes | v0.4 | x | x | |

## Lookup & Reference

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| ADDRESS | . | yes | v0.4 | | | Builds A1 string |
| CHOOSE | x | yes | v0.1 | x | | |
| COLUMN | x | yes | v0.3 | x | | |
| COLUMNS | x | yes | v0.3 | x | | |
| HLOOKUP | x | yes | v0.1 | x | | |
| INDEX | x | yes | v0.1 | x | x | |
| LOOKUP | . | yes | v0.4 | x | | Legacy |
| MATCH | x | yes | v0.1 | x | x | |
| ROW | x | yes | v0.3 | x | | |
| ROWS | x | yes | v0.3 | x | | |
| VLOOKUP | x | yes | v0.1 | x | x | |
| XLOOKUP | x | yes | v0.1 | x | | |
| XMATCH | x | yes | v0.1 | x | | |
| AREAS | - | no | - | | | Runtime range inspection |
| CHOOSECOLS | - | no | - | | | Spill |
| CHOOSEROWS | - | no | - | | | Spill |
| DROP | - | no | - | | | Spill |
| EXPAND | - | no | - | | | Spill |
| FILTER | - | no | - | x | | Spill |
| FORMULATEXT | - | no | - | | | Runtime inspection |
| HSTACK | - | no | - | | | Spill |
| HYPERLINK | - | no | - | x | | Side-effecting |
| INDIRECT | - | no | - | x | | Runtime address |
| OFFSET | - | no | - | x | | Runtime address |
| SORT | - | no | - | x | | Spill |
| SORTBY | - | no | - | | | Spill |
| TAKE | - | no | - | | | Spill |
| TOCOL | - | no | - | | | Spill |
| TOROW | - | no | - | | | Spill |
| TRANSPOSE | - | no | - | | | Spill |
| UNIQUE | - | no | - | x | | Spill |
| VSTACK | - | no | - | | | Spill |

## Information

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| ISBLANK | x | yes | v0.1 | x | | |
| ISERROR | x | yes | v0.1 | x | | |
| ISLOGICAL | x | yes | v0.1 | x | | |
| ISNA | x | yes | v0.1 | x | | |
| ISNONTEXT | x | yes | v0.1 | x | | |
| ISNUMBER | x | yes | v0.1 | x | | |
| ISREF | x | yes | v0.1 | x | | Lazy dispatch |
| ISTEXT | x | yes | v0.1 | x | | |
| NA | x | yes | v0.1 | x | | |
| TYPE | x | yes | v0.1 | x | | |
| ERROR.TYPE | . | yes | v0.4 | | | |
| ISERR | . | yes | v0.4 | | | |
| ISEVEN | . | yes | v0.4 | | | |
| ISODD | . | yes | v0.4 | | | |
| N | . | yes | v0.4 | | | |
| CELL | - | no | - | | | Runtime environment |
| INFO | - | no | - | | | Runtime environment |
| ISFORMULA | - | no | - | | | Runtime inspection |
| SHEET | - | no | - | | | Runtime inspection |
| SHEETS | - | no | - | | | Runtime inspection |

## Financial

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| FV | x | yes | v0.1 | x | | |
| IRR | x | yes | v0.1 | x | | Iterative |
| NPV | x | yes | v0.1 | x | | |
| PMT | x | yes | v0.1 | x | | |
| PV | x | yes | v0.1 | x | | |
| RATE | x | yes | v0.1 | x | | Iterative |
| ACCRINT | . | yes | v0.5 | | | |
| ACCRINTM | . | yes | v0.5 | | | |
| CUMIPMT | . | yes | v0.5 | | | |
| CUMPRINC | . | yes | v0.5 | | | |
| DB | . | yes | v0.5 | | | |
| DDB | . | yes | v0.5 | | | |
| DISC | . | yes | v0.5 | | | |
| DOLLARDE | . | yes | v0.5 | | | |
| DOLLARFR | . | yes | v0.5 | | | |
| DURATION | . | yes | v0.5 | | | |
| EFFECT | . | yes | v0.5 | | | |
| FVSCHEDULE | . | yes | v0.5 | | | |
| INTRATE | . | yes | v0.5 | | | |
| IPMT | . | yes | v0.5 | | | |
| ISPMT | . | yes | v0.5 | | | |
| MDURATION | . | yes | v0.5 | | | |
| MIRR | . | yes | v0.5 | | | |
| NOMINAL | . | yes | v0.5 | | | |
| NPER | . | yes | v0.5 | | | |
| PDURATION | . | yes | v0.5 | | | |
| PPMT | . | yes | v0.5 | | | |
| PRICE | . | yes | v0.5 | | | |
| PRICEDISC | . | yes | v0.5 | | | |
| PRICEMAT | . | yes | v0.5 | | | |
| RECEIVED | . | yes | v0.5 | | | |
| RRI | . | yes | v0.5 | | | |
| SLN | . | yes | v0.5 | | | |
| SYD | . | yes | v0.5 | | | |
| TBILLEQ | . | yes | v0.5 | | | |
| TBILLPRICE | . | yes | v0.5 | | | |
| TBILLYIELD | . | yes | v0.5 | | | |
| VDB | . | yes | v0.5 | | | |
| XIRR | . | yes | v0.5 | | | |
| XNPV | . | yes | v0.5 | | | |
| YIELD | . | yes | v0.5 | | | |
| YIELDDISC | . | yes | v0.5 | | | |
| YIELDMAT | . | yes | v0.5 | | | |

## Engineering

All row-local (pure math). No streaming concerns.

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| BIN2DEC | x | yes | v0.3 | | x | |
| BIN2HEX | x | yes | v0.3 | | x | |
| BIN2OCT | x | yes | v0.3 | | x | |
| BITAND | x | yes | v0.3 | | | |
| BITLSHIFT | x | yes | v0.3 | | | |
| BITOR | x | yes | v0.3 | | | |
| BITRSHIFT | x | yes | v0.3 | | | |
| BITXOR | x | yes | v0.3 | | | |
| COMPLEX | x | yes | v0.3 | | | |
| CONVERT | x | yes | v0.3 | | | |
| DEC2BIN | x | yes | v0.3 | | x | |
| DEC2HEX | x | yes | v0.3 | | x | |
| DEC2OCT | x | yes | v0.3 | | x | |
| DELTA | x | yes | v0.3 | | | |
| ERF | x | yes | v0.3 | | | |
| ERF.PRECISE | x | yes | v0.3 | | | |
| ERFC | x | yes | v0.3 | | | |
| ERFC.PRECISE | x | yes | v0.3 | | | |
| GESTEP | x | yes | v0.3 | | | |
| HEX2BIN | x | yes | v0.3 | | x | |
| HEX2DEC | x | yes | v0.3 | | x | |
| HEX2OCT | x | yes | v0.3 | | x | |
| OCT2BIN | x | yes | v0.3 | | x | |
| OCT2DEC | x | yes | v0.3 | | x | |
| OCT2HEX | x | yes | v0.3 | | x | |
| BESSELI | . | yes | v0.4 | | | |
| BESSELJ | . | yes | v0.4 | | | |
| BESSELK | . | yes | v0.4 | | | |
| BESSELY | . | yes | v0.4 | | | |
| IMABS | . | yes | v0.4 | | | Complex number ops |
| IMAGINARY | x | yes | v0.3 | | | |
| IMARGUMENT | . | yes | v0.4 | | | |
| IMCONJUGATE | . | yes | v0.4 | | | |
| IMCOS | . | yes | v0.4 | | | |
| IMCOSH | . | yes | v0.4 | | | |
| IMCOT | . | yes | v0.4 | | | |
| IMCSC | . | yes | v0.4 | | | |
| IMCSCH | . | yes | v0.4 | | | |
| IMDIV | . | yes | v0.4 | | | |
| IMEXP | . | yes | v0.4 | | | |
| IMLN | . | yes | v0.4 | | | |
| IMLOG10 | . | yes | v0.4 | | | |
| IMLOG2 | . | yes | v0.4 | | | |
| IMPOWER | . | yes | v0.4 | | | |
| IMPRODUCT | . | yes | v0.4 | | | |
| IMREAL | x | yes | v0.3 | | | |
| IMSEC | . | yes | v0.4 | | | |
| IMSECH | . | yes | v0.4 | | | |
| IMSIN | . | yes | v0.4 | | | |
| IMSINH | . | yes | v0.4 | | | |
| IMSQRT | . | yes | v0.4 | | | |
| IMSUB | . | yes | v0.4 | | | |
| IMSUM | . | yes | v0.4 | | | |
| IMTAN | . | yes | v0.4 | | | |

## Database

All implementable as prelude aggregates with structured criteria parsing.

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| DAVERAGE | . | yes | v0.5 | | | |
| DCOUNT | . | yes | v0.5 | | | |
| DCOUNTA | . | yes | v0.5 | | | |
| DGET | . | yes | v0.5 | | | |
| DMAX | . | yes | v0.5 | | | |
| DMIN | . | yes | v0.5 | | | |
| DPRODUCT | . | yes | v0.5 | | | |
| DSTDEV | . | yes | v0.5 | | | |
| DSTDEVP | . | yes | v0.5 | | | |
| DSUM | . | yes | v0.5 | | | |
| DVAR | . | yes | v0.5 | | | |
| DVARP | . | yes | v0.5 | | | |

## Compatibility

Old function names. Thin wrappers over modern equivalents (implemented in v0.3/v0.4).

| Function | xlstream | Version | Maps to |
|---|---|---|---|
| BETADIST | . | v0.5 | BETA.DIST |
| BETAINV | . | v0.5 | BETA.INV |
| BINOMDIST | . | v0.5 | BINOM.DIST |
| CHIDIST | . | v0.5 | CHISQ.DIST.RT |
| CHIINV | . | v0.5 | CHISQ.INV.RT |
| CHITEST | . | v0.5 | CHISQ.TEST |
| CONFIDENCE | . | v0.5 | CONFIDENCE.NORM |
| COVAR | . | v0.5 | COVARIANCE.P |
| CRITBINOM | . | v0.5 | BINOM.INV |
| EXPONDIST | . | v0.5 | EXPON.DIST |
| FDIST | . | v0.5 | F.DIST.RT |
| FINV | . | v0.5 | F.INV.RT |
| FORECAST | . | v0.5 | FORECAST.LINEAR |
| FTEST | . | v0.5 | F.TEST |
| GAMMADIST | . | v0.5 | GAMMA.DIST |
| GAMMAINV | . | v0.5 | GAMMA.INV |
| HYPGEOMDIST | . | v0.5 | HYPGEOM.DIST |
| LOGINV | . | v0.5 | LOGNORM.INV |
| LOGNORMDIST | . | v0.5 | LOGNORM.DIST |
| MODE | . | v0.5 | MODE.SNGL |
| NEGBINOMDIST | . | v0.5 | NEGBINOM.DIST |
| NORMDIST | . | v0.5 | NORM.DIST |
| NORMINV | . | v0.5 | NORM.INV |
| NORMSDIST | . | v0.5 | NORM.S.DIST |
| NORMSINV | . | v0.5 | NORM.S.INV |
| PERCENTILE | . | v0.5 | PERCENTILE.INC |
| PERCENTRANK | . | v0.5 | PERCENTRANK.INC |
| POISSON | x | v0.3 | POISSON.DIST |
| QUARTILE | . | v0.5 | QUARTILE.INC |
| RANK | . | v0.5 | RANK.EQ |
| STDEV | . | v0.5 | STDEV.S |
| STDEVP | . | v0.5 | STDEV.P |
| TDIST | . | v0.5 | T.DIST.2T |
| TINV | . | v0.5 | T.INV.2T |
| TTEST | . | v0.5 | T.TEST |
| VAR | . | v0.5 | VAR.S |
| VARP | . | v0.5 | VAR.P |
| WEIBULL | . | v0.5 | WEIBULL.DIST |
| ZTEST | . | v0.5 | Z.TEST |

## Permanently excluded

| Function | Category | Why |
|---|---|---|
| OFFSET | Lookup | Runtime address resolution |
| INDIRECT | Lookup | Runtime address resolution |
| FILTER | Lookup | Dynamic array spill |
| SORT | Lookup | Dynamic array spill |
| SORTBY | Lookup | Dynamic array spill |
| UNIQUE | Lookup | Dynamic array spill |
| SEQUENCE | Math | Dynamic array spill |
| RAND | Math | Volatile, non-deterministic |
| RANDARRAY | Math | Volatile + spill |
| RANDBETWEEN | Math | Volatile, non-deterministic |
| LAMBDA | Logical | Closures, recursion |
| MAP | Logical | LAMBDA + spill |
| REDUCE | Logical | LAMBDA + spill |
| SCAN | Logical | LAMBDA + spill |
| BYROW | Logical | LAMBDA + spill |
| BYCOL | Logical | LAMBDA + spill |
| MAKEARRAY | Logical | LAMBDA + spill |
| TEXTSPLIT | Text | Spill |
| TRANSPOSE | Lookup | Spill |
| HSTACK / VSTACK | Lookup | Spill |
| TOCOL / TOROW | Lookup | Spill |
| CHOOSECOLS / CHOOSEROWS | Lookup | Spill |
| DROP / TAKE / EXPAND | Lookup | Spill |
| HYPERLINK | Lookup | Side-effecting |
| WEBSERVICE | Web | Network I/O |
| ENCODEURL | Web | Network context |
| FILTERXML | Web | Network context |
| CUBE* (7) | Cube | OLAP connection |
| CELL | Info | Runtime environment |
| INFO | Info | Runtime environment |
| ISFORMULA | Info | Runtime inspection |
| SHEET / SHEETS | Info | Runtime inspection |
| FORMULATEXT | Lookup | Runtime inspection |

## Summary

| Category | Excel total | xlstream now | Streamable | Excluded |
|---|---|---|---|---|
| Logical | 19 | 11 | 12 | 7 |
| Math & Trig | 82 | 30 | 78 | 4 |
| Statistical | 109 | 15 | 109 | 0 |
| Text | 49 | 19 | 48 | 1 |
| Date & Time | 25 | 12 | 25 | 0 |
| Lookup & Reference | 40 | 10 | 16 | 24 |
| Information | 22 | 10 | 17 | 5 |
| Financial | 55 | 6 | 55 | 0 |
| Engineering | 54 | 0 | 54 | 0 |
| Database | 12 | 0 | 12 | 0 |
| Compatibility | 39 | 0 | 39 | 0 |
| Cube/Web | 10 | 0 | 0 | 10 |
| **Total** | **~516** | **113** | **~465** | **~51** |

## Reference types

| Type | Status | Notes |
|---|---|---|
| Cell reference (`A2`, `Sheet1!B3`) | [x] | Row-local only |
| Range reference (`A:A`, `A1:B10`) | [x] | Aggregate/lookup context only |
| Named range (`MyRange`) | [x] | Resolved via `defined_names()` at classification time (v0.2) |
| Table reference (`Table1[Column]`) | [x] | Resolved at classification time (v0.2) |
| External reference (`[Book.xlsx]Sheet1!A1`) | - | Violates single-file model |

## Tallies

### Unsupported — 2 functions

| Function | Reason |
|---|---|
| RAND | Volatile; deterministic seeding deferred |
| RANDBETWEEN | Volatile; deterministic seeding deferred |

## How to request a new function

Open a GitHub issue with:
- The function name and Excel signature.
- A realistic use case.
- Why a workaround using existing functions isn't sufficient.

We evaluate against the project's "pure Excel only" rule and the streaming invariant.
