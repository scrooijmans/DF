//! Unit Service (OSDU-inspired Unit Conversion)
//!
//! Provides unit lookup and conversion functionality following OSDU patterns.
//! This module enables:
//! - Lookup of units by symbol or ID
//! - Conversion between units of the same measurement type
//! - Validation that units are compatible with curve properties

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use crate::error::{Error, Result};
use crate::models::{MeasurementType, Unit};

/// Parse a datetime string from SQLite
fn parse_datetime(s: String) -> DateTime<Utc> {
	DateTime::parse_from_rfc3339(&s)
		.map(|dt| dt.with_timezone(&Utc))
		.unwrap_or_else(|_| {
			// Fallback: try parsing SQLite's default format
			chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
				.map(|ndt| DateTime::from_naive_utc_and_offset(ndt, Utc))
				.unwrap_or_else(|_| Utc::now())
		})
}

/// Unit conversion service that wraps database lookups
pub struct UnitService<'a> {
	conn: &'a Connection,
}

impl<'a> UnitService<'a> {
	/// Create a new UnitService wrapping a database connection
	pub fn new(conn: &'a Connection) -> Self {
		Self { conn }
	}

	/// Look up a unit by its ID
	pub fn get_unit(&self, id: &str) -> Result<Option<Unit>> {
		let result = self.conn.query_row(
			r#"SELECT id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base, created_at
			   FROM units WHERE id = ?1"#,
			params![id],
			|row| {
				Ok(Unit {
					id: row.get(0)?,
					measurement_type_id: row.get(1)?,
					symbol: row.get(2)?,
					name: row.get(3)?,
					to_base_factor: row.get(4)?,
					to_base_offset: row.get(5)?,
					is_base: row.get::<_, i32>(6)? == 1,
					created_at: parse_datetime(row.get::<_, String>(7)?),
				})
			},
		);

		match result {
			Ok(unit) => Ok(Some(unit)),
			Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
			Err(e) => Err(Error::Database(e)),
		}
	}

	/// Look up a unit by its symbol (case-insensitive)
	pub fn get_unit_by_symbol(&self, symbol: &str) -> Result<Option<Unit>> {
		// Normalize symbol for lookup
		let normalized = normalize_unit_symbol(symbol);

		let result = self.conn.query_row(
			r#"SELECT id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base, created_at
			   FROM units WHERE LOWER(symbol) = LOWER(?1) OR id = ?1"#,
			params![normalized],
			|row| {
				Ok(Unit {
					id: row.get(0)?,
					measurement_type_id: row.get(1)?,
					symbol: row.get(2)?,
					name: row.get(3)?,
					to_base_factor: row.get(4)?,
					to_base_offset: row.get(5)?,
					is_base: row.get::<_, i32>(6)? == 1,
					created_at: parse_datetime(row.get::<_, String>(7)?),
				})
			},
		);

		match result {
			Ok(unit) => Ok(Some(unit)),
			Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
			Err(e) => Err(Error::Database(e)),
		}
	}

	/// Look up a measurement type by its ID
	pub fn get_measurement_type(&self, id: &str) -> Result<Option<MeasurementType>> {
		let result = self.conn.query_row(
			r#"SELECT id, name, description, base_unit_id, created_at
			   FROM measurement_types WHERE id = ?1"#,
			params![id],
			|row| {
				Ok(MeasurementType {
					id: row.get(0)?,
					name: row.get(1)?,
					description: row.get(2)?,
					base_unit_id: row.get(3)?,
					created_at: parse_datetime(row.get::<_, String>(4)?),
				})
			},
		);

		match result {
			Ok(mt) => Ok(Some(mt)),
			Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
			Err(e) => Err(Error::Database(e)),
		}
	}

	/// Get all units for a measurement type
	pub fn get_units_for_type(&self, measurement_type_id: &str) -> Result<Vec<Unit>> {
		let mut stmt = self.conn.prepare(
			r#"SELECT id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base, created_at
			   FROM units WHERE measurement_type_id = ?1 ORDER BY is_base DESC, symbol"#,
		)?;

		let units = stmt
			.query_map(params![measurement_type_id], |row| {
				Ok(Unit {
					id: row.get(0)?,
					measurement_type_id: row.get(1)?,
					symbol: row.get(2)?,
					name: row.get(3)?,
					to_base_factor: row.get(4)?,
					to_base_offset: row.get(5)?,
					is_base: row.get::<_, i32>(6)? == 1,
					created_at: parse_datetime(row.get::<_, String>(7)?),
				})
			})?
			.collect::<std::result::Result<Vec<_>, _>>()?;

		Ok(units)
	}

	/// Convert a value between two units
	///
	/// Returns an error if the units are not of the same measurement type.
	pub fn convert(&self, value: f64, from_unit_id: &str, to_unit_id: &str) -> Result<f64> {
		let from_unit = self.get_unit(from_unit_id)?
			.ok_or_else(|| Error::InvalidData(format!("Unit not found: {}", from_unit_id)))?;
		let to_unit = self.get_unit(to_unit_id)?
			.ok_or_else(|| Error::InvalidData(format!("Unit not found: {}", to_unit_id)))?;

		if from_unit.measurement_type_id != to_unit.measurement_type_id {
			return Err(Error::InvalidData(format!(
				"Cannot convert between different measurement types: {} and {}",
				from_unit.measurement_type_id, to_unit.measurement_type_id
			)));
		}

		// Convert: from_unit -> base -> to_unit
		let base_value = from_unit.to_base(value);
		let result = to_unit.from_base(base_value);

		Ok(result)
	}

	/// Convert a value from a symbol to another symbol
	pub fn convert_by_symbol(&self, value: f64, from_symbol: &str, to_symbol: &str) -> Result<f64> {
		let from_unit = self.get_unit_by_symbol(from_symbol)?
			.ok_or_else(|| Error::InvalidData(format!("Unit not found: {}", from_symbol)))?;
		let to_unit = self.get_unit_by_symbol(to_symbol)?
			.ok_or_else(|| Error::InvalidData(format!("Unit not found: {}", to_symbol)))?;

		if from_unit.measurement_type_id != to_unit.measurement_type_id {
			return Err(Error::InvalidData(format!(
				"Cannot convert between different measurement types: {} and {}",
				from_unit.measurement_type_id, to_unit.measurement_type_id
			)));
		}

		let base_value = from_unit.to_base(value);
		let result = to_unit.from_base(base_value);

		Ok(result)
	}

	/// Check if two units are compatible (same measurement type)
	pub fn are_compatible(&self, unit1_id: &str, unit2_id: &str) -> Result<bool> {
		let unit1 = self.get_unit(unit1_id)?;
		let unit2 = self.get_unit(unit2_id)?;

		match (unit1, unit2) {
			(Some(u1), Some(u2)) => Ok(u1.measurement_type_id == u2.measurement_type_id),
			_ => Ok(false),
		}
	}

	/// Get the base unit for a measurement type
	pub fn get_base_unit(&self, measurement_type_id: &str) -> Result<Option<Unit>> {
		let result = self.conn.query_row(
			r#"SELECT id, measurement_type_id, symbol, name, to_base_factor, to_base_offset, is_base, created_at
			   FROM units WHERE measurement_type_id = ?1 AND is_base = 1"#,
			params![measurement_type_id],
			|row| {
				Ok(Unit {
					id: row.get(0)?,
					measurement_type_id: row.get(1)?,
					symbol: row.get(2)?,
					name: row.get(3)?,
					to_base_factor: row.get(4)?,
					to_base_offset: row.get(5)?,
					is_base: true,
					created_at: parse_datetime(row.get::<_, String>(7)?),
				})
			},
		);

		match result {
			Ok(unit) => Ok(Some(unit)),
			Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
			Err(e) => Err(Error::Database(e)),
		}
	}
}

/// Normalize a unit symbol for lookup
///
/// Handles common variations like:
/// - "feet" -> "ft"
/// - "meters" -> "m"
/// - "GAPI" -> "gapi"
pub fn normalize_unit_symbol(symbol: &str) -> String {
	let lower = symbol.to_lowercase().trim().to_string();

	match lower.as_str() {
		// Length
		"feet" | "foot" => "ft".to_string(),
		"meters" | "meter" | "metre" | "metres" => "m".to_string(),
		"inches" | "inch" => "in".to_string(),
		"centimeters" | "centimeter" | "centimetre" => "cm".to_string(),
		"millimeters" | "millimeter" | "millimetre" => "mm".to_string(),

		// Density
		"g/cc" | "gcc" | "gm/cc" => "g/cm3".to_string(),
		"kg/m3" | "kg/m^3" => "kg_m3".to_string(),

		// Porosity
		"frac" | "fraction" | "v/v" | "dec" => "v_v".to_string(),
		"%" | "percent" | "pct" => "percent".to_string(),
		"p.u." | "pu" => "pu".to_string(),

		// Resistivity
		"ohm-m" | "ohm.m" | "ohmm" | "ohm*m" => "ohm_m".to_string(),

		// Slowness
		"us/f" | "usec/ft" | "usec/f" => "us_ft".to_string(),
		"us/m" | "usec/m" => "us_m".to_string(),

		// Voltage
		"millivolt" | "millivolts" => "mv".to_string(),
		"volt" | "volts" => "v".to_string(),

		// Force
		"pounds" | "lbs" | "lb" => "lbf".to_string(),
		"newtons" | "newton" => "n".to_string(),

		// Gamma ray
		"api" | "gapi" => "gapi".to_string(),

		// Default: return as-is
		_ => lower,
	}
}

/// Detect the unit ID from a raw unit string
///
/// Tries to match the unit string to a known unit in the database.
pub fn detect_unit_id(conn: &Connection, raw_unit: &str) -> Result<Option<String>> {
	let service = UnitService::new(conn);
	let normalized = normalize_unit_symbol(raw_unit);

	if let Some(unit) = service.get_unit_by_symbol(&normalized)? {
		return Ok(Some(unit.id));
	}

	// Try exact match on ID
	if let Some(unit) = service.get_unit(&normalized)? {
		return Ok(Some(unit.id));
	}

	Ok(None)
}

/// Convert an array of values between units
pub fn convert_values(values: &[f64], from_factor: f64, from_offset: f64, to_factor: f64, to_offset: f64) -> Vec<f64> {
	values
		.iter()
		.map(|&v| {
			if v.is_nan() {
				v
			} else {
				// from -> base -> to
				let base = v * from_factor + from_offset;
				(base - to_offset) / to_factor
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::db::open_memory_db;

	#[test]
	fn test_unit_service_lookup() {
		let conn = open_memory_db().expect("Failed to create memory database");
		let service = UnitService::new(&conn);

		// Look up feet
		let ft = service.get_unit("ft").unwrap().expect("Should find ft unit");
		assert_eq!(ft.symbol, "ft");
		assert!((ft.to_base_factor - 0.3048).abs() < 0.0001);

		// Look up by symbol
		let ft2 = service.get_unit_by_symbol("feet").unwrap().expect("Should find feet");
		assert_eq!(ft2.id, "ft");
	}

	#[test]
	fn test_unit_conversion() {
		let conn = open_memory_db().expect("Failed to create memory database");
		let service = UnitService::new(&conn);

		// Convert 1 foot to meters
		let meters = service.convert(1.0, "ft", "m").unwrap();
		assert!((meters - 0.3048).abs() < 0.0001);

		// Convert 1 meter to feet
		let feet = service.convert(1.0, "m", "ft").unwrap();
		assert!((feet - 3.28084).abs() < 0.001);

		// Round trip: 100 ft -> m -> ft
		let ft_value = 100.0;
		let m_value = service.convert(ft_value, "ft", "m").unwrap();
		let ft_back = service.convert(m_value, "m", "ft").unwrap();
		assert!((ft_back - ft_value).abs() < 0.0001);
	}

	#[test]
	fn test_incompatible_units() {
		let conn = open_memory_db().expect("Failed to create memory database");
		let service = UnitService::new(&conn);

		// Try to convert between incompatible units
		let result = service.convert(1.0, "ft", "psi");
		assert!(result.is_err());
	}

	#[test]
	fn test_normalize_unit_symbol() {
		assert_eq!(normalize_unit_symbol("feet"), "ft");
		assert_eq!(normalize_unit_symbol("METERS"), "m");
		assert_eq!(normalize_unit_symbol("g/cc"), "g/cm3");
		assert_eq!(normalize_unit_symbol("ohm-m"), "ohm_m");
		assert_eq!(normalize_unit_symbol("GAPI"), "gapi");
	}

	#[test]
	fn test_get_units_for_type() {
		let conn = open_memory_db().expect("Failed to create memory database");
		let service = UnitService::new(&conn);

		let length_units = service.get_units_for_type("length").unwrap();
		assert!(length_units.len() >= 4);

		// Base unit should be first
		assert!(length_units[0].is_base);
		assert_eq!(length_units[0].symbol, "m");
	}

	#[test]
	fn test_measurement_type_lookup() {
		let conn = open_memory_db().expect("Failed to create memory database");
		let service = UnitService::new(&conn);

		let length = service.get_measurement_type("length").unwrap().expect("Should find length");
		assert_eq!(length.name, "Length");
		assert_eq!(length.base_unit_id, Some("m".to_string()));
	}

	#[test]
	fn test_detect_unit_id() {
		let conn = open_memory_db().expect("Failed to create memory database");

		assert_eq!(detect_unit_id(&conn, "feet").unwrap(), Some("ft".to_string()));
		assert_eq!(detect_unit_id(&conn, "g/cc").unwrap(), Some("g_cm3".to_string()));
		assert_eq!(detect_unit_id(&conn, "GAPI").unwrap(), Some("gapi".to_string()));
	}

	#[test]
	fn test_convert_values() {
		// Convert feet to meters: factor = 0.3048, offset = 0
		let values = vec![0.0, 1.0, 2.0, f64::NAN, 10.0];
		let result = convert_values(&values, 0.3048, 0.0, 1.0, 0.0);

		assert!((result[0] - 0.0).abs() < 0.0001);
		assert!((result[1] - 0.3048).abs() < 0.0001);
		assert!((result[2] - 0.6096).abs() < 0.0001);
		assert!(result[3].is_nan());
		assert!((result[4] - 3.048).abs() < 0.0001);
	}
}
