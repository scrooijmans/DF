//! Well depth grid and resampling utilities
//!
//! This module provides the core depth grid abstraction and resampling algorithms
//! for normalizing well log data to a consistent sampling interval.
//!
//! # Overview
//!
//! Each well has a canonical depth grid that defines:
//! - Unit (feet or meters)
//! - Step size (e.g., 0.5 ft)
//! - Origin (typically 0.0)
//!
//! When curves are imported, they are resampled to align with this grid,
//! ensuring all curves for a well share the same depth index.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during resampling
#[derive(Error, Debug)]
pub enum ResampleError {
	#[error("Empty input data")]
	EmptyInput,

	#[error("Mismatched array lengths: depths={0}, values={1}")]
	LengthMismatch(usize, usize),

	#[error("Invalid depth range: start={0}, end={1}")]
	InvalidRange(f64, f64),

	#[error("Invalid step size: {0}")]
	InvalidStep(f64),

	#[error("Unit conversion not supported: {from} to {to}")]
	UnsupportedConversion { from: String, to: String },
}

/// Result type for resampling operations
pub type Result<T> = std::result::Result<T, ResampleError>;

/// Depth unit (feet or meters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DepthUnit {
	#[default]
	Feet,
	Meters,
}

impl DepthUnit {
	/// Conversion factor to convert FROM this unit TO feet
	pub fn to_feet_factor(&self) -> f64 {
		match self {
			DepthUnit::Feet => 1.0,
			DepthUnit::Meters => 3.28084,
		}
	}

	/// Conversion factor to convert FROM this unit TO meters
	pub fn to_meters_factor(&self) -> f64 {
		match self {
			DepthUnit::Feet => 0.3048,
			DepthUnit::Meters => 1.0,
		}
	}

	/// Convert a depth value to another unit
	pub fn convert(&self, value: f64, to: DepthUnit) -> f64 {
		if *self == to {
			return value;
		}
		match (*self, to) {
			(DepthUnit::Feet, DepthUnit::Meters) => value * 0.3048,
			(DepthUnit::Meters, DepthUnit::Feet) => value / 0.3048,
			_ => value,
		}
	}

	/// Parse from string (case-insensitive)
	pub fn from_str_loose(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"ft" | "feet" | "f" => Some(DepthUnit::Feet),
			"m" | "meters" | "metre" | "metres" => Some(DepthUnit::Meters),
			_ => None,
		}
	}
}

impl std::fmt::Display for DepthUnit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DepthUnit::Feet => write!(f, "ft"),
			DepthUnit::Meters => write!(f, "m"),
		}
	}
}

impl std::str::FromStr for DepthUnit {
	type Err = String;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		DepthUnit::from_str_loose(s).ok_or_else(|| format!("Unknown depth unit: {}", s))
	}
}

/// Resampling method for interpolating curve values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResampleMethod {
	/// Linear interpolation between adjacent points (default)
	#[default]
	Linear,

	/// Use nearest neighbor value
	Nearest,

	/// Use previous value (step function)
	Previous,

	/// Use next value
	Next,

	/// No interpolation - only keep exact matches (within tolerance)
	None,
}

impl std::fmt::Display for ResampleMethod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ResampleMethod::Linear => write!(f, "linear"),
			ResampleMethod::Nearest => write!(f, "nearest"),
			ResampleMethod::Previous => write!(f, "previous"),
			ResampleMethod::Next => write!(f, "next"),
			ResampleMethod::None => write!(f, "none"),
		}
	}
}

/// Well depth grid configuration
///
/// Defines the canonical depth sampling for a well.
/// All curves are resampled to align with this grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellDepthGrid {
	/// Depth unit (feet or meters)
	pub unit: DepthUnit,

	/// Sampling step size (e.g., 0.5 for half-foot sampling)
	pub step: f64,

	/// Grid origin (typically 0.0)
	pub origin: f64,
}

impl Default for WellDepthGrid {
	fn default() -> Self {
		Self {
			unit: DepthUnit::Feet,
			step: 0.5,
			origin: 0.0,
		}
	}
}

impl WellDepthGrid {
	/// Create a new depth grid
	pub fn new(unit: DepthUnit, step: f64, origin: f64) -> Self {
		Self { unit, step, origin }
	}

	/// Create a feet-based grid with the given step
	pub fn feet(step: f64) -> Self {
		Self::new(DepthUnit::Feet, step, 0.0)
	}

	/// Create a meters-based grid with the given step
	pub fn meters(step: f64) -> Self {
		Self::new(DepthUnit::Meters, step, 0.0)
	}

	/// Compute grid index for a depth value
	///
	/// Returns the index i such that depth ≈ origin + i * step
	pub fn depth_to_index(&self, depth: f64) -> i64 {
		((depth - self.origin) / self.step).round() as i64
	}

	/// Compute depth from grid index
	pub fn index_to_depth(&self, index: i64) -> f64 {
		self.origin + (index as f64) * self.step
	}

	/// Snap a depth to the nearest grid point
	pub fn snap(&self, depth: f64) -> f64 {
		self.index_to_depth(self.depth_to_index(depth))
	}

	/// Generate grid indices for a depth range
	///
	/// Returns all indices from start to end (inclusive)
	pub fn indices_for_range(&self, start_depth: f64, end_depth: f64) -> Vec<i64> {
		let start_idx = self.depth_to_index(start_depth);
		let end_idx = self.depth_to_index(end_depth);

		let (min_idx, max_idx) = if start_idx <= end_idx {
			(start_idx, end_idx)
		} else {
			(end_idx, start_idx)
		};

		(min_idx..=max_idx).collect()
	}

	/// Generate depths for a range (snapped to grid)
	pub fn depths_for_range(&self, start_depth: f64, end_depth: f64) -> Vec<f64> {
		self.indices_for_range(start_depth, end_depth)
			.iter()
			.map(|&idx| self.index_to_depth(idx))
			.collect()
	}

	/// Convert depths from another unit to this grid's unit
	pub fn convert_depths(&self, depths: &[f64], from_unit: DepthUnit) -> Vec<f64> {
		if from_unit == self.unit {
			return depths.to_vec();
		}
		depths.iter().map(|&d| from_unit.convert(d, self.unit)).collect()
	}
}

/// Resampling policy that tracks how a curve was resampled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResamplePolicy {
	/// Method used for interpolation
	pub method: ResampleMethod,

	/// Original depth unit (before conversion)
	pub source_unit: DepthUnit,

	/// Target grid
	pub target_unit: DepthUnit,
	pub target_step: f64,

	/// Whether unit conversion was applied
	pub was_unit_converted: bool,

	/// Snapped start depth (in target unit)
	pub snapped_start: f64,

	/// Snapped end depth (in target unit)
	pub snapped_end: f64,
}

/// Result of resampling a curve
#[derive(Debug, Clone)]
pub struct ResampledCurve {
	/// Grid indices (can be used to reconstruct depths)
	pub indices: Vec<i64>,

	/// Resampled values (None for missing/null)
	pub values: Vec<Option<f64>>,

	/// Resampling policy used
	pub policy: ResamplePolicy,

	/// Statistics
	pub sample_count: usize,
	pub null_count: usize,
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
	pub mean_value: Option<f64>,
}

/// Resample curve data to a target depth grid
///
/// # Arguments
/// * `source_depths` - Original depth values
/// * `source_values` - Original curve values (NaN for null)
/// * `source_unit` - Unit of source depths
/// * `target_grid` - Target depth grid to resample to
/// * `method` - Interpolation method
///
/// # Returns
/// Resampled curve data with grid indices and interpolated values
pub fn resample_curve(
	source_depths: &[f64],
	source_values: &[f64],
	source_unit: DepthUnit,
	target_grid: &WellDepthGrid,
	method: ResampleMethod,
) -> Result<ResampledCurve> {
	if source_depths.is_empty() || source_values.is_empty() {
		return Err(ResampleError::EmptyInput);
	}

	if source_depths.len() != source_values.len() {
		return Err(ResampleError::LengthMismatch(
			source_depths.len(),
			source_values.len(),
		));
	}

	// Convert depths to target unit if needed
	let converted_depths = target_grid.convert_depths(source_depths, source_unit);
	let was_unit_converted = source_unit != target_grid.unit;

	// Find the range in target grid
	let min_depth = converted_depths.iter().copied().fold(f64::INFINITY, f64::min);
	let max_depth = converted_depths.iter().copied().fold(f64::NEG_INFINITY, f64::max);

	let snapped_start = target_grid.snap(min_depth);
	let snapped_end = target_grid.snap(max_depth);

	// Generate target indices
	let indices = target_grid.indices_for_range(snapped_start, snapped_end);

	// Interpolate values at each grid point
	let mut values: Vec<Option<f64>> = Vec::with_capacity(indices.len());
	let mut null_count = 0;
	let mut min_val: Option<f64> = None;
	let mut max_val: Option<f64> = None;
	let mut sum = 0.0;
	let mut valid_count = 0usize;

	for &idx in &indices {
		let target_depth = target_grid.index_to_depth(idx);
		let interpolated = interpolate_at(
			&converted_depths,
			source_values,
			target_depth,
			method,
			target_grid.step,
		);

		if let Some(val) = interpolated {
			if !val.is_nan() {
				min_val = Some(min_val.map_or(val, |m| f64::min(m, val)));
				max_val = Some(max_val.map_or(val, |m| f64::max(m, val)));
				sum += val;
				valid_count += 1;
				values.push(Some(val));
			} else {
				null_count += 1;
				values.push(None);
			}
		} else {
			null_count += 1;
			values.push(None);
		}
	}

	let mean_value = if valid_count > 0 {
		Some(sum / valid_count as f64)
	} else {
		None
	};

	let policy = ResamplePolicy {
		method,
		source_unit,
		target_unit: target_grid.unit,
		target_step: target_grid.step,
		was_unit_converted,
		snapped_start,
		snapped_end,
	};

	Ok(ResampledCurve {
		indices,
		values,
		policy,
		sample_count: valid_count,
		null_count,
		min_value: min_val,
		max_value: max_val,
		mean_value,
	})
}

/// Interpolate a value at a specific depth
fn interpolate_at(
	depths: &[f64],
	values: &[f64],
	target_depth: f64,
	method: ResampleMethod,
	tolerance: f64,
) -> Option<f64> {
	if depths.is_empty() {
		return None;
	}

	// Find surrounding points using binary search
	let pos = depths.partition_point(|&d| d < target_depth);

	match method {
		ResampleMethod::Linear => linear_interpolate(depths, values, target_depth, pos),
		ResampleMethod::Nearest => nearest_interpolate(depths, values, target_depth, pos),
		ResampleMethod::Previous => {
			if pos > 0 {
				let val = values[pos - 1];
				if val.is_nan() { None } else { Some(val) }
			} else {
				None
			}
		}
		ResampleMethod::Next => {
			if pos < values.len() {
				let val = values[pos];
				if val.is_nan() { None } else { Some(val) }
			} else {
				None
			}
		}
		ResampleMethod::None => {
			// Only return value if we have an exact match within tolerance
			if pos > 0 && (depths[pos - 1] - target_depth).abs() < tolerance / 2.0 {
				let val = values[pos - 1];
				if val.is_nan() { None } else { Some(val) }
			} else if pos < depths.len() && (depths[pos] - target_depth).abs() < tolerance / 2.0 {
				let val = values[pos];
				if val.is_nan() { None } else { Some(val) }
			} else {
				None
			}
		}
	}
}

/// Linear interpolation between two points
fn linear_interpolate(depths: &[f64], values: &[f64], target_depth: f64, pos: usize) -> Option<f64> {
	// Check for exact match at pos-1 (partition_point returns first >= target)
	if pos > 0 {
		let d_prev = depths[pos - 1];
		if (d_prev - target_depth).abs() < 1e-9 {
			let val = values[pos - 1];
			return if val.is_nan() { None } else { Some(val) };
		}
	}

	// Check for exact match at pos
	if pos < depths.len() {
		let d_curr = depths[pos];
		if (d_curr - target_depth).abs() < 1e-9 {
			let val = values[pos];
			return if val.is_nan() { None } else { Some(val) };
		}
	}

	if pos == 0 {
		// Before first point - no interpolation possible
		return None;
	}
	if pos >= depths.len() {
		// After last point - no interpolation possible
		return None;
	}

	let d0 = depths[pos - 1];
	let d1 = depths[pos];
	let v0 = values[pos - 1];
	let v1 = values[pos];

	// Skip if either value is NaN
	if v0.is_nan() || v1.is_nan() {
		return None;
	}

	// Linear interpolation: v = v0 + (v1 - v0) * (t - d0) / (d1 - d0)
	let t = (target_depth - d0) / (d1 - d0);
	Some(v0 + (v1 - v0) * t)
}

/// Nearest neighbor interpolation
fn nearest_interpolate(
	depths: &[f64],
	values: &[f64],
	target_depth: f64,
	pos: usize,
) -> Option<f64> {
	if depths.is_empty() {
		return None;
	}

	let nearest_idx = if pos == 0 {
		0
	} else if pos >= depths.len() {
		depths.len() - 1
	} else {
		// Choose closer of the two surrounding points
		let d_prev = (depths[pos - 1] - target_depth).abs();
		let d_next = (depths[pos] - target_depth).abs();
		if d_prev <= d_next { pos - 1 } else { pos }
	};

	let val = values[nearest_idx];
	if val.is_nan() { None } else { Some(val) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_depth_unit_conversion() {
		let feet_val = 100.0;
		let meters_val = DepthUnit::Feet.convert(feet_val, DepthUnit::Meters);
		assert!((meters_val - 30.48).abs() < 0.001);

		let back_to_feet = DepthUnit::Meters.convert(meters_val, DepthUnit::Feet);
		assert!((back_to_feet - feet_val).abs() < 0.001);
	}

	#[test]
	fn test_well_depth_grid_snap() {
		let grid = WellDepthGrid::feet(0.5);

		assert!((grid.snap(100.0) - 100.0).abs() < 0.001);
		assert!((grid.snap(100.2) - 100.0).abs() < 0.001);
		assert!((grid.snap(100.3) - 100.5).abs() < 0.001);
		assert!((grid.snap(100.5) - 100.5).abs() < 0.001);
	}

	#[test]
	fn test_grid_index_round_trip() {
		let grid = WellDepthGrid::feet(0.5);

		for depth in [0.0, 50.5, 100.0, 1000.5, 2500.0] {
			let idx = grid.depth_to_index(depth);
			let reconstructed = grid.index_to_depth(idx);
			assert!(
				(reconstructed - depth).abs() < 0.001,
				"depth {} -> index {} -> {}",
				depth,
				idx,
				reconstructed
			);
		}
	}

	#[test]
	fn test_indices_for_range() {
		let grid = WellDepthGrid::feet(0.5);
		let indices = grid.indices_for_range(100.0, 102.0);

		assert_eq!(indices.len(), 5); // 100.0, 100.5, 101.0, 101.5, 102.0
		assert_eq!(indices[0], 200); // 100.0 / 0.5
		assert_eq!(indices[4], 204); // 102.0 / 0.5
	}

	#[test]
	fn test_linear_resample() {
		let grid = WellDepthGrid::feet(1.0);

		let source_depths = vec![0.0, 1.0, 2.0, 3.0, 4.0];
		let source_values = vec![10.0, 20.0, 30.0, 40.0, 50.0];

		let result = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Feet,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		assert_eq!(result.values.len(), 5);
		assert_eq!(result.values[0], Some(10.0));
		assert_eq!(result.values[2], Some(30.0));
		assert_eq!(result.values[4], Some(50.0));
	}

	#[test]
	fn test_resample_with_interpolation() {
		let grid = WellDepthGrid::feet(0.5);

		// Source has 1.0 ft spacing, target has 0.5 ft spacing
		let source_depths = vec![0.0, 1.0, 2.0];
		let source_values = vec![0.0, 10.0, 20.0];

		let result = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Feet,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		// Should have interpolated values at 0.5 and 1.5
		assert_eq!(result.values.len(), 5); // 0.0, 0.5, 1.0, 1.5, 2.0

		// Check interpolated values
		assert_eq!(result.values[0], Some(0.0)); // at 0.0
		assert_eq!(result.values[1], Some(5.0)); // at 0.5 (interpolated)
		assert_eq!(result.values[2], Some(10.0)); // at 1.0
		assert_eq!(result.values[3], Some(15.0)); // at 1.5 (interpolated)
		assert_eq!(result.values[4], Some(20.0)); // at 2.0
	}

	#[test]
	fn test_resample_with_unit_conversion() {
		let grid = WellDepthGrid::feet(0.5);

		// Source is in meters (roughly 0.3048m = 1ft)
		let source_depths = vec![0.0, 0.3048, 0.6096]; // 0, 1, 2 feet in meters
		let source_values = vec![0.0, 10.0, 20.0];

		let result = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Meters,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		assert!(result.policy.was_unit_converted);
		assert_eq!(result.policy.source_unit, DepthUnit::Meters);
		assert_eq!(result.policy.target_unit, DepthUnit::Feet);
	}

	#[test]
	fn test_resample_with_nan_values() {
		let grid = WellDepthGrid::feet(1.0);

		let source_depths = vec![0.0, 1.0, 2.0, 3.0];
		let source_values = vec![10.0, f64::NAN, 30.0, 40.0];

		let result = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Feet,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		assert_eq!(result.values[0], Some(10.0));
		// Interpolation through NaN should produce None
		assert_eq!(result.values[1], None);
		assert_eq!(result.values[2], Some(30.0));
		assert_eq!(result.values[3], Some(40.0));
	}

	#[test]
	fn test_resample_statistics() {
		let grid = WellDepthGrid::feet(1.0);

		let source_depths = vec![0.0, 1.0, 2.0, 3.0, 4.0];
		let source_values = vec![10.0, 20.0, 30.0, 40.0, 50.0];

		let result = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Feet,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		assert_eq!(result.min_value, Some(10.0));
		assert_eq!(result.max_value, Some(50.0));
		assert_eq!(result.mean_value, Some(30.0));
		assert_eq!(result.sample_count, 5);
		assert_eq!(result.null_count, 0);
	}
}
