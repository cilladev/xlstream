//! CONVERT unit conversion function.
//!
//! Converts a number from one measurement unit to another across 13
//! categories: weight, distance, time, pressure, force, energy, power,
//! magnetism, temperature, volume, area, speed, information.

use xlstream_core::{coerce, CellError, Value};

use super::math::num_arg_ce;

// ---------------------------------------------------------------------------
// Category + unit representation
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Weight,
    Distance,
    Time,
    Pressure,
    Force,
    Energy,
    Power,
    Magnetism,
    Temperature,
    Volume,
    Area,
    Speed,
    Information,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
}

#[derive(Debug, Clone, Copy)]
enum UnitInfo {
    Standard { category: Category, factor: f64 },
    Temp(TempUnit),
}

impl UnitInfo {
    fn category(&self) -> Category {
        match self {
            Self::Standard { category, .. } => *category,
            Self::Temp(_) => Category::Temperature,
        }
    }

    fn accepts_prefix(&self) -> bool {
        !matches!(self, Self::Temp(_))
    }
}

// ---------------------------------------------------------------------------
// Base unit lookup (case-sensitive, ~100 entries + aliases)
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn lookup_base_unit(s: &str) -> Option<UnitInfo> {
    use Category::{
        Area, Distance, Energy, Force, Information, Magnetism, Power, Pressure, Speed, Time,
        Volume, Weight,
    };
    use TempUnit::{Celsius, Fahrenheit, Kelvin, Rankine, Reaumur};

    let info = match s {
        // -- Weight (base: kg) --
        "g" => UnitInfo::Standard { category: Weight, factor: 0.001 },
        "sg" => UnitInfo::Standard { category: Weight, factor: 14.593_902_94 },
        "lbm" => UnitInfo::Standard { category: Weight, factor: 0.453_592_37 },
        "u" => UnitInfo::Standard { category: Weight, factor: 1.660_538_921e-27 },
        "ozm" => UnitInfo::Standard { category: Weight, factor: 0.028_349_523_125 },
        "grain" => UnitInfo::Standard { category: Weight, factor: 0.000_064_798_91 },
        "cwt" | "shweight" => UnitInfo::Standard { category: Weight, factor: 45.359_237 },
        "uk_cwt" | "lcwt" | "hweight" => {
            UnitInfo::Standard { category: Weight, factor: 50.802_345_44 }
        }
        "stone" => UnitInfo::Standard { category: Weight, factor: 6.350_293_18 },
        "ton" => UnitInfo::Standard { category: Weight, factor: 907.184_74 },
        "uk_ton" | "LTON" | "brton" => {
            UnitInfo::Standard { category: Weight, factor: 1_016.046_908_8 }
        }

        // -- Distance (base: m) --
        "m" => UnitInfo::Standard { category: Distance, factor: 1.0 },
        "mi" => UnitInfo::Standard { category: Distance, factor: 1609.344 },
        "Nmi" => UnitInfo::Standard { category: Distance, factor: 1852.0 },
        "in" => UnitInfo::Standard { category: Distance, factor: 0.0254 },
        "ft" => UnitInfo::Standard { category: Distance, factor: 0.3048 },
        "yd" => UnitInfo::Standard { category: Distance, factor: 0.9144 },
        "ang" => UnitInfo::Standard { category: Distance, factor: 1e-10 },
        "ell" => UnitInfo::Standard { category: Distance, factor: 1.143 },
        "ly" => UnitInfo::Standard { category: Distance, factor: 9.460_730_472_58e15 },
        "parsec" | "pc" => UnitInfo::Standard { category: Distance, factor: 3.085_677_581_49e16 },
        "Picapt" | "Pica" => UnitInfo::Standard { category: Distance, factor: 0.000_352_777_78 },
        "pica" => UnitInfo::Standard { category: Distance, factor: 0.004_233_333_33 },
        "survey_mi" => UnitInfo::Standard { category: Distance, factor: 1_609.347_219 },

        // -- Time (base: s) --
        "yr" => UnitInfo::Standard { category: Time, factor: 31_557_600.0 },
        "day" => UnitInfo::Standard { category: Time, factor: 86_400.0 },
        "hr" => UnitInfo::Standard { category: Time, factor: 3600.0 },
        "mn" => UnitInfo::Standard { category: Time, factor: 60.0 },
        "sec" => UnitInfo::Standard { category: Time, factor: 1.0 },

        // -- Pressure (base: Pa) --
        "Pa" | "p" => UnitInfo::Standard { category: Pressure, factor: 1.0 },
        "atm" | "at" => UnitInfo::Standard { category: Pressure, factor: 101_325.0 },
        "mmHg" => UnitInfo::Standard { category: Pressure, factor: 133.322 },
        "psi" => UnitInfo::Standard { category: Pressure, factor: 6_894.757_293_168 },
        "Torr" => UnitInfo::Standard { category: Pressure, factor: 133.322_368_421_1 },

        // -- Force (base: N) --
        "N" => UnitInfo::Standard { category: Force, factor: 1.0 },
        "dyn" | "dy" => UnitInfo::Standard { category: Force, factor: 1e-5 },
        "lbf" => UnitInfo::Standard { category: Force, factor: 4.448_221_615_260_5 },
        "pond" => UnitInfo::Standard { category: Force, factor: 0.009_806_65 },

        // -- Energy (base: J) --
        "J" => UnitInfo::Standard { category: Energy, factor: 1.0 },
        "eV" | "ev" => UnitInfo::Standard { category: Energy, factor: 1.602_176_634e-19 },
        "cal" => UnitInfo::Standard { category: Energy, factor: 4.1868 },
        "c" => UnitInfo::Standard { category: Energy, factor: 4.184 },
        "BTU" | "btu" => UnitInfo::Standard { category: Energy, factor: 1_055.055_852_62 },
        "HPh" | "hh" => UnitInfo::Standard { category: Energy, factor: 2_684_519.536_885_6 },
        "Wh" | "wh" => UnitInfo::Standard { category: Energy, factor: 3600.0 },
        "flb" => UnitInfo::Standard { category: Energy, factor: 1.355_817_948_331_4 },

        // -- Power (base: W) --
        "W" | "w" => UnitInfo::Standard { category: Power, factor: 1.0 },
        "HP" | "h" => UnitInfo::Standard { category: Power, factor: 745.699_871_582_27 },
        "PS" => UnitInfo::Standard { category: Power, factor: 735.498_75 },

        // -- Magnetism (base: T) --
        "T" => UnitInfo::Standard { category: Magnetism, factor: 1.0 },
        "ga" => UnitInfo::Standard { category: Magnetism, factor: 1e-4 },

        // -- Temperature --
        "C" | "cel" => UnitInfo::Temp(Celsius),
        "F" | "fah" => UnitInfo::Temp(Fahrenheit),
        "K" | "kel" => UnitInfo::Temp(Kelvin),
        "Rank" => UnitInfo::Temp(Rankine),
        "Reau" => UnitInfo::Temp(Reaumur),

        // -- Volume (base: l) --
        "tsp" => UnitInfo::Standard { category: Volume, factor: 0.004_928_921_593_75 },
        "tspm" => UnitInfo::Standard { category: Volume, factor: 0.005 },
        "tbs" => UnitInfo::Standard { category: Volume, factor: 0.014_786_764_781_25 },
        "oz" => UnitInfo::Standard { category: Volume, factor: 0.029_573_529_562_5 },
        "cup" => UnitInfo::Standard { category: Volume, factor: 0.236_588_236_5 },
        "pt" | "us_pt" => UnitInfo::Standard { category: Volume, factor: 0.473_176_473 },
        "uk_pt" => UnitInfo::Standard { category: Volume, factor: 0.568_261_25 },
        "qt" => UnitInfo::Standard { category: Volume, factor: 0.946_352_946 },
        "uk_qt" => UnitInfo::Standard { category: Volume, factor: 1.136_522_5 },
        "gal" => UnitInfo::Standard { category: Volume, factor: 3.785_411_784 },
        "uk_gal" => UnitInfo::Standard { category: Volume, factor: 4.546_09 },
        "l" | "L" | "lt" => UnitInfo::Standard { category: Volume, factor: 1.0 },
        "ang3" | "ang^3" => UnitInfo::Standard { category: Volume, factor: 1e-27 },
        "barrel" => UnitInfo::Standard { category: Volume, factor: 158.987_294_928 },
        "bushel" => UnitInfo::Standard { category: Volume, factor: 35.239_070_17 },
        "ft3" | "ft^3" => UnitInfo::Standard { category: Volume, factor: 28.316_846_592 },
        "in3" | "in^3" => UnitInfo::Standard { category: Volume, factor: 0.016_387_064 },
        "ly3" | "ly^3" => UnitInfo::Standard { category: Volume, factor: 8.467_866_646_24e47 },
        "m3" | "m^3" => UnitInfo::Standard { category: Volume, factor: 1000.0 },
        "mi3" | "mi^3" => UnitInfo::Standard { category: Volume, factor: 4.168_181_825e12 },
        "yd3" | "yd^3" => UnitInfo::Standard { category: Volume, factor: 764.554_857_984 },
        "Nmi3" | "Nmi^3" => UnitInfo::Standard { category: Volume, factor: 6.352_182_208e9 },
        "Picapt3" | "Picapt^3" => UnitInfo::Standard { category: Volume, factor: 4.391_57e-11 },
        "GRT" | "regton" => UnitInfo::Standard { category: Volume, factor: 2_831.684_659_2 },
        "MTON" => UnitInfo::Standard { category: Volume, factor: 1_132.673_863_68 },

        // -- Area (base: m^2) --
        "uk_acre" => UnitInfo::Standard { category: Area, factor: 4_046.856_422_4 },
        "us_acre" => UnitInfo::Standard { category: Area, factor: 4_046.872_609_874 },
        "ang2" | "ang^2" => UnitInfo::Standard { category: Area, factor: 1e-20 },
        "ar" => UnitInfo::Standard { category: Area, factor: 100.0 },
        "ft2" | "ft^2" => UnitInfo::Standard { category: Area, factor: 0.092_903_04 },
        "ha" => UnitInfo::Standard { category: Area, factor: 10_000.0 },
        "in2" | "in^2" => UnitInfo::Standard { category: Area, factor: 0.000_645_16 },
        "ly2" | "ly^2" => UnitInfo::Standard { category: Area, factor: 8.950_542_107_5e31 },
        "m2" | "m^2" => UnitInfo::Standard { category: Area, factor: 1.0 },
        "Morgen" => UnitInfo::Standard { category: Area, factor: 2500.0 },
        "mi2" | "mi^2" => UnitInfo::Standard { category: Area, factor: 2_589_988.110_336 },
        "Nmi2" | "Nmi^2" => UnitInfo::Standard { category: Area, factor: 3_429_904.0 },
        "Picapt2" | "Picapt^2" => UnitInfo::Standard { category: Area, factor: 1.244_52e-7 },
        "yd2" | "yd^2" => UnitInfo::Standard { category: Area, factor: 0.836_127_36 },

        // -- Speed (base: m/s) --
        "admkn" => UnitInfo::Standard { category: Speed, factor: 0.514_773_333 },
        "kn" => UnitInfo::Standard { category: Speed, factor: 0.514_444_444 },
        "m/h" | "m/hr" => UnitInfo::Standard { category: Speed, factor: 1.0 / 3600.0 },
        "m/s" | "m/sec" => UnitInfo::Standard { category: Speed, factor: 1.0 },
        "mph" => UnitInfo::Standard { category: Speed, factor: 0.447_04 },

        // -- Information (base: bit) --
        "bit" => UnitInfo::Standard { category: Information, factor: 1.0 },
        "byte" => UnitInfo::Standard { category: Information, factor: 8.0 },

        _ => return None,
    };
    Some(info)
}

// ---------------------------------------------------------------------------
// SI and binary prefix tables
// ---------------------------------------------------------------------------

/// SI prefixes, ordered longest symbol first to avoid ambiguity.
const SI_PREFIXES: &[(&str, f64)] = &[
    ("da", 1e1),
    ("Yi", 1.208_925_819_615e24),
    ("Zi", 1.180_591_620_717e21),
    ("Gi", 1_073_741_824.0),
    ("Ti", 1_099_511_627_776.0),
    ("Mi", 1_048_576.0),
    ("Pi", 1_125_899_906_842_624.0),
    ("Ei", 1_152_921_504_606_846_976.0),
    ("ki", 1024.0),
    ("Y", 1e24),
    ("Z", 1e21),
    ("E", 1e18),
    ("P", 1e15),
    ("T", 1e12),
    ("G", 1e9),
    ("M", 1e6),
    ("k", 1e3),
    ("h", 1e2),
    ("d", 1e-1),
    ("c", 1e-2),
    ("m", 1e-3),
    ("u", 1e-6),
    ("n", 1e-9),
    ("p", 1e-12),
    ("f", 1e-15),
    ("a", 1e-18),
    ("z", 1e-21),
    ("y", 1e-24),
];

// ---------------------------------------------------------------------------
// Unit string parsing
// ---------------------------------------------------------------------------

/// Parse a unit string into a `UnitInfo`, trying exact match first,
/// then prefix stripping.
fn parse_unit(s: &str) -> Option<UnitInfo> {
    if let Some(info) = lookup_base_unit(s) {
        return Some(info);
    }

    for &(prefix, prefix_factor) in SI_PREFIXES {
        if let Some(remainder) = s.strip_prefix(prefix) {
            if remainder.is_empty() {
                continue;
            }
            if let Some(base) = lookup_base_unit(remainder) {
                if !base.accepts_prefix() {
                    return None;
                }
                if let UnitInfo::Standard { category, factor } = base {
                    return Some(UnitInfo::Standard { category, factor: factor * prefix_factor });
                }
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Temperature conversion
// ---------------------------------------------------------------------------

fn to_celsius(val: f64, from: TempUnit) -> f64 {
    match from {
        TempUnit::Celsius => val,
        TempUnit::Fahrenheit => (val - 32.0) * 5.0 / 9.0,
        TempUnit::Kelvin => val - 273.15,
        TempUnit::Rankine => val * 5.0 / 9.0 - 273.15,
        TempUnit::Reaumur => val * 5.0 / 4.0,
    }
}

fn from_celsius(c: f64, to: TempUnit) -> f64 {
    match to {
        TempUnit::Celsius => c,
        TempUnit::Fahrenheit => c * 9.0 / 5.0 + 32.0,
        TempUnit::Kelvin => c + 273.15,
        TempUnit::Rankine => (c + 273.15) * 9.0 / 5.0,
        TempUnit::Reaumur => c * 4.0 / 5.0,
    }
}

fn convert_temperature(val: f64, from: TempUnit, to: TempUnit) -> f64 {
    if from == to {
        return val;
    }
    from_celsius(to_celsius(val, from), to)
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// `CONVERT(number, from_unit, to_unit)` — convert between measurement units.
///
/// Supports ~100 base units across 13 categories (weight, distance, time,
/// pressure, force, energy, power, magnetism, temperature, volume, area,
/// speed, information). SI and binary prefixes multiply the base factor.
///
/// # Errors
///
/// Returns `#VALUE!` for wrong arity or non-numeric first argument.
/// Returns `#NUM!` for non-finite numeric input.
/// Returns `#N/A` for unknown units or cross-category conversion.
/// Propagates errors from any argument.
///
/// # Examples
///
/// ```
/// use xlstream_core::Value;
/// use xlstream_eval::builtins::convert::builtin_convert;
///
/// let result = builtin_convert(&[
///     Value::Number(1.0),
///     Value::Text("lbm".into()),
///     Value::Text("kg".into()),
/// ]);
/// assert!(matches!(result, Value::Number(n) if (n - 0.45359237).abs() < 1e-6));
/// ```
#[must_use]
pub fn builtin_convert(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value::Error(CellError::Value);
    }

    let number = match num_arg_ce(args, 0) {
        Ok(n) => n,
        Err(e) => return Value::Error(e),
    };
    if !number.is_finite() {
        return Value::Error(CellError::Num);
    }

    for arg in &args[1..3] {
        if let Value::Error(e) = arg {
            return Value::Error(*e);
        }
    }
    let from_str = coerce::to_text(&args[1]);
    let to_str = coerce::to_text(&args[2]);

    let Some(from_unit) = parse_unit(from_str.as_ref()) else {
        return Value::Error(CellError::Na);
    };
    let Some(to_unit) = parse_unit(to_str.as_ref()) else {
        return Value::Error(CellError::Na);
    };

    if from_unit.category() != to_unit.category() {
        return Value::Error(CellError::Na);
    }

    let result = match (from_unit, to_unit) {
        (UnitInfo::Temp(from), UnitInfo::Temp(to)) => convert_temperature(number, from, to),
        (UnitInfo::Standard { factor: from_f, .. }, UnitInfo::Standard { factor: to_f, .. }) => {
            number * (from_f / to_f)
        }
        _ => return Value::Error(CellError::Na),
    };

    Value::Number(result)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use xlstream_core::{CellError, Value};

    use super::*;

    fn convert(num: f64, from: &str, to: &str) -> Value {
        builtin_convert(&[Value::Number(num), Value::Text(from.into()), Value::Text(to.into())])
    }

    fn assert_num(val: Value, expected: f64) {
        match val {
            Value::Number(n) => {
                assert!(
                    (n - expected).abs() < 1e-6
                        || (expected != 0.0 && ((n - expected) / expected).abs() < 1e-9),
                    "expected {expected}, got {n}"
                );
            }
            other => panic!("expected Number({expected}), got {other:?}"),
        }
    }

    fn assert_error(val: &Value, err: CellError) {
        assert_eq!(*val, Value::Error(err), "expected error {err:?}");
    }

    // -- Basic conversions (one per category) --

    #[test]
    fn convert_weight() {
        assert_num(convert(1.0, "lbm", "kg"), 0.453_592_37);
    }

    #[test]
    fn convert_distance() {
        assert_num(convert(1.0, "mi", "km"), 1.609_344);
    }

    #[test]
    fn convert_time() {
        assert_num(convert(1.0, "day", "hr"), 24.0);
    }

    #[test]
    fn convert_pressure() {
        assert_num(convert(1.0, "atm", "Pa"), 101_325.0);
    }

    #[test]
    fn convert_force() {
        assert_num(convert(1.0, "lbf", "N"), 4.448_221_615_260_5);
    }

    #[test]
    fn convert_energy() {
        assert_num(convert(1.0, "BTU", "J"), 1_055.055_852_62);
    }

    #[test]
    fn convert_power() {
        assert_num(convert(1.0, "HP", "W"), 745.699_871_582_27);
    }

    #[test]
    fn convert_magnetism() {
        assert_num(convert(1.0, "T", "ga"), 10_000.0);
    }

    #[test]
    fn convert_volume() {
        assert_num(convert(1.0, "gal", "l"), 3.785_411_784);
    }

    #[test]
    fn convert_area() {
        assert_num(convert(1.0, "ha", "m2"), 10_000.0);
    }

    #[test]
    fn convert_speed() {
        assert_num(convert(1.0, "kn", "m/s"), 0.514_444_444);
    }

    #[test]
    fn convert_information() {
        assert_num(convert(1.0, "byte", "bit"), 8.0);
    }

    // -- Temperature --

    #[test]
    fn convert_celsius_to_fahrenheit_zero() {
        assert_num(convert(0.0, "C", "F"), 32.0);
    }

    #[test]
    fn convert_celsius_to_fahrenheit_100() {
        assert_num(convert(100.0, "C", "F"), 212.0);
    }

    #[test]
    fn convert_fahrenheit_to_celsius() {
        assert_num(convert(68.0, "F", "C"), 20.0);
    }

    #[test]
    fn convert_celsius_to_kelvin() {
        assert_num(convert(0.0, "C", "K"), 273.15);
    }

    #[test]
    fn convert_kelvin_to_celsius() {
        assert_num(convert(0.0, "K", "C"), -273.15);
    }

    #[test]
    fn convert_celsius_to_rankine() {
        assert_num(convert(0.0, "C", "Rank"), 491.67);
    }

    #[test]
    fn convert_celsius_to_reaumur() {
        assert_num(convert(0.0, "C", "Reau"), 0.0);
    }

    #[test]
    fn convert_reaumur_to_celsius() {
        assert_num(convert(80.0, "Reau", "C"), 100.0);
    }

    #[test]
    fn convert_minus_40_celsius_to_fahrenheit() {
        assert_num(convert(-40.0, "C", "F"), -40.0);
    }

    // -- SI prefixes --

    #[test]
    fn convert_meter_to_kilometer() {
        assert_num(convert(1.0, "m", "km"), 0.001);
    }

    #[test]
    fn convert_kilometer_to_meter() {
        assert_num(convert(1.0, "km", "m"), 1000.0);
    }

    #[test]
    fn convert_kilogram_to_gram() {
        assert_num(convert(1.0, "kg", "g"), 1000.0);
    }

    #[test]
    fn convert_milligram_to_gram() {
        assert_num(convert(1.0, "mg", "g"), 0.001);
    }

    #[test]
    fn convert_megawatt_to_watt() {
        assert_num(convert(1.0, "MW", "W"), 1_000_000.0);
    }

    #[test]
    fn convert_inch_to_centimeter() {
        assert_num(convert(1.0, "in", "cm"), 2.54);
    }

    // -- Binary prefixes --

    #[test]
    fn convert_byte_to_kbyte_si() {
        assert_num(convert(1024.0, "byte", "kbyte"), 1.024);
    }

    #[test]
    fn convert_mibibyte_to_byte() {
        assert_num(convert(1.0, "Mibyte", "byte"), 1_048_576.0);
    }

    #[test]
    fn convert_gibibit_to_bit() {
        assert_num(convert(1.0, "Gibit", "bit"), 1_073_741_824.0);
    }

    #[test]
    fn convert_kibibyte_to_byte() {
        assert_num(convert(1.0, "kibyte", "byte"), 1024.0);
    }

    // -- Same unit --

    #[test]
    fn convert_same_unit() {
        assert_num(convert(42.0, "m", "m"), 42.0);
    }

    // -- Unit aliases --

    #[test]
    fn convert_liter_aliases() {
        assert_num(convert(1.0, "l", "lt"), 1.0);
        assert_num(convert(1.0, "L", "l"), 1.0);
    }

    // -- Cross-category error --

    #[test]
    fn convert_cross_category_returns_na() {
        assert_error(&convert(1.0, "kg", "m"), CellError::Na);
        assert_error(&convert(1.0, "C", "m"), CellError::Na);
    }

    // -- Unknown unit --

    #[test]
    fn convert_unknown_unit_returns_na() {
        assert_error(&convert(1.0, "xyz", "m"), CellError::Na);
        assert_error(&convert(1.0, "m", "xyz"), CellError::Na);
    }

    // -- Temperature + SI prefix rejected --

    #[test]
    fn convert_temperature_with_si_prefix_returns_na() {
        assert_error(&convert(1.0, "kC", "F"), CellError::Na);
    }

    // -- Arity errors --

    #[test]
    fn convert_wrong_arity() {
        assert_error(
            &builtin_convert(&[Value::Number(1.0), Value::Text("m".into())]),
            CellError::Value,
        );
        assert_error(
            &builtin_convert(&[
                Value::Number(1.0),
                Value::Text("m".into()),
                Value::Text("km".into()),
                Value::Number(1.0),
            ]),
            CellError::Value,
        );
    }

    // -- Type errors --

    #[test]
    fn convert_non_numeric_first_arg_returns_value_error() {
        assert_error(
            &builtin_convert(&[
                Value::Text("abc".into()),
                Value::Text("m".into()),
                Value::Text("km".into()),
            ]),
            CellError::Value,
        );
    }

    #[test]
    fn convert_nan_returns_num_error() {
        assert_error(
            &builtin_convert(&[
                Value::Number(f64::NAN),
                Value::Text("m".into()),
                Value::Text("km".into()),
            ]),
            CellError::Num,
        );
    }

    #[test]
    fn convert_infinity_returns_num_error() {
        assert_error(
            &builtin_convert(&[
                Value::Number(f64::INFINITY),
                Value::Text("m".into()),
                Value::Text("km".into()),
            ]),
            CellError::Num,
        );
    }

    // -- Error propagation --

    #[test]
    fn convert_error_in_first_arg_propagates() {
        assert_error(
            &builtin_convert(&[
                Value::Error(CellError::Na),
                Value::Text("m".into()),
                Value::Text("km".into()),
            ]),
            CellError::Na,
        );
    }

    #[test]
    fn convert_error_in_second_arg_propagates() {
        assert_error(
            &builtin_convert(&[
                Value::Number(1.0),
                Value::Error(CellError::Na),
                Value::Text("km".into()),
            ]),
            CellError::Na,
        );
    }

    #[test]
    fn convert_error_in_third_arg_propagates() {
        assert_error(
            &builtin_convert(&[
                Value::Number(1.0),
                Value::Text("m".into()),
                Value::Error(CellError::Div0),
            ]),
            CellError::Div0,
        );
    }

    // -- Coercion --

    #[test]
    fn convert_bool_true_coerced_to_one() {
        assert_num(
            builtin_convert(&[
                Value::Bool(true),
                Value::Text("m".into()),
                Value::Text("km".into()),
            ]),
            0.001,
        );
    }

    #[test]
    fn convert_text_number_coerced() {
        assert_num(
            builtin_convert(&[
                Value::Text("100".into()),
                Value::Text("km".into()),
                Value::Text("mi".into()),
            ]),
            62.137_119_22,
        );
    }

    // -- Prefix collision cases --

    #[test]
    fn convert_exact_match_beats_prefix() {
        assert_num(convert(1.0, "ft", "m"), 0.3048);
        assert_num(convert(1.0, "T", "ga"), 10_000.0);
        assert_num(convert(1.0, "c", "J"), 4.184);
        assert_num(convert(1.0, "h", "W"), 745.699_871_582_27);
        assert_num(convert(1.0, "Pa", "atm"), 1.0 / 101_325.0);
    }

    // -- Additional coverage --

    #[test]
    fn convert_stone_to_lbm() {
        assert_num(convert(1.0, "stone", "lbm"), 6.350_293_18 / 0.453_592_37);
    }

    #[test]
    fn convert_ozm_to_g() {
        assert_num(convert(1.0, "ozm", "g"), 28.349_523_125);
    }

    #[test]
    fn convert_nautical_mile_to_meter() {
        assert_num(convert(1.0, "Nmi", "m"), 1852.0);
    }

    #[test]
    fn convert_year_to_day() {
        assert_num(convert(1.0, "yr", "day"), 365.25);
    }

    #[test]
    fn convert_atm_to_psi() {
        assert_num(convert(1.0, "atm", "psi"), 101_325.0 / 6_894.757_293_168);
    }

    #[test]
    fn convert_hectare_to_are() {
        assert_num(convert(1.0, "ha", "ar"), 100.0);
    }

    #[test]
    fn convert_mph_to_mps() {
        assert_num(convert(1.0, "mph", "m/s"), 0.447_04);
    }

    #[test]
    fn convert_temperature_aliases() {
        assert_num(convert(100.0, "cel", "fah"), 212.0);
        assert_num(convert(0.0, "kel", "cel"), -273.15);
    }
}
