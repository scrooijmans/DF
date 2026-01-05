//! Trajectory Column Type Detection
//!
//! Automatically detects trajectory column types from CSV header names.
//! Supports various naming conventions used in the industry.

use crate::models::TrajectoryColumnType;

/// A detected column with type and confidence
#[derive(Debug, Clone)]
pub struct DetectedColumn {
    /// Original column name from CSV
    pub original_name: String,
    /// Detected column type
    pub column_type: TrajectoryColumnType,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Detected unit (if any)
    pub unit: Option<String>,
}

/// Detect column type from a header name
///
/// Returns the detected type and a confidence score (0.0 - 1.0).
/// Higher confidence means more certain match.
pub fn detect_column_type(header: &str) -> (TrajectoryColumnType, f64) {
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
    (TrajectoryColumnType::Unknown, 0.0)
}

/// Detect column type and extract unit from header
///
/// Handles headers like "MD (ft)", "Inclination [deg]", etc.
pub fn detect_column_with_unit(header: &str) -> DetectedColumn {
    let (name, unit) = extract_unit_from_header(header);
    let (column_type, confidence) = detect_column_type(&name);

    DetectedColumn {
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

/// Extract unit from header like "MD (ft)" or "Inclination [deg]"
fn extract_unit_from_header(header: &str) -> (String, Option<String>) {
    // Try parentheses: "MD (ft)"
    if let Some(idx) = header.find('(') {
        if let Some(end_idx) = header.find(')') {
            let name = header[..idx].trim().to_string();
            let unit = header[idx + 1..end_idx].trim().to_string();
            return (name, Some(unit));
        }
    }

    // Try square brackets: "Inclination [deg]"
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
fn try_exact_match(normalized: &str) -> Option<(TrajectoryColumnType, f64)> {
    match normalized {
        // Measured Depth - exact matches
        "md" | "measureddepth" | "dept" | "depth" => {
            Some((TrajectoryColumnType::MeasuredDepth, 0.95))
        }

        // Inclination - exact matches
        "inc" | "incl" | "inclination" | "theta" | "dip" | "wellboreincl" => {
            Some((TrajectoryColumnType::Inclination, 0.95))
        }

        // Azimuth - exact matches
        "azi" | "azim" | "azimuth" | "phi" | "bearing" | "wellboreazi" | "azm" => {
            Some((TrajectoryColumnType::Azimuth, 0.95))
        }

        // True Vertical Depth - exact matches
        "tvd" | "trueverticaldepth" | "tvdss" | "truevertdepth" => {
            Some((TrajectoryColumnType::TrueVerticalDepth, 0.95))
        }

        // North-South - exact matches
        "ns" | "northsouth" | "departurens" | "northing" | "deltanorth" | "vsectns" => {
            Some((TrajectoryColumnType::NorthSouth, 0.95))
        }

        // East-West - exact matches
        "ew" | "eastwest" | "departureew" | "easting" | "deltaeast" | "vsectew" => {
            Some((TrajectoryColumnType::EastWest, 0.95))
        }

        // Dogleg Severity - exact matches
        "dls" | "dogleg" | "doglegseverity" | "dl" | "curvature" | "buildrate" => {
            Some((TrajectoryColumnType::DoglegSeverity, 0.95))
        }

        // Delta columns (incremental offsets)
        "dx" | "deltax" => Some((TrajectoryColumnType::DeltaX, 0.95)),
        "dy" | "deltay" => Some((TrajectoryColumnType::DeltaY, 0.95)),
        "dz" | "deltaz" => Some((TrajectoryColumnType::DeltaZ, 0.95)),

        // Coordinates
        "x" | "xcoord" | "localx" => Some((TrajectoryColumnType::LocalX, 0.90)),
        "y" | "ycoord" | "localy" => Some((TrajectoryColumnType::LocalY, 0.90)),
        "lat" | "latitude" => Some((TrajectoryColumnType::Latitude, 0.95)),
        "lon" | "lng" | "longitude" | "long" => Some((TrajectoryColumnType::Longitude, 0.95)),

        _ => None,
    }
}

/// Try pattern-based matches
fn try_pattern_match(normalized: &str) -> Option<(TrajectoryColumnType, f64)> {
    // Measured Depth patterns
    if normalized.contains("measureddepth") || normalized.contains("meas") && normalized.contains("depth") {
        return Some((TrajectoryColumnType::MeasuredDepth, 0.85));
    }

    // Inclination patterns
    if normalized.contains("inclination") || normalized.contains("inclin") {
        return Some((TrajectoryColumnType::Inclination, 0.85));
    }

    // Azimuth patterns
    if normalized.contains("azimuth") || normalized.contains("azim") {
        return Some((TrajectoryColumnType::Azimuth, 0.85));
    }

    // TVD patterns
    if normalized.contains("truevertical") || normalized.contains("tvd") {
        return Some((TrajectoryColumnType::TrueVerticalDepth, 0.85));
    }

    // NS patterns
    if (normalized.contains("north") && normalized.contains("south")) || normalized.contains("departure") && normalized.contains("n") {
        return Some((TrajectoryColumnType::NorthSouth, 0.80));
    }

    // EW patterns
    if (normalized.contains("east") && normalized.contains("west")) || normalized.contains("departure") && normalized.contains("e") {
        return Some((TrajectoryColumnType::EastWest, 0.80));
    }

    // DLS patterns
    if normalized.contains("dogleg") || normalized.contains("severity") {
        return Some((TrajectoryColumnType::DoglegSeverity, 0.80));
    }

    // Coordinate patterns
    if normalized.contains("latitude") {
        return Some((TrajectoryColumnType::Latitude, 0.85));
    }
    if normalized.contains("longitude") {
        return Some((TrajectoryColumnType::Longitude, 0.85));
    }

    None
}

/// Check if required columns are present for trajectory calculation
///
/// Returns Ok if MD, INC, and AZI columns are detected with sufficient confidence.
pub fn validate_required_columns(columns: &[DetectedColumn]) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let min_confidence = 0.5;

    let has_md = columns.iter().any(|c| {
        c.column_type == TrajectoryColumnType::MeasuredDepth && c.confidence >= min_confidence
    });
    if !has_md {
        missing.push("Measured Depth (MD)".to_string());
    }

    let has_inc = columns.iter().any(|c| {
        c.column_type == TrajectoryColumnType::Inclination && c.confidence >= min_confidence
    });
    if !has_inc {
        missing.push("Inclination (INC)".to_string());
    }

    let has_azi = columns.iter().any(|c| {
        c.column_type == TrajectoryColumnType::Azimuth && c.confidence >= min_confidence
    });
    if !has_azi {
        missing.push("Azimuth (AZI)".to_string());
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_md() {
        let cases = [
            ("MD", 0.95),
            ("Measured Depth", 0.95),
            ("DEPT", 0.95),
            ("depth", 0.95),
            ("measured_depth", 0.95),
        ];

        for (header, expected_conf) in cases {
            let (col_type, conf) = detect_column_type(header);
            assert_eq!(col_type, TrajectoryColumnType::MeasuredDepth, "Failed for: {}", header);
            assert!((conf - expected_conf).abs() < 0.1, "Confidence mismatch for: {}", header);
        }
    }

    #[test]
    fn test_detect_inclination() {
        let cases = [
            ("INC", TrajectoryColumnType::Inclination),
            ("INCL", TrajectoryColumnType::Inclination),
            ("Inclination", TrajectoryColumnType::Inclination),
            ("theta", TrajectoryColumnType::Inclination),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_azimuth() {
        let cases = [
            ("AZI", TrajectoryColumnType::Azimuth),
            ("AZIM", TrajectoryColumnType::Azimuth),
            ("Azimuth", TrajectoryColumnType::Azimuth),
            ("bearing", TrajectoryColumnType::Azimuth),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_calculated_columns() {
        assert_eq!(detect_column_type("TVD").0, TrajectoryColumnType::TrueVerticalDepth);
        assert_eq!(detect_column_type("NS").0, TrajectoryColumnType::NorthSouth);
        assert_eq!(detect_column_type("EW").0, TrajectoryColumnType::EastWest);
        assert_eq!(detect_column_type("DLS").0, TrajectoryColumnType::DoglegSeverity);
    }

    #[test]
    fn test_extract_unit() {
        let (name, unit) = extract_unit_from_header("MD (ft)");
        assert_eq!(name, "MD");
        assert_eq!(unit, Some("ft".to_string()));

        let (name2, unit2) = extract_unit_from_header("Inclination [deg]");
        assert_eq!(name2, "Inclination");
        assert_eq!(unit2, Some("deg".to_string()));

        let (name3, unit3) = extract_unit_from_header("TVD");
        assert_eq!(name3, "TVD");
        assert_eq!(unit3, None);
    }

    #[test]
    fn test_detect_with_unit() {
        let detected = detect_column_with_unit("Measured Depth (ft)");
        assert_eq!(detected.column_type, TrajectoryColumnType::MeasuredDepth);
        assert_eq!(detected.unit, Some("ft".to_string()));
    }

    #[test]
    fn test_validate_required_columns() {
        let valid_columns = vec![
            DetectedColumn {
                original_name: "MD".to_string(),
                column_type: TrajectoryColumnType::MeasuredDepth,
                confidence: 0.95,
                unit: None,
            },
            DetectedColumn {
                original_name: "INC".to_string(),
                column_type: TrajectoryColumnType::Inclination,
                confidence: 0.95,
                unit: None,
            },
            DetectedColumn {
                original_name: "AZI".to_string(),
                column_type: TrajectoryColumnType::Azimuth,
                confidence: 0.95,
                unit: None,
            },
        ];

        assert!(validate_required_columns(&valid_columns).is_ok());

        let missing_azi = vec![
            DetectedColumn {
                original_name: "MD".to_string(),
                column_type: TrajectoryColumnType::MeasuredDepth,
                confidence: 0.95,
                unit: None,
            },
            DetectedColumn {
                original_name: "INC".to_string(),
                column_type: TrajectoryColumnType::Inclination,
                confidence: 0.95,
                unit: None,
            },
        ];

        let result = validate_required_columns(&missing_azi);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Azimuth (AZI)".to_string()));
    }

    #[test]
    fn test_unknown_column() {
        let (col_type, conf) = detect_column_type("SomeRandomColumn");
        assert_eq!(col_type, TrajectoryColumnType::Unknown);
        assert_eq!(conf, 0.0);
    }
}
