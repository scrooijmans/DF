//! Surface Column Type Detection
//!
//! Automatically detects surface column types from CSV header names.
//! Supports various naming conventions used in the industry for X, Y, Z coordinates.

use crate::models::SurfaceColumnType;

/// A detected column with type and confidence
#[derive(Debug, Clone)]
pub struct DetectedSurfaceColumn {
    /// Column index in the CSV
    pub index: usize,
    /// Original column name from CSV
    pub original_name: String,
    /// Detected column type
    pub column_type: SurfaceColumnType,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Detected unit (if any)
    pub unit: Option<String>,
}

/// Detect column type from a header name
///
/// Returns the detected type and a confidence score (0.0 - 1.0).
/// Higher confidence means more certain match.
pub fn detect_surface_column_type(header: &str) -> (SurfaceColumnType, f64) {
    let normalized = normalize_header(header);

    // Try exact matches first (highest confidence)
    if let Some((col_type, conf)) = try_exact_match(&normalized) {
        return (col_type, conf);
    }

    // Try pattern matches
    if let Some((col_type, conf)) = try_pattern_match(&normalized) {
        return (col_type, conf);
    }

    // Unknown
    (SurfaceColumnType::Unknown, 0.0)
}

/// Detect column type and extract unit from header
///
/// Handles headers like "X (m)", "Depth [ft]", etc.
pub fn detect_surface_column_with_unit(index: usize, header: &str) -> DetectedSurfaceColumn {
    let (name, unit) = extract_unit_from_header(header);
    let (column_type, confidence) = detect_surface_column_type(&name);

    DetectedSurfaceColumn {
        index,
        original_name: header.to_string(),
        column_type,
        confidence,
        unit,
    }
}

/// Normalize a header for matching
fn normalize_header(header: &str) -> String {
    header
        .to_lowercase()
        .replace(['-', '_', ' ', '.'], "")
        .trim()
        .to_string()
}

/// Extract unit from header like "X (m)" or "Depth [ft]"
fn extract_unit_from_header(header: &str) -> (String, Option<String>) {
    // Try parentheses: "X (m)"
    if let Some(idx) = header.find('(') {
        if let Some(end_idx) = header.find(')') {
            let name = header[..idx].trim().to_string();
            let unit = header[idx + 1..end_idx].trim().to_string();
            return (name, Some(unit));
        }
    }

    // Try square brackets: "Depth [ft]"
    if let Some(idx) = header.find('[') {
        if let Some(end_idx) = header.find(']') {
            let name = header[..idx].trim().to_string();
            let unit = header[idx + 1..end_idx].trim().to_string();
            return (name, Some(unit));
        }
    }

    (header.to_string(), None)
}

/// Try exact string matches
fn try_exact_match(normalized: &str) -> Option<(SurfaceColumnType, f64)> {
    match normalized {
        // X coordinate matches (highest confidence)
        "x" | "xcoord" | "xcoordinate" | "easting" | "east" | "e" => {
            Some((SurfaceColumnType::X, 0.95))
        }
        "longitude" | "lon" | "long" => Some((SurfaceColumnType::X, 0.90)),

        // Y coordinate matches
        "y" | "ycoord" | "ycoordinate" | "northing" | "north" | "n" => {
            Some((SurfaceColumnType::Y, 0.95))
        }
        "latitude" | "lat" => Some((SurfaceColumnType::Y, 0.90)),

        // Z coordinate matches
        "z" | "zcoord" | "zcoordinate" | "depth" | "elevation" | "elev" | "height" => {
            Some((SurfaceColumnType::Z, 0.95))
        }
        "twt" | "twowaytime" | "time" | "value" | "val" => Some((SurfaceColumnType::Z, 0.90)),

        // Surface name matches
        "surface" | "surfacename" | "horizon" | "horizonname" | "name" | "layer" | "layername" => {
            Some((SurfaceColumnType::SurfaceName, 0.95))
        }

        // Quality matches
        "quality" | "confidence" | "uncertainty" | "error" | "qc" => {
            Some((SurfaceColumnType::Quality, 0.90))
        }

        _ => None,
    }
}

/// Try pattern-based matches
fn try_pattern_match(normalized: &str) -> Option<(SurfaceColumnType, f64)> {
    // X patterns
    if normalized.starts_with("x") && (normalized.contains("coord") || normalized.len() <= 3) {
        return Some((SurfaceColumnType::X, 0.85));
    }
    if normalized.contains("easting") || normalized.ends_with("east") {
        return Some((SurfaceColumnType::X, 0.80));
    }

    // Y patterns
    if normalized.starts_with("y") && (normalized.contains("coord") || normalized.len() <= 3) {
        return Some((SurfaceColumnType::Y, 0.85));
    }
    if normalized.contains("northing") || normalized.ends_with("north") {
        return Some((SurfaceColumnType::Y, 0.80));
    }

    // Z patterns
    if normalized.starts_with("z") && (normalized.contains("coord") || normalized.len() <= 3) {
        return Some((SurfaceColumnType::Z, 0.85));
    }
    if normalized.contains("depth") || normalized.contains("elev") || normalized.contains("height")
    {
        return Some((SurfaceColumnType::Z, 0.80));
    }

    // Surface name patterns
    if normalized.contains("surface") || normalized.contains("horizon") || normalized.contains("layer")
    {
        // But not if it looks like a type column
        if !normalized.contains("type") {
            return Some((SurfaceColumnType::SurfaceName, 0.80));
        }
    }

    // Quality patterns
    if normalized.contains("quality") || normalized.contains("confidence") {
        return Some((SurfaceColumnType::Quality, 0.75));
    }

    None
}

/// Check if required columns are present for surface ingestion
///
/// Returns Ok if X, Y, Z columns are detected with sufficient confidence.
pub fn validate_required_columns(columns: &[DetectedSurfaceColumn]) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let min_confidence = 0.5;

    let has_x = columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::X && c.confidence >= min_confidence);
    if !has_x {
        missing.push("X coordinate (Easting)".to_string());
    }

    let has_y = columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::Y && c.confidence >= min_confidence);
    if !has_y {
        missing.push("Y coordinate (Northing)".to_string());
    }

    let has_z = columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::Z && c.confidence >= min_confidence);
    if !has_z {
        missing.push("Z coordinate (Depth/Elevation)".to_string());
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

/// Get the index of a column by type
pub fn get_column_index(
    columns: &[DetectedSurfaceColumn],
    column_type: SurfaceColumnType,
) -> Option<usize> {
    columns
        .iter()
        .find(|c| c.column_type == column_type)
        .map(|c| c.index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_x_coordinate() {
        let cases = [
            ("X", 0.95),
            ("Easting", 0.95),
            ("EAST", 0.95),
            ("x_coord", 0.85),
            ("Longitude", 0.90),
        ];

        for (header, expected_conf) in cases {
            let (col_type, conf) = detect_surface_column_type(header);
            assert_eq!(col_type, SurfaceColumnType::X, "Failed for: {}", header);
            assert!(
                (conf - expected_conf).abs() < 0.1,
                "Confidence mismatch for: {} (got {}, expected {})",
                header,
                conf,
                expected_conf
            );
        }
    }

    #[test]
    fn test_detect_y_coordinate() {
        let cases = [
            ("Y", SurfaceColumnType::Y),
            ("Northing", SurfaceColumnType::Y),
            ("NORTH", SurfaceColumnType::Y),
            ("Latitude", SurfaceColumnType::Y),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_surface_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_z_coordinate() {
        let cases = [
            ("Z", SurfaceColumnType::Z),
            ("Depth", SurfaceColumnType::Z),
            ("Elevation", SurfaceColumnType::Z),
            ("TWT", SurfaceColumnType::Z),
            ("Value", SurfaceColumnType::Z),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_surface_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_surface_name() {
        let cases = [
            ("Surface", SurfaceColumnType::SurfaceName),
            ("Horizon", SurfaceColumnType::SurfaceName),
            ("Name", SurfaceColumnType::SurfaceName),
            ("Layer", SurfaceColumnType::SurfaceName),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_surface_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_extract_unit() {
        let (name, unit) = extract_unit_from_header("X (m)");
        assert_eq!(name, "X");
        assert_eq!(unit, Some("m".to_string()));

        let (name2, unit2) = extract_unit_from_header("Depth [ft]");
        assert_eq!(name2, "Depth");
        assert_eq!(unit2, Some("ft".to_string()));

        let (name3, unit3) = extract_unit_from_header("Easting");
        assert_eq!(name3, "Easting");
        assert_eq!(unit3, None);
    }

    #[test]
    fn test_detect_with_unit() {
        let detected = detect_surface_column_with_unit(0, "Easting (m)");
        assert_eq!(detected.column_type, SurfaceColumnType::X);
        assert_eq!(detected.unit, Some("m".to_string()));
        assert_eq!(detected.index, 0);
    }

    #[test]
    fn test_validate_required_columns() {
        let valid_columns = vec![
            DetectedSurfaceColumn {
                index: 0,
                original_name: "X".to_string(),
                column_type: SurfaceColumnType::X,
                confidence: 0.95,
                unit: None,
            },
            DetectedSurfaceColumn {
                index: 1,
                original_name: "Y".to_string(),
                column_type: SurfaceColumnType::Y,
                confidence: 0.95,
                unit: None,
            },
            DetectedSurfaceColumn {
                index: 2,
                original_name: "Z".to_string(),
                column_type: SurfaceColumnType::Z,
                confidence: 0.95,
                unit: None,
            },
        ];

        assert!(validate_required_columns(&valid_columns).is_ok());

        let missing_z = vec![
            DetectedSurfaceColumn {
                index: 0,
                original_name: "X".to_string(),
                column_type: SurfaceColumnType::X,
                confidence: 0.95,
                unit: None,
            },
            DetectedSurfaceColumn {
                index: 1,
                original_name: "Y".to_string(),
                column_type: SurfaceColumnType::Y,
                confidence: 0.95,
                unit: None,
            },
        ];

        let result = validate_required_columns(&missing_z);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(&"Z coordinate (Depth/Elevation)".to_string()));
    }

    #[test]
    fn test_unknown_column() {
        let (col_type, conf) = detect_surface_column_type("SomeRandomColumn");
        assert_eq!(col_type, SurfaceColumnType::Unknown);
        assert_eq!(conf, 0.0);
    }
}
