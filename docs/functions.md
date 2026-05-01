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
| ACOSH | . | yes | v0.3 | x | | |
| ACOT | . | yes | v0.3 | | | |
| ACOTH | . | yes | v0.3 | | | |
| AGGREGATE | . | yes | v0.3 | | | Multi-mode aggregate |
| ARABIC | . | yes | v0.3 | | | |
| ASIN | x | yes | v0.1 | x | | |
| ASINH | . | yes | v0.3 | x | | |
| ATAN | x | yes | v0.1 | x | | |
| ATAN2 | x | yes | v0.1 | x | | |
| ATANH | . | yes | v0.3 | x | | |
| BASE | . | yes | v0.3 | | | |
| CEILING | x | yes | v0.1 | x | x | |
| CEILING.MATH | . | yes | v0.3 | | | |
| CEILING.PRECISE | . | yes | v0.3 | | | |
| COMBIN | . | yes | v0.3 | | | |
| COMBINA | . | yes | v0.3 | | | |
| COS | x | yes | v0.1 | x | | |
| COSH | . | yes | v0.3 | x | | |
| COT | . | yes | v0.3 | | | |
| COTH | . | yes | v0.3 | | | |
| CSC | . | yes | v0.3 | | | |
| CSCH | . | yes | v0.3 | | | |
| DECIMAL | . | yes | v0.3 | | | |
| DEGREES | . | yes | v0.3 | x | | |
| EVEN | . | yes | v0.3 | x | | |
| EXP | x | yes | v0.1 | x | | |
| FACT | . | yes | v0.3 | | | |
| FACTDOUBLE | . | yes | v0.3 | | | |
| FLOOR | x | yes | v0.1 | x | x | |
| FLOOR.MATH | . | yes | v0.3 | | | |
| FLOOR.PRECISE | . | yes | v0.3 | | | |
| GCD | . | yes | v0.3 | | | |
| INT | x | yes | v0.1 | x | x | |
| ISO.CEILING | . | yes | v0.3 | | | |
| LCM | . | yes | v0.3 | | | |
| LN | x | yes | v0.1 | x | x | |
| LOG | x | yes | v0.1 | x | x | |
| LOG10 | x | yes | v0.1 | x | | |
| MDETERM | . | yes | v0.3 | | | |
| MINVERSE | . | yes | v0.3 | | | |
| MMULT | . | yes | v0.3 | | | |
| MOD | x | yes | v0.1 | x | x | |
| MROUND | . | yes | v0.3 | | | |
| MULTINOMIAL | . | yes | v0.3 | | | |
| MUNIT | . | yes | v0.3 | | | |
| ODD | . | yes | v0.3 | x | | |
| PI | x | yes | v0.1 | x | x | |
| POWER | x | yes | v0.1 | x | x | |
| PRODUCT | x | yes | v0.1 | x | | |
| QUOTIENT | . | yes | v0.3 | | | |
| RADIANS | . | yes | v0.3 | x | | |
| RAND | - | no | - | x | x | Volatile |
| RANDARRAY | - | no | - | | | Volatile + spill |
| RANDBETWEEN | - | no | - | x | x | Volatile |
| ROMAN | . | yes | v0.3 | | | |
| ROUND | x | yes | v0.1 | x | x | |
| ROUNDDOWN | x | yes | v0.1 | x | x | |
| ROUNDUP | x | yes | v0.1 | x | x | |
| SEC | . | yes | v0.3 | | | |
| SECH | . | yes | v0.3 | | | |
| SEQUENCE | - | no | - | | | Spill |
| SERIESSUM | . | yes | v0.3 | | | |
| SIGN | x | yes | v0.1 | x | | |
| SIN | x | yes | v0.1 | x | | |
| SINH | . | yes | v0.3 | x | | |
| SQRT | x | yes | v0.1 | x | x | |
| SQRTPI | . | yes | v0.3 | | | |
| SUBTOTAL | . | yes | v0.3 | | | |
| SUM | x | yes | v0.1 | x | x | |
| SUMIF | x | yes | v0.1 | x | x | |
| SUMIFS | x | yes | v0.1 | x | | |
| SUMPRODUCT | x | yes | v0.2 | x | x | |
| SUMSQ | . | yes | v0.3 | | | |
| SUMX2MY2 | . | yes | v0.3 | | | |
| SUMX2PY2 | . | yes | v0.3 | | | |
| SUMXMY2 | . | yes | v0.3 | | | |
| TAN | x | yes | v0.1 | x | | |
| TANH | . | yes | v0.3 | x | | |
| TRUNC | . | yes | v0.3 | x | | |

## Statistical

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| AVERAGE | x | yes | v0.1 | x | x | |
| AVERAGEA | . | yes | v0.3 | | | |
| AVERAGEIF | x | yes | v0.1 | x | | |
| AVERAGEIFS | x | yes | v0.1 | x | | |
| AVEDEV | . | yes | v0.3 | | | |
| BETA.DIST | . | yes | v0.3 | | | |
| BETA.INV | . | yes | v0.3 | | | |
| BINOM.DIST | . | yes | v0.3 | | | |
| BINOM.DIST.RANGE | . | yes | v0.3 | | | |
| BINOM.INV | . | yes | v0.3 | | | |
| CHISQ.DIST | . | yes | v0.3 | | | |
| CHISQ.DIST.RT | . | yes | v0.3 | | | |
| CHISQ.INV | . | yes | v0.3 | | | |
| CHISQ.INV.RT | . | yes | v0.3 | | | |
| CHISQ.TEST | . | yes | v0.3 | | | |
| CONFIDENCE.NORM | . | yes | v0.3 | | | |
| CONFIDENCE.T | . | yes | v0.3 | | | |
| CORREL | . | yes | v0.3 | | | Prelude: two-column |
| COUNT | x | yes | v0.1 | x | x | |
| COUNTA | x | yes | v0.1 | x | x | |
| COUNTBLANK | x | yes | v0.1 | x | | |
| COUNTIF | x | yes | v0.1 | x | x | |
| COUNTIFS | x | yes | v0.1 | x | | |
| COVARIANCE.P | . | yes | v0.3 | | | |
| COVARIANCE.S | . | yes | v0.3 | | | |
| DEVSQ | . | yes | v0.3 | | | |
| EXPON.DIST | . | yes | v0.3 | | | |
| F.DIST | . | yes | v0.3 | | | |
| F.DIST.RT | . | yes | v0.3 | | | |
| F.INV | . | yes | v0.3 | | | |
| F.INV.RT | . | yes | v0.3 | | | |
| F.TEST | . | yes | v0.3 | | | |
| FISHER | . | yes | v0.3 | | | |
| FISHERINV | . | yes | v0.3 | | | |
| FORECAST.LINEAR | . | yes | v0.3 | | | |
| FREQUENCY | . | yes | v0.3 | | | |
| GAMMA | . | yes | v0.3 | | | |
| GAMMA.DIST | . | yes | v0.3 | | | |
| GAMMA.INV | . | yes | v0.3 | | | |
| GAMMALN | . | yes | v0.3 | | | |
| GAMMALN.PRECISE | . | yes | v0.3 | | | |
| GAUSS | . | yes | v0.3 | | | |
| GEOMEAN | . | yes | v0.3 | | | |
| GROWTH | . | yes | v0.3 | | | |
| HARMEAN | . | yes | v0.3 | | | |
| HYPGEOM.DIST | . | yes | v0.3 | | | |
| INTERCEPT | . | yes | v0.3 | | | |
| KURT | . | yes | v0.3 | | | |
| LARGE | . | yes | v0.3 | | | Prelude: sorted |
| LINEST | . | yes | v0.3 | | | |
| LOGEST | . | yes | v0.3 | | | |
| LOGNORM.DIST | . | yes | v0.3 | | | |
| LOGNORM.INV | . | yes | v0.3 | | | |
| MAX | x | yes | v0.1 | x | x | |
| MAXA | . | yes | v0.3 | | | |
| MAXIFS | x | yes | v0.2 | x | x | |
| MEDIAN | x | yes | v0.1 | x | | |
| MIN | x | yes | v0.1 | x | x | |
| MINA | . | yes | v0.3 | | | |
| MINIFS | x | yes | v0.2 | x | x | |
| MODE.SNGL | . | yes | v0.3 | | | |
| MODE.MULT | . | yes | v0.3 | | | |
| NEGBINOM.DIST | . | yes | v0.3 | | | |
| NORM.DIST | . | yes | v0.3 | | | |
| NORM.INV | . | yes | v0.3 | | | |
| NORM.S.DIST | . | yes | v0.3 | | | |
| NORM.S.INV | . | yes | v0.3 | | | |
| PEARSON | . | yes | v0.3 | | | |
| PERCENTILE.EXC | . | yes | v0.3 | | | |
| PERCENTILE.INC | . | yes | v0.3 | | | |
| PERCENTRANK.EXC | . | yes | v0.3 | | | |
| PERCENTRANK.INC | . | yes | v0.3 | | | |
| PERMUT | . | yes | v0.3 | | | |
| PERMUTATIONA | . | yes | v0.3 | | | |
| PHI | . | yes | v0.3 | | | |
| POISSON.DIST | . | yes | v0.3 | | | |
| PROB | . | yes | v0.3 | | | |
| QUARTILE.EXC | . | yes | v0.3 | | | |
| QUARTILE.INC | . | yes | v0.3 | | | |
| RANK.AVG | . | yes | v0.3 | | | |
| RANK.EQ | . | yes | v0.3 | | | |
| RSQ | . | yes | v0.3 | | | |
| SKEW | . | yes | v0.3 | | | |
| SKEW.P | . | yes | v0.3 | | | |
| SLOPE | . | yes | v0.3 | | | |
| SMALL | . | yes | v0.3 | | | |
| STANDARDIZE | . | yes | v0.3 | | | |
| STDEV.P | . | yes | v0.3 | | | |
| STDEV.S | . | yes | v0.3 | | | |
| STDEVA | . | yes | v0.3 | | | |
| STDEVPA | . | yes | v0.3 | | | |
| STEYX | . | yes | v0.3 | | | |
| T.DIST | . | yes | v0.3 | | | |
| T.DIST.2T | . | yes | v0.3 | | | |
| T.DIST.RT | . | yes | v0.3 | | | |
| T.INV | . | yes | v0.3 | | | |
| T.INV.2T | . | yes | v0.3 | | | |
| T.TEST | . | yes | v0.3 | | | |
| TREND | . | yes | v0.3 | | | |
| TRIMMEAN | . | yes | v0.3 | | | |
| VAR.P | . | yes | v0.3 | | | |
| VAR.S | . | yes | v0.3 | | | |
| VARA | . | yes | v0.3 | | | |
| VARPA | . | yes | v0.3 | | | |
| WEIBULL.DIST | . | yes | v0.3 | | | |
| Z.TEST | . | yes | v0.3 | | | |

## Text

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| CHAR | . | yes | v0.3 | | | |
| CLEAN | x | yes | v0.1 | x | | |
| CODE | . | yes | v0.3 | | | |
| CONCAT | x | yes | v0.1 | x | x | |
| CONCATENATE | x | yes | v0.1 | x | x | |
| DOLLAR | . | yes | v0.3 | | | |
| EXACT | x | yes | v0.1 | x | | |
| FIND | x | yes | v0.1 | x | x | |
| FINDB | . | yes | v0.3 | | | |
| FIXED | . | yes | v0.3 | | | |
| LEFT | x | yes | v0.1 | x | x | |
| LEFTB | . | yes | v0.3 | | | |
| LEN | x | yes | v0.1 | x | x | |
| LENB | . | yes | v0.3 | | | |
| LOWER | x | yes | v0.1 | x | x | |
| MID | x | yes | v0.1 | x | x | |
| MIDB | . | yes | v0.3 | | | |
| NUMBERVALUE | . | yes | v0.3 | | | |
| PROPER | x | yes | v0.1 | x | | |
| REPLACE | x | yes | v0.1 | x | | |
| REPLACEB | . | yes | v0.3 | | | |
| REPT | . | yes | v0.3 | | | |
| RIGHT | x | yes | v0.1 | x | x | |
| RIGHTB | . | yes | v0.3 | | | |
| SEARCH | x | yes | v0.1 | x | | |
| SEARCHB | . | yes | v0.3 | | | |
| SUBSTITUTE | x | yes | v0.1 | x | | |
| T | . | yes | v0.3 | | | |
| TEXT | x | yes | v0.1 | x | | |
| TEXTAFTER | . | yes | v0.3 | | | |
| TEXTBEFORE | . | yes | v0.3 | | | |
| TEXTJOIN | x | yes | v0.1 | x | | |
| TEXTSPLIT | - | no | - | | | Spill |
| TRIM | x | yes | v0.1 | x | x | |
| UNICHAR | . | yes | v0.3 | | | |
| UNICODE | . | yes | v0.3 | | | |
| UPPER | x | yes | v0.1 | x | x | |
| VALUE | x | yes | v0.1 | x | x | |
| VALUETOTEXT | . | yes | v0.3 | | | |

## Date & Time

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| DATE | x | yes | v0.1 | x | x | |
| DATEDIF | x | yes | v0.1 | x | | |
| DATEVALUE | . | yes | v0.3 | | | |
| DAY | x | yes | v0.1 | x | x | |
| DAYS | . | yes | v0.3 | | | |
| DAYS360 | . | yes | v0.3 | | | |
| EDATE | x | yes | v0.1 | x | x | |
| EOMONTH | x | yes | v0.1 | x | x | |
| HOUR | . | yes | v0.3 | | | |
| ISOWEEKNUM | . | yes | v0.3 | x | | |
| MINUTE | . | yes | v0.3 | | | |
| MONTH | x | yes | v0.1 | x | x | |
| NETWORKDAYS | x | yes | v0.1 | x | | |
| NETWORKDAYS.INTL | . | yes | v0.3 | | | |
| NOW | x | yes | v0.1 | x | x | Once per run |
| SECOND | . | yes | v0.3 | | | |
| TIME | . | yes | v0.3 | | | |
| TIMEVALUE | . | yes | v0.3 | | | |
| TODAY | x | yes | v0.1 | x | x | Once per run |
| WEEKDAY | x | yes | v0.1 | x | x | |
| WEEKNUM | . | yes | v0.3 | | | |
| WORKDAY | x | yes | v0.1 | x | | |
| WORKDAY.INTL | . | yes | v0.3 | | | |
| YEAR | x | yes | v0.1 | x | x | |
| YEARFRAC | . | yes | v0.3 | x | x | |

## Lookup & Reference

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| ADDRESS | . | yes | v0.3 | | | Builds A1 string |
| CHOOSE | x | yes | v0.1 | x | | |
| COLUMN | . | yes | v0.2 | | | |
| COLUMNS | . | yes | v0.2 | | | |
| HLOOKUP | x | yes | v0.1 | x | | |
| INDEX | x | yes | v0.1 | x | x | |
| LOOKUP | . | yes | v0.3 | x | | Legacy |
| MATCH | x | yes | v0.1 | x | x | |
| ROW | . | yes | v0.3 | | | |
| ROWS | . | yes | v0.2 | | | |
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
| ERROR.TYPE | . | yes | v0.3 | | | |
| ISERR | . | yes | v0.3 | | | |
| ISEVEN | . | yes | v0.3 | | | |
| ISODD | . | yes | v0.3 | | | |
| N | . | yes | v0.3 | | | |
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
| ACCRINT | . | yes | v0.4 | | | |
| ACCRINTM | . | yes | v0.4 | | | |
| CUMIPMT | . | yes | v0.4 | | | |
| CUMPRINC | . | yes | v0.4 | | | |
| DB | . | yes | v0.4 | | | |
| DDB | . | yes | v0.4 | | | |
| DISC | . | yes | v0.4 | | | |
| DOLLARDE | . | yes | v0.4 | | | |
| DOLLARFR | . | yes | v0.4 | | | |
| DURATION | . | yes | v0.4 | | | |
| EFFECT | . | yes | v0.4 | | | |
| FVSCHEDULE | . | yes | v0.4 | | | |
| INTRATE | . | yes | v0.4 | | | |
| IPMT | . | yes | v0.4 | | | |
| ISPMT | . | yes | v0.4 | | | |
| MDURATION | . | yes | v0.4 | | | |
| MIRR | . | yes | v0.4 | | | |
| NOMINAL | . | yes | v0.4 | | | |
| NPER | . | yes | v0.4 | | | |
| PDURATION | . | yes | v0.4 | | | |
| PPMT | . | yes | v0.4 | | | |
| PRICE | . | yes | v0.4 | | | |
| PRICEDISC | . | yes | v0.4 | | | |
| PRICEMAT | . | yes | v0.4 | | | |
| RECEIVED | . | yes | v0.4 | | | |
| RRI | . | yes | v0.4 | | | |
| SLN | . | yes | v0.4 | | | |
| SYD | . | yes | v0.4 | | | |
| TBILLEQ | . | yes | v0.4 | | | |
| TBILLPRICE | . | yes | v0.4 | | | |
| TBILLYIELD | . | yes | v0.4 | | | |
| VDB | . | yes | v0.4 | | | |
| XIRR | . | yes | v0.4 | | | |
| XNPV | . | yes | v0.4 | | | |
| YIELD | . | yes | v0.4 | | | |
| YIELDDISC | . | yes | v0.4 | | | |
| YIELDMAT | . | yes | v0.4 | | | |

## Engineering

All row-local (pure math). No streaming concerns.

| Function | xlstream | Streamable | Version | formualizer | xlcalculator | Notes |
|---|---|---|---|---|---|---|
| BIN2DEC | . | yes | v0.3 | | x | |
| BIN2HEX | . | yes | v0.3 | | x | |
| BIN2OCT | . | yes | v0.3 | | x | |
| BITAND | . | yes | v0.3 | | | |
| BITLSHIFT | . | yes | v0.3 | | | |
| BITOR | . | yes | v0.3 | | | |
| BITRSHIFT | . | yes | v0.3 | | | |
| BITXOR | . | yes | v0.3 | | | |
| COMPLEX | . | yes | v0.3 | | | |
| CONVERT | . | yes | v0.3 | | | |
| DEC2BIN | . | yes | v0.3 | | x | |
| DEC2HEX | . | yes | v0.3 | | x | |
| DEC2OCT | . | yes | v0.3 | | x | |
| DELTA | . | yes | v0.3 | | | |
| ERF | . | yes | v0.3 | | | |
| ERF.PRECISE | . | yes | v0.3 | | | |
| ERFC | . | yes | v0.3 | | | |
| ERFC.PRECISE | . | yes | v0.3 | | | |
| GESTEP | . | yes | v0.3 | | | |
| HEX2BIN | . | yes | v0.3 | | x | |
| HEX2DEC | . | yes | v0.3 | | x | |
| HEX2OCT | . | yes | v0.3 | | x | |
| OCT2BIN | . | yes | v0.3 | | x | |
| OCT2DEC | . | yes | v0.3 | | x | |
| OCT2HEX | . | yes | v0.3 | | x | |
| BESSELI | . | yes | v0.3 | | | |
| BESSELJ | . | yes | v0.3 | | | |
| BESSELK | . | yes | v0.3 | | | |
| BESSELY | . | yes | v0.3 | | | |
| IMABS | . | yes | v0.3 | | | Complex number ops |
| IMAGINARY | . | yes | v0.3 | | | |
| IMARGUMENT | . | yes | v0.3 | | | |
| IMCONJUGATE | . | yes | v0.3 | | | |
| IMCOS | . | yes | v0.3 | | | |
| IMCOSH | . | yes | v0.3 | | | |
| IMCOT | . | yes | v0.3 | | | |
| IMCSC | . | yes | v0.3 | | | |
| IMCSCH | . | yes | v0.3 | | | |
| IMDIV | . | yes | v0.3 | | | |
| IMEXP | . | yes | v0.3 | | | |
| IMLN | . | yes | v0.3 | | | |
| IMLOG10 | . | yes | v0.3 | | | |
| IMLOG2 | . | yes | v0.3 | | | |
| IMPOWER | . | yes | v0.3 | | | |
| IMPRODUCT | . | yes | v0.3 | | | |
| IMREAL | . | yes | v0.3 | | | |
| IMSEC | . | yes | v0.3 | | | |
| IMSECH | . | yes | v0.3 | | | |
| IMSIN | . | yes | v0.3 | | | |
| IMSINH | . | yes | v0.3 | | | |
| IMSQRT | . | yes | v0.3 | | | |
| IMSUB | . | yes | v0.3 | | | |
| IMSUM | . | yes | v0.3 | | | |
| IMTAN | . | yes | v0.3 | | | |

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
| POISSON | . | v0.5 | POISSON.DIST |
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

## How to add a new function

1. Pick a home — category in this file, matching phase doc.
2. Implement in the right `xlstream-eval/src/builtins/*.rs` module.
3. Add a match arm in `dispatch()` (`builtins/mod.rs`).
4. Write >= 5 unit tests (happy, empty, error-propagation, coercion, edge case).
5. Tick the box here in the same PR.
6. Update `CHANGELOG.md`.

## How to promote a v0.2 function to v0.1

Open an issue with:
- The function name.
- Why it's needed for v0.1 release (real use case, not "would be nice").
- Estimate of implementation + test effort.

Decision rule: we promote only if leaving it out would break a common workbook shape the release is meant to handle.

## How to request a new function

Open a GitHub issue with:
- The function name and Excel signature.
- A realistic use case.
- Why a workaround using existing functions isn't sufficient.

We evaluate against the project's "pure Excel only" rule and the streaming invariant.
