//! LAS file processing
//!
//! Wraps the `las-parser` crate for LAS file parsing and provides
//! integration with the DataForge data model.

use las_types::LASFile;
use std::path::Path;
use tracing::info;

use crate::error::{Error, Result};
use crate::models::MainCurveType;

/// Parsed LAS file with extracted metadata
#[derive(Debug)]
pub struct ParsedLasFile {
	/// Original filename
	pub filename: String,

	/// Well metadata
	pub well_name: Option<String>,
	pub uwi: Option<String>,
	pub field: Option<String>,
	pub company: Option<String>,
	pub location: Option<String>,

	/// Depth range from header (STRT/STOP/STEP)
	pub start_depth: f64,
	pub stop_depth: f64,
	pub step: f64,
	pub null_value: f64,

	/// Actual depth values extracted from the DEPT/DEPTH curve
	/// This is the authoritative source for depth indexing - not reconstructed from STEP
	pub actual_depths: Vec<f64>,

	/// Mnemonic of the depth curve (e.g., "DEPT", "DEPTH", "MD")
	pub depth_mnemonic: String,

	/// Unit of the depth curve (e.g., "M", "FT")
	pub depth_unit: String,

	/// Whether the depth spacing is irregular (gaps detected)
	/// True if STEP=0 or actual step varies beyond tolerance
	pub has_irregular_spacing: bool,

	/// Curve data (excluding the depth curve)
	pub curves: Vec<ParsedCurve>,

	/// LAS version
	pub version: String,
}

/// A parsed curve from a LAS file
#[derive(Debug)]
pub struct ParsedCurve {
	/// Curve mnemonic (e.g., "GR", "RHOB")
	pub mnemonic: String,

	/// Unit from LAS file (e.g., "GAPI", "G/CC")
	pub unit: String,

	/// Description from LAS file
	pub description: String,

	/// Detected curve type
	pub detected_type: MainCurveType,

	/// Curve data values
	pub values: Vec<f64>,

	/// Statistics
	pub row_count: usize,
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
}

/// Parse a LAS file from a file path
pub fn parse_las_file(path: &Path) -> Result<ParsedLasFile> {
	let filename = path
		.file_name()
		.map(|s| s.to_string_lossy().to_string())
		.unwrap_or_else(|| "unknown.las".to_string());

	// Parse using las-types
	let las_file = LASFile::read(path).map_err(|e| Error::LasParsing(e.to_string()))?;

	convert_las_file(las_file, &filename)
}

/// Parse a LAS file from bytes (writes to temp file first)
pub fn parse_las_bytes(data: &[u8], filename: &str) -> Result<ParsedLasFile> {
	// Write to temp file and parse
	let temp_dir = std::env::temp_dir();
	let temp_path = temp_dir.join(format!("dataforge_las_{}", uuid::Uuid::new_v4()));
	std::fs::write(&temp_path, data)?;

	let result = parse_las_file(&temp_path);

	// Clean up temp file
	let _ = std::fs::remove_file(&temp_path);

	result.map(|mut parsed| {
		parsed.filename = filename.to_string();
		parsed
	})
}

/// Convert a LASFile to ParsedLasFile
fn convert_las_file(las_file: LASFile, filename: &str) -> Result<ParsedLasFile> {
	info!(
		filename = filename,
		curves = las_file.curves.columns.len(),
		"Parsed LAS file"
	);

	// Extract well metadata
	let well_name = las_file.well.get_well_name().map(|s| s.to_string());
	let uwi = las_file.well.get_uwi().map(|s| s.to_string());
	let field = las_file.well.get_field().map(|s| s.to_string());
	let company = las_file.well.get_company().map(|s| s.to_string());
	let location = las_file.well.get_location().map(|s| s.to_string());
	let start_depth = las_file.well.get_start_depth();
	let stop_depth = las_file.well.get_stop_depth();
	let step = las_file.well.get_step();
	let null_value = las_file.well.get_null_value();

	// Find the depth curve - it's typically the first column and is named DEPT, DEPTH, MD, etc.
	let mut depth_curve_index: Option<usize> = None;
	let mut depth_mnemonic = String::from("DEPT");
	let mut depth_unit = String::new();

	for (i, column) in las_file.curves.columns.iter().enumerate() {
		let detected_type = detect_curve_type(&column.mnemonic);
		if detected_type == MainCurveType::DEPTH {
			depth_curve_index = Some(i);
			depth_mnemonic = column.mnemonic.clone();
			depth_unit = column.unit.clone();
			break;
		}
	}

	// Extract actual depth values from the DEPT curve
	// LAS standard: The index curve (usually first column) contains the actual depth values
	let actual_depths: Vec<f64> = if let Some(idx) = depth_curve_index {
		las_file.curves.columns[idx]
			.values
			.iter()
			.map(|&v| {
				// Depth values should never be null, but handle it gracefully
				if (v - null_value).abs() < 1e-10 {
					f64::NAN
				} else {
					v
				}
			})
			.collect()
	} else {
		// Fallback: If no DEPT curve found, use first column as index (LAS convention)
		// This handles non-standard LAS files
		if !las_file.curves.columns.is_empty() {
			depth_mnemonic = las_file.curves.columns[0].mnemonic.clone();
			depth_unit = las_file.curves.columns[0].unit.clone();
			depth_curve_index = Some(0);
			las_file.curves.columns[0]
				.values
				.iter()
				.map(|&v| {
					if (v - null_value).abs() < 1e-10 {
						f64::NAN
					} else {
						v
					}
				})
				.collect()
		} else {
			Vec::new()
		}
	};

	// Detect irregular spacing
	// Per LAS standard: STEP=0 means non-constant spacing
	// Also check if actual depth steps vary beyond tolerance
	let has_irregular_spacing = detect_irregular_spacing(&actual_depths, step);

	if has_irregular_spacing {
		info!(
			filename = filename,
			depth_mnemonic = depth_mnemonic,
			step = step,
			"Detected irregular depth spacing - gaps may be present"
		);
	}

	// Parse curves (excluding the depth curve which is now stored separately)
	let mut curves = Vec::new();
	for (i, column) in las_file.curves.columns.iter().enumerate() {
		// Skip the depth curve - it's stored in actual_depths
		if Some(i) == depth_curve_index {
			continue;
		}

		let mnemonic = column.mnemonic.clone();
		let unit = column.unit.clone();
		let description = column.description.clone();

		// Detect curve type from mnemonic
		let detected_type = detect_curve_type(&mnemonic);

		// Get values, replacing null values with NaN
		let values: Vec<f64> = column
			.values
			.iter()
			.map(|&v| {
				if (v - null_value).abs() < 1e-10 {
					f64::NAN
				} else {
					v
				}
			})
			.collect();

		// Compute statistics (excluding NaN)
		let valid_values: Vec<f64> = values.iter().copied().filter(|v| !v.is_nan()).collect();

		let min_value = valid_values.iter().copied().reduce(f64::min);
		let max_value = valid_values.iter().copied().reduce(f64::max);

		curves.push(ParsedCurve {
			mnemonic,
			unit,
			description,
			detected_type,
			row_count: values.len(),
			min_value,
			max_value,
			values,
		});
	}

	// Get version string from VersionSection
	let version = las_file
		.version
		.get_version()
		.map(|s| s.to_string())
		.unwrap_or_else(|| "2.0".to_string());

	Ok(ParsedLasFile {
		filename: filename.to_string(),
		well_name,
		uwi,
		field,
		company,
		location,
		start_depth,
		stop_depth,
		step,
		null_value,
		actual_depths,
		depth_mnemonic,
		depth_unit,
		has_irregular_spacing,
		curves,
		version,
	})
}

/// Detect if depth values have irregular spacing (gaps)
/// Returns true if STEP=0 or actual steps vary beyond tolerance
fn detect_irregular_spacing(depths: &[f64], header_step: f64) -> bool {
	// Per LAS standard: STEP=0 explicitly means non-constant spacing
	if header_step.abs() < 1e-10 {
		return true;
	}

	// Check if we have enough points to analyze
	if depths.len() < 2 {
		return false;
	}

	// Calculate actual step sizes and check for irregularity
	// Allow 1% tolerance for floating point comparison
	let tolerance = header_step.abs() * 0.01;

	for window in depths.windows(2) {
		let actual_step = (window[1] - window[0]).abs();

		// Skip if either depth is NaN
		if window[0].is_nan() || window[1].is_nan() {
			continue;
		}

		// Check if actual step differs significantly from header step
		if (actual_step - header_step.abs()).abs() > tolerance {
			return true;
		}
	}

	false
}

/// Detect curve type from mnemonic
pub fn detect_curve_type(mnemonic: &str) -> MainCurveType {
	let upper = mnemonic.to_uppercase();

	// Depth indicators
	if upper == "DEPT"
		|| upper == "DEPTH"
		|| upper == "MD"
		|| upper == "TVD"
		|| upper == "TVDSS"
	{
		return MainCurveType::DEPTH;
	}

	// Gamma Ray
	if upper.starts_with("GR")
		|| upper == "SGR"
		|| upper == "CGR"
		|| upper.contains("GAMMA")
	{
		return MainCurveType::GR;
	}

	// Density
	if upper.starts_with("RHOB")
		|| upper == "RHOZ"
		|| upper == "DEN"
		|| upper == "ZDEN"
		|| upper.contains("DENSITY")
	{
		return MainCurveType::RHOB;
	}

	// Neutron Porosity
	if upper.starts_with("NPHI")
		|| upper == "TNPH"
		|| upper == "NPOR"
		|| upper == "NEU"
		|| upper.contains("NEUTRON")
	{
		return MainCurveType::NPHI;
	}

	// Resistivity
	if upper.starts_with("RT")
		|| upper.starts_with("RES")
		|| upper == "LLD"
		|| upper == "LLS"
		|| upper == "ILD"
		|| upper == "ILS"
		|| upper == "HDRS"
		|| upper == "HMRS"
		|| upper.contains("RESISTIVITY")
	{
		return MainCurveType::RT;
	}

	// Caliper
	if upper.starts_with("CAL") || upper == "HCAL" || upper.contains("CALIPER") {
		return MainCurveType::CALI;
	}

	// Sonic
	if upper.starts_with("DT")
		|| upper == "AC"
		|| upper == "DTCO"
		|| upper.contains("SONIC")
	{
		return MainCurveType::DT;
	}

	// Spontaneous Potential
	if upper == "SP" || upper == "SSP" || upper.contains("SPONTANEOUS") {
		return MainCurveType::SP;
	}

	// Photoelectric
	if upper.starts_with("PE") || upper == "PEF" || upper.contains("PHOTOELECTRIC") {
		return MainCurveType::PE;
	}

	MainCurveType::OTHER
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_detect_curve_type() {
		assert_eq!(detect_curve_type("DEPT"), MainCurveType::DEPTH);
		assert_eq!(detect_curve_type("GR"), MainCurveType::GR);
		assert_eq!(detect_curve_type("RHOB"), MainCurveType::RHOB);
		assert_eq!(detect_curve_type("NPHI"), MainCurveType::NPHI);
		assert_eq!(detect_curve_type("RT"), MainCurveType::RT);
		assert_eq!(detect_curve_type("LLD"), MainCurveType::RT);
		assert_eq!(detect_curve_type("CALI"), MainCurveType::CALI);
		assert_eq!(detect_curve_type("DT"), MainCurveType::DT);
		assert_eq!(detect_curve_type("SP"), MainCurveType::SP);
		assert_eq!(detect_curve_type("PE"), MainCurveType::PE);
		assert_eq!(detect_curve_type("UNKNOWN"), MainCurveType::OTHER);
	}
}
