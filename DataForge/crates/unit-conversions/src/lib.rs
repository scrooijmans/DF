//! Unit conversion utilities for petrophysical well log data.
//!
//! This crate provides type-safe unit definitions and conversions for common
//! measurements in well logging, including depth, density, porosity, resistivity,
//! sonic transit time, and caliper measurements.
//!
//! # Design Philosophy
//!
//! Units are represented as strongly-typed enums rather than strings to provide:
//! - Compile-time safety
//! - Exhaustive pattern matching
//! - Zero-cost abstractions
//!
//! # Example
//!
//! ```
//! use unit_conversions::{DepthUnit, convert_curve_value};
//!
//! // Convert 100 feet to meters
//! let meters = DepthUnit::convert(100.0, DepthUnit::Feet, DepthUnit::Meters);
//! assert!((meters - 30.48).abs() < 0.001);
//!
//! // Type-safe conversion using curve type
//! let result = convert_curve_value(100.0, "DEPTH", "ft", "m").unwrap();
//! assert!((result - 30.48).abs() < 0.001);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors that can occur during unit conversion.
#[derive(Debug, Error)]
pub enum UnitConversionError {
    /// Unknown unit for the given curve type
    #[error("Unknown {curve_type} unit: {unit}")]
    UnknownUnit { curve_type: String, unit: String },

    /// No conversion available between units
    #[error("No conversion available for curve type '{curve_type}' from '{from}' to '{to}'")]
    NoConversion {
        curve_type: String,
        from: String,
        to: String,
    },
}

/// Depth measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DepthUnit {
    /// Feet (default for US oil industry)
    #[default]
    #[serde(rename = "ft")]
    Feet,
    /// Meters (SI unit)
    #[serde(rename = "m")]
    Meters,
}

impl fmt::Display for DepthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DepthUnit::Feet => write!(f, "ft"),
            DepthUnit::Meters => write!(f, "m"),
        }
    }
}

impl DepthUnit {
    /// Parse a depth unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ft" | "feet" | "foot" => Some(DepthUnit::Feet),
            "m" | "meters" | "meter" => Some(DepthUnit::Meters),
            _ => None,
        }
    }

    /// Convert a value from one depth unit to another.
    pub fn convert(value: f64, from: DepthUnit, to: DepthUnit) -> f64 {
        if from == to {
            return value;
        }
        match (from, to) {
            (DepthUnit::Feet, DepthUnit::Meters) => value * 0.3048,
            (DepthUnit::Meters, DepthUnit::Feet) => value / 0.3048,
            _ => value,
        }
    }
}

/// Gamma ray measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GammaRayUnit {
    /// API gamma ray units (standard)
    #[default]
    #[serde(rename = "gAPI")]
    Gapi,
}

impl fmt::Display for GammaRayUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GammaRayUnit::Gapi => write!(f, "gAPI"),
        }
    }
}

/// Density measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DensityUnit {
    /// Grams per cubic centimeter (standard for well logs)
    #[default]
    #[serde(rename = "g/cm³")]
    GPerCm3,
    /// Kilograms per cubic meter (SI unit)
    #[serde(rename = "kg/m³")]
    KgPerM3,
}

impl fmt::Display for DensityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DensityUnit::GPerCm3 => write!(f, "g/cm³"),
            DensityUnit::KgPerM3 => write!(f, "kg/m³"),
        }
    }
}

impl DensityUnit {
    /// Parse a density unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "g/cm³" | "g/cm3" | "g/cc" => Some(DensityUnit::GPerCm3),
            "kg/m³" | "kg/m3" => Some(DensityUnit::KgPerM3),
            _ => None,
        }
    }

    /// Convert a value from one density unit to another.
    pub fn convert(value: f64, from: DensityUnit, to: DensityUnit) -> f64 {
        if from == to {
            return value;
        }
        match (from, to) {
            (DensityUnit::GPerCm3, DensityUnit::KgPerM3) => value * 1000.0,
            (DensityUnit::KgPerM3, DensityUnit::GPerCm3) => value / 1000.0,
            _ => value,
        }
    }
}

/// Porosity measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PorosityUnit {
    /// Volume per volume (decimal fraction)
    #[default]
    #[serde(rename = "v/v")]
    VolumePerVolume,
    /// Decimal fraction (0-1)
    #[serde(rename = "fraction")]
    Fraction,
    /// Percentage (0-100)
    #[serde(rename = "%")]
    Percent,
}

impl fmt::Display for PorosityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PorosityUnit::VolumePerVolume => write!(f, "v/v"),
            PorosityUnit::Fraction => write!(f, "fraction"),
            PorosityUnit::Percent => write!(f, "%"),
        }
    }
}

impl PorosityUnit {
    /// Parse a porosity unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "v/v" | "volume/volume" => Some(PorosityUnit::VolumePerVolume),
            "fraction" => Some(PorosityUnit::Fraction),
            "%" | "percent" => Some(PorosityUnit::Percent),
            _ => None,
        }
    }

    /// Convert a value from one porosity unit to another.
    pub fn convert(value: f64, from: PorosityUnit, to: PorosityUnit) -> f64 {
        if from == to {
            return value;
        }
        match (from, to) {
            (PorosityUnit::Percent, PorosityUnit::VolumePerVolume) => value / 100.0,
            (PorosityUnit::VolumePerVolume, PorosityUnit::Percent) => value * 100.0,
            (PorosityUnit::Fraction, PorosityUnit::VolumePerVolume) => value,
            (PorosityUnit::VolumePerVolume, PorosityUnit::Fraction) => value,
            (PorosityUnit::Percent, PorosityUnit::Fraction) => value / 100.0,
            (PorosityUnit::Fraction, PorosityUnit::Percent) => value * 100.0,
            _ => value,
        }
    }
}

/// Resistivity measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ResistivityUnit {
    /// Ohm-meters (standard for well logs)
    #[default]
    #[serde(rename = "ohm-m")]
    OhmM,
    /// Ohm-centimeters
    #[serde(rename = "ohm-cm")]
    OhmCm,
}

impl fmt::Display for ResistivityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResistivityUnit::OhmM => write!(f, "ohm-m"),
            ResistivityUnit::OhmCm => write!(f, "ohm-cm"),
        }
    }
}

impl ResistivityUnit {
    /// Parse a resistivity unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ohm-m" | "ohm.m" | "ohmm" => Some(ResistivityUnit::OhmM),
            "ohm-cm" | "ohm.cm" | "ohmcm" => Some(ResistivityUnit::OhmCm),
            _ => None,
        }
    }

    /// Convert a value from one resistivity unit to another.
    pub fn convert(value: f64, from: ResistivityUnit, to: ResistivityUnit) -> f64 {
        if from == to {
            return value;
        }
        match (from, to) {
            (ResistivityUnit::OhmCm, ResistivityUnit::OhmM) => value / 100.0,
            (ResistivityUnit::OhmM, ResistivityUnit::OhmCm) => value * 100.0,
            _ => value,
        }
    }
}

/// Sonic transit time measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SonicUnit {
    /// Microseconds per foot (standard for US)
    #[default]
    #[serde(rename = "µs/ft")]
    MicrosecondsPerFoot,
    /// Microseconds per meter
    #[serde(rename = "µs/m")]
    MicrosecondsPerMeter,
    /// Alternative notation for microseconds per foot
    #[serde(rename = "us/ft")]
    UsPerFoot,
    /// Alternative notation for microseconds per meter
    #[serde(rename = "us/m")]
    UsPerMeter,
}

impl fmt::Display for SonicUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SonicUnit::MicrosecondsPerFoot => write!(f, "µs/ft"),
            SonicUnit::MicrosecondsPerMeter => write!(f, "µs/m"),
            SonicUnit::UsPerFoot => write!(f, "us/ft"),
            SonicUnit::UsPerMeter => write!(f, "us/m"),
        }
    }
}

impl SonicUnit {
    /// Parse a sonic unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        // Normalize both µ (U+00B5 micro sign) and μ (U+03BC Greek mu)
        let normalized = s.replace('μ', "µ").replace('Μ', "µ").to_lowercase();
        match normalized.as_str() {
            "µs/ft" | "us/ft" | "microseconds/foot" => Some(SonicUnit::MicrosecondsPerFoot),
            "µs/m" | "us/m" | "microseconds/meter" => Some(SonicUnit::MicrosecondsPerMeter),
            _ => None,
        }
    }

    /// Convert a value from one sonic unit to another.
    pub fn convert(value: f64, from: SonicUnit, to: SonicUnit) -> f64 {
        if from == to {
            return value;
        }
        // Normalize to microseconds per foot first
        let normalized = match from {
            SonicUnit::MicrosecondsPerMeter | SonicUnit::UsPerMeter => value * 0.3048,
            SonicUnit::MicrosecondsPerFoot | SonicUnit::UsPerFoot => value,
        };

        match to {
            SonicUnit::MicrosecondsPerMeter | SonicUnit::UsPerMeter => normalized / 0.3048,
            SonicUnit::MicrosecondsPerFoot | SonicUnit::UsPerFoot => normalized,
        }
    }
}

/// Caliper measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CaliperUnit {
    /// Inches (standard for US)
    #[default]
    #[serde(rename = "inches")]
    Inches,
    /// Centimeters
    #[serde(rename = "cm")]
    Centimeters,
    /// Millimeters
    #[serde(rename = "mm")]
    Millimeters,
}

impl fmt::Display for CaliperUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaliperUnit::Inches => write!(f, "inches"),
            CaliperUnit::Centimeters => write!(f, "cm"),
            CaliperUnit::Millimeters => write!(f, "mm"),
        }
    }
}

impl CaliperUnit {
    /// Parse a caliper unit from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "inches" | "inch" | "in" => Some(CaliperUnit::Inches),
            "cm" | "centimeters" | "centimeter" => Some(CaliperUnit::Centimeters),
            "mm" | "millimeters" | "millimeter" => Some(CaliperUnit::Millimeters),
            _ => None,
        }
    }

    /// Convert a value from one caliper unit to another.
    pub fn convert(value: f64, from: CaliperUnit, to: CaliperUnit) -> f64 {
        if from == to {
            return value;
        }
        // Normalize to inches first
        let normalized = match from {
            CaliperUnit::Centimeters => value / 2.54,
            CaliperUnit::Millimeters => value / 25.4,
            CaliperUnit::Inches => value,
        };

        match to {
            CaliperUnit::Centimeters => normalized * 2.54,
            CaliperUnit::Millimeters => normalized * 25.4,
            CaliperUnit::Inches => normalized,
        }
    }
}

/// Spontaneous potential measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SpontaneousPotentialUnit {
    /// Millivolts (standard)
    #[default]
    #[serde(rename = "mV")]
    Millivolts,
}

impl fmt::Display for SpontaneousPotentialUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpontaneousPotentialUnit::Millivolts => write!(f, "mV"),
        }
    }
}

/// Photo-electric factor measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PhotoElectricUnit {
    /// Barns per electron (standard)
    #[default]
    #[serde(rename = "b/e")]
    BarnsPerElectron,
}

impl fmt::Display for PhotoElectricUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhotoElectricUnit::BarnsPerElectron => write!(f, "b/e"),
        }
    }
}

/// Standard units for each curve type.
pub mod standard_units {
    use super::*;

    /// Standard depth unit
    pub const DEPTH: DepthUnit = DepthUnit::Feet;
    /// Standard gamma ray unit
    pub const GAMMA_RAY: GammaRayUnit = GammaRayUnit::Gapi;
    /// Standard density unit
    pub const DENSITY: DensityUnit = DensityUnit::GPerCm3;
    /// Standard porosity unit
    pub const POROSITY: PorosityUnit = PorosityUnit::VolumePerVolume;
    /// Standard resistivity unit
    pub const RESISTIVITY: ResistivityUnit = ResistivityUnit::OhmM;
    /// Standard sonic unit
    pub const SONIC: SonicUnit = SonicUnit::MicrosecondsPerFoot;
    /// Standard caliper unit
    pub const CALIPER: CaliperUnit = CaliperUnit::Inches;
    /// Standard spontaneous potential unit
    pub const SPONTANEOUS_POTENTIAL: SpontaneousPotentialUnit =
        SpontaneousPotentialUnit::Millivolts;
    /// Standard photo-electric unit
    pub const PHOTO_ELECTRIC: PhotoElectricUnit = PhotoElectricUnit::BarnsPerElectron;
}

/// Get available units for a main curve type.
pub fn get_available_units_for_curve_type(curve_type: &str) -> Vec<String> {
    match curve_type {
        "GR" => vec!["gAPI".to_string()],
        "RHOB" => vec!["g/cm³".to_string(), "kg/m³".to_string()],
        "NPHI" | "POR" => vec!["v/v".to_string(), "fraction".to_string(), "%".to_string()],
        "RT" => vec!["ohm-m".to_string(), "ohm-cm".to_string()],
        "DT" | "DTC" | "DTS" => vec!["µs/ft".to_string(), "µs/m".to_string()],
        "CALI" => vec!["inches".to_string(), "cm".to_string(), "mm".to_string()],
        "SP" => vec!["mV".to_string()],
        "PE" => vec!["b/e".to_string()],
        "DEPTH" | "DEPT" | "MD" | "TVD" => vec!["ft".to_string(), "m".to_string()],
        _ => vec![],
    }
}

/// Convert a curve value based on curve type and units.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `curve_type` - The type of curve (e.g., "DEPTH", "RHOB", "GR")
/// * `from_unit` - The source unit string
/// * `to_unit` - The target unit string
///
/// # Returns
///
/// The converted value or an error if conversion is not available.
pub fn convert_curve_value(
    value: f64,
    curve_type: &str,
    from_unit: &str,
    to_unit: &str,
) -> Result<f64, UnitConversionError> {
    // Normalize sonic units (handle both µ and μ characters)
    let normalized_from = from_unit.replace('μ', "µ").replace('Μ', "µ");
    let normalized_to = to_unit.replace('μ', "µ").replace('Μ', "µ");

    match curve_type {
        "DEPTH" | "DEPT" | "MD" | "TVD" => {
            let from = DepthUnit::from_str(&normalized_from).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "depth".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = DepthUnit::from_str(&normalized_to).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "depth".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(DepthUnit::convert(value, from, to))
        }
        "OTHER" => {
            // For OTHER type, try to infer from units
            if let (Some(from_depth), Some(to_depth)) = (
                DepthUnit::from_str(&normalized_from),
                DepthUnit::from_str(&normalized_to),
            ) {
                return Ok(DepthUnit::convert(value, from_depth, to_depth));
            }
            if let (Some(from_sonic), Some(to_sonic)) = (
                SonicUnit::from_str(&normalized_from),
                SonicUnit::from_str(&normalized_to),
            ) {
                return Ok(SonicUnit::convert(value, from_sonic, to_sonic));
            }
            if normalized_from == normalized_to {
                Ok(value)
            } else {
                Err(UnitConversionError::NoConversion {
                    curve_type: curve_type.to_string(),
                    from: from_unit.to_string(),
                    to: to_unit.to_string(),
                })
            }
        }
        "RHOB" => {
            let from = DensityUnit::from_str(from_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "density".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = DensityUnit::from_str(to_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "density".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(DensityUnit::convert(value, from, to))
        }
        "NPHI" | "POR" | "PHIE" => {
            let from = PorosityUnit::from_str(from_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "porosity".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = PorosityUnit::from_str(to_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "porosity".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(PorosityUnit::convert(value, from, to))
        }
        "RT" | "RT_DEEP" | "RT_SHALLOW" | "RT_MEDIUM" | "RT_MICRO" => {
            let from = ResistivityUnit::from_str(from_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "resistivity".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = ResistivityUnit::from_str(to_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "resistivity".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(ResistivityUnit::convert(value, from, to))
        }
        "DT" | "DTC" | "DTS" => {
            let from = SonicUnit::from_str(&normalized_from).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "sonic".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = SonicUnit::from_str(&normalized_to).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "sonic".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(SonicUnit::convert(value, from, to))
        }
        "CALI" => {
            let from = CaliperUnit::from_str(from_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "caliper".to_string(),
                    unit: from_unit.to_string(),
                }
            })?;
            let to = CaliperUnit::from_str(to_unit).ok_or_else(|| {
                UnitConversionError::UnknownUnit {
                    curve_type: "caliper".to_string(),
                    unit: to_unit.to_string(),
                }
            })?;
            Ok(CaliperUnit::convert(value, from, to))
        }
        _ => {
            // For curves without conversion (GR, SP, PE), return as-is if units match
            if from_unit == to_unit {
                Ok(value)
            } else {
                Err(UnitConversionError::NoConversion {
                    curve_type: curve_type.to_string(),
                    from: from_unit.to_string(),
                    to: to_unit.to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_depth_conversion_feet_to_meters() {
        let feet = 100.0;
        let meters = DepthUnit::convert(feet, DepthUnit::Feet, DepthUnit::Meters);
        assert_relative_eq!(meters, 30.48, epsilon = 0.001);
    }

    #[test]
    fn test_depth_conversion_meters_to_feet() {
        let meters = 30.48;
        let feet = DepthUnit::convert(meters, DepthUnit::Meters, DepthUnit::Feet);
        assert_relative_eq!(feet, 100.0, epsilon = 0.001);
    }

    #[test]
    fn test_depth_same_unit() {
        let value = 100.0;
        let result = DepthUnit::convert(value, DepthUnit::Feet, DepthUnit::Feet);
        assert_relative_eq!(result, value, epsilon = 0.001);
    }

    #[test]
    fn test_density_conversion() {
        let g_per_cm3 = 2.5;
        let kg_per_m3 = DensityUnit::convert(g_per_cm3, DensityUnit::GPerCm3, DensityUnit::KgPerM3);
        assert_relative_eq!(kg_per_m3, 2500.0, epsilon = 0.001);

        let back = DensityUnit::convert(kg_per_m3, DensityUnit::KgPerM3, DensityUnit::GPerCm3);
        assert_relative_eq!(back, g_per_cm3, epsilon = 0.001);
    }

    #[test]
    fn test_porosity_conversion() {
        let percent = 25.0;
        let fraction = PorosityUnit::convert(percent, PorosityUnit::Percent, PorosityUnit::VolumePerVolume);
        assert_relative_eq!(fraction, 0.25, epsilon = 0.001);

        let back = PorosityUnit::convert(fraction, PorosityUnit::VolumePerVolume, PorosityUnit::Percent);
        assert_relative_eq!(back, percent, epsilon = 0.001);
    }

    #[test]
    fn test_resistivity_conversion() {
        let ohm_m = 10.0;
        let ohm_cm = ResistivityUnit::convert(ohm_m, ResistivityUnit::OhmM, ResistivityUnit::OhmCm);
        assert_relative_eq!(ohm_cm, 1000.0, epsilon = 0.001);
    }

    #[test]
    fn test_sonic_conversion() {
        let us_per_ft = 100.0;
        let us_per_m = SonicUnit::convert(us_per_ft, SonicUnit::MicrosecondsPerFoot, SonicUnit::MicrosecondsPerMeter);
        // 100 µs/ft * (1 ft / 0.3048 m) = 328.08 µs/m
        assert_relative_eq!(us_per_m, 328.083989501, epsilon = 0.01);
    }

    #[test]
    fn test_caliper_conversion() {
        let inches = 8.5;
        let cm = CaliperUnit::convert(inches, CaliperUnit::Inches, CaliperUnit::Centimeters);
        assert_relative_eq!(cm, 21.59, epsilon = 0.01);

        let mm = CaliperUnit::convert(inches, CaliperUnit::Inches, CaliperUnit::Millimeters);
        assert_relative_eq!(mm, 215.9, epsilon = 0.1);
    }

    #[test]
    fn test_convert_curve_value_depth() {
        let result = convert_curve_value(100.0, "DEPTH", "ft", "m").unwrap();
        assert_relative_eq!(result, 30.48, epsilon = 0.001);
    }

    #[test]
    fn test_convert_curve_value_unknown_unit() {
        let result = convert_curve_value(100.0, "DEPTH", "unknown", "m");
        assert!(result.is_err());
    }

    #[test]
    fn test_display_traits() {
        assert_eq!(format!("{}", DepthUnit::Feet), "ft");
        assert_eq!(format!("{}", DepthUnit::Meters), "m");
        assert_eq!(format!("{}", DensityUnit::GPerCm3), "g/cm³");
        assert_eq!(format!("{}", PorosityUnit::Percent), "%");
        assert_eq!(format!("{}", SonicUnit::MicrosecondsPerFoot), "µs/ft");
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(DepthUnit::from_str("FT"), Some(DepthUnit::Feet));
        assert_eq!(DepthUnit::from_str("FEET"), Some(DepthUnit::Feet));
        assert_eq!(DepthUnit::from_str("Meters"), Some(DepthUnit::Meters));
    }
}
