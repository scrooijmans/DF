//! Marker Column Type Detection
//!
//! Automatically detects marker column types from CSV header names.
//! Supports various naming conventions used in the industry.

use crate::models::MarkerColumnType;

/// A detected column with type and confidence
#[derive(Debug, Clone)]
pub struct DetectedMarkerColumn {
    /// Original column name from CSV
    pub original_name: String,
    /// Detected column type
    pub column_type: MarkerColumnType,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Detected unit (if any)
    pub unit: Option<String>,
}

/// Detect column type from a header name
///
/// Returns the detected type and a confidence score (0.0 - 1.0).
/// Higher confidence means more certain match.
pub fn detect_marker_column_type(header: &str) -> (MarkerColumnType, f64) {
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
    (MarkerColumnType::Unknown, 0.0)
}

/// Detect column type and extract unit from header
///
/// Handles headers like "Depth (ft)", "MD [m]", etc.
pub fn detect_marker_column_with_unit(header: &str) -> DetectedMarkerColumn {
    let (name, unit) = extract_unit_from_header(header);
    let (column_type, confidence) = detect_marker_column_type(&name);

    DetectedMarkerColumn {
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

/// Extract unit from header like "MD (ft)" or "Depth [m]"
fn extract_unit_from_header(header: &str) -> (String, Option<String>) {
    // Try parentheses: "MD (ft)"
    if let Some(idx) = header.find('(') {
        if let Some(end_idx) = header.find(')') {
            let name = header[..idx].trim().to_string();
            let unit = header[idx + 1..end_idx].trim().to_string();
            return (name, Some(unit));
        }
    }

    // Try square brackets: "Depth [m]"
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
fn try_exact_match(normalized: &str) -> Option<(MarkerColumnType, f64)> {
    match normalized {
        // Well name matches
        "well" | "wellname" | "wellid" | "wellidentifier" => {
            Some((MarkerColumnType::WellName, 0.95))
        }

        // Well UWI/API matches
        "uwi" | "api" | "apinumber" | "welluwi" => Some((MarkerColumnType::WellUwi, 0.95)),

        // Marker/Formation name matches (high confidence)
        "marker" | "markername" | "formation" | "formationname" | "top" | "topname" | "horizon"
        | "horizonname" | "zone" | "zonename" | "surface" | "surfacename" | "pick" | "pickname" => {
            Some((MarkerColumnType::MarkerName, 0.95))
        }

        // Measured Depth matches
        "md" | "depth" | "measureddepth" | "pickdepth" | "topdepth" | "markerdepth" | "depthtop" => {
            Some((MarkerColumnType::MeasuredDepth, 0.95))
        }

        // TVD matches
        "tvd" | "trueverticaldepth" | "truevertdepth" => {
            Some((MarkerColumnType::TrueVerticalDepth, 0.95))
        }

        // TVDSS matches
        "tvdss" | "tvdsubsea" | "sstvd" => Some((MarkerColumnType::TrueVerticalDepthSS, 0.95)),

        // Marker Type
        "type" | "markertype" | "toptype" | "picktype" | "formationtype" => {
            Some((MarkerColumnType::MarkerType, 0.90))
        }

        // Thickness
        "thickness" | "thick" | "isochore" | "thk" => Some((MarkerColumnType::Thickness, 0.90)),

        // Quality
        "quality" | "confidence" | "certainty" | "status" | "qc" => {
            Some((MarkerColumnType::Quality, 0.90))
        }

        // Interpreter
        "interpreter" | "pickedby" | "analyst" | "author" | "geologist" => {
            Some((MarkerColumnType::Interpreter, 0.90))
        }

        // Pick Date
        "date" | "pickdate" | "interpretationdate" | "updatedate" => {
            Some((MarkerColumnType::PickDate, 0.85))
        }

        // Comments
        "comment" | "comments" | "notes" | "remark" | "remarks" | "description" => {
            Some((MarkerColumnType::Comments, 0.90))
        }

        // Color
        "color" | "colour" | "displaycolor" => Some((MarkerColumnType::Color, 0.90)),

        _ => None,
    }
}

/// Try pattern-based matches
fn try_pattern_match(normalized: &str) -> Option<(MarkerColumnType, f64)> {
    // Well name patterns
    if normalized.contains("wellname") || normalized.contains("wellid") {
        return Some((MarkerColumnType::WellName, 0.85));
    }

    // Marker name patterns
    if normalized.contains("formation")
        || normalized.contains("marker")
        || normalized.contains("horizon")
        || normalized.contains("surface")
        || normalized.contains("pick")
    {
        // But not if it's a type column
        if !normalized.contains("type") {
            return Some((MarkerColumnType::MarkerName, 0.80));
        }
    }

    // Depth patterns (but not TVD)
    if normalized.contains("depth") && !normalized.contains("vertical") && !normalized.contains("tvd")
    {
        return Some((MarkerColumnType::MeasuredDepth, 0.80));
    }

    // TVD patterns
    if normalized.contains("tvd") || normalized.contains("vertical") {
        if normalized.contains("ss") || normalized.contains("subsea") {
            return Some((MarkerColumnType::TrueVerticalDepthSS, 0.80));
        }
        return Some((MarkerColumnType::TrueVerticalDepth, 0.80));
    }

    // Type patterns
    if normalized.contains("type") {
        return Some((MarkerColumnType::MarkerType, 0.75));
    }

    None
}

/// Check if required columns are present for marker ingestion
///
/// Returns Ok if MarkerName and MeasuredDepth columns are detected with sufficient confidence.
pub fn validate_required_columns(columns: &[DetectedMarkerColumn]) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let min_confidence = 0.5;

    let has_marker_name = columns.iter().any(|c| {
        c.column_type == MarkerColumnType::MarkerName && c.confidence >= min_confidence
    });
    if !has_marker_name {
        missing.push("Marker Name (Formation, Top, Horizon)".to_string());
    }

    let has_md = columns.iter().any(|c| {
        c.column_type == MarkerColumnType::MeasuredDepth && c.confidence >= min_confidence
    });
    if !has_md {
        missing.push("Measured Depth (MD)".to_string());
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
    fn test_detect_marker_name() {
        let cases = [
            ("Marker", 0.95),
            ("Formation", 0.95),
            ("TopName", 0.95),
            ("HORIZON", 0.95),
            ("zone", 0.95),
        ];

        for (header, expected_conf) in cases {
            let (col_type, conf) = detect_marker_column_type(header);
            assert_eq!(
                col_type,
                MarkerColumnType::MarkerName,
                "Failed for: {}",
                header
            );
            assert!(
                (conf - expected_conf).abs() < 0.1,
                "Confidence mismatch for: {}",
                header
            );
        }
    }

    #[test]
    fn test_detect_measured_depth() {
        let cases = [
            ("MD", MarkerColumnType::MeasuredDepth),
            ("Depth", MarkerColumnType::MeasuredDepth),
            ("PickDepth", MarkerColumnType::MeasuredDepth),
            ("MeasuredDepth", MarkerColumnType::MeasuredDepth),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_marker_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_well_name() {
        let cases = [
            ("Well", MarkerColumnType::WellName),
            ("WellName", MarkerColumnType::WellName),
            ("WellID", MarkerColumnType::WellName),
            ("UWI", MarkerColumnType::WellUwi),
            ("API", MarkerColumnType::WellUwi),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_marker_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_tvd() {
        assert_eq!(
            detect_marker_column_type("TVD").0,
            MarkerColumnType::TrueVerticalDepth
        );
        assert_eq!(
            detect_marker_column_type("TVDSS").0,
            MarkerColumnType::TrueVerticalDepthSS
        );
    }

    #[test]
    fn test_extract_unit() {
        let (name, unit) = extract_unit_from_header("MD (ft)");
        assert_eq!(name, "MD");
        assert_eq!(unit, Some("ft".to_string()));

        let (name2, unit2) = extract_unit_from_header("Depth [m]");
        assert_eq!(name2, "Depth");
        assert_eq!(unit2, Some("m".to_string()));

        let (name3, unit3) = extract_unit_from_header("Formation");
        assert_eq!(name3, "Formation");
        assert_eq!(unit3, None);
    }

    #[test]
    fn test_detect_with_unit() {
        let detected = detect_marker_column_with_unit("Measured Depth (ft)");
        assert_eq!(detected.column_type, MarkerColumnType::MeasuredDepth);
        assert_eq!(detected.unit, Some("ft".to_string()));
    }

    #[test]
    fn test_validate_required_columns() {
        let valid_columns = vec![
            DetectedMarkerColumn {
                original_name: "Formation".to_string(),
                column_type: MarkerColumnType::MarkerName,
                confidence: 0.95,
                unit: None,
            },
            DetectedMarkerColumn {
                original_name: "Depth".to_string(),
                column_type: MarkerColumnType::MeasuredDepth,
                confidence: 0.95,
                unit: None,
            },
        ];

        assert!(validate_required_columns(&valid_columns).is_ok());

        let missing_depth = vec![DetectedMarkerColumn {
            original_name: "Formation".to_string(),
            column_type: MarkerColumnType::MarkerName,
            confidence: 0.95,
            unit: None,
        }];

        let result = validate_required_columns(&missing_depth);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Measured Depth (MD)".to_string()));
    }

    #[test]
    fn test_unknown_column() {
        let (col_type, conf) = detect_marker_column_type("SomeRandomColumn");
        assert_eq!(col_type, MarkerColumnType::Unknown);
        assert_eq!(conf, 0.0);
    }
}
