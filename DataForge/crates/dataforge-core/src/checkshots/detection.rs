//! Checkshot Column Type Detection
//!
//! Automatically detects checkshot column types from CSV header names.
//! Supports various naming conventions used in the industry.

use crate::models::CheckshotColumnType;

/// A detected column with type and confidence
#[derive(Debug, Clone)]
pub struct DetectedCheckColumn {
    /// Original column name from CSV
    pub original_name: String,
    /// Detected column type
    pub column_type: CheckshotColumnType,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Detected unit (if any)
    pub unit: Option<String>,
}

/// Detect column type from a header name
///
/// Returns the detected type and a confidence score (0.0 - 1.0).
/// Higher confidence means more certain match.
pub fn detect_checkshot_column_type(header: &str) -> (CheckshotColumnType, f64) {
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
    (CheckshotColumnType::Unknown, 0.0)
}

/// Detect column type and extract unit from header
///
/// Handles headers like "MD (ft)", "TWT [ms]", "TVD_SS (m)", etc.
pub fn detect_checkshot_column_with_unit(header: &str) -> DetectedCheckColumn {
    let (name, unit) = extract_unit_from_header(header);
    let (column_type, confidence) = detect_checkshot_column_type(&name);

    DetectedCheckColumn {
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

/// Extract unit from header like "MD (ft)" or "TWT [ms]"
fn extract_unit_from_header(header: &str) -> (String, Option<String>) {
    // Try parentheses: "MD (ft)"
    if let Some(idx) = header.find('(') {
        if let Some(end_idx) = header.find(')') {
            let name = header[..idx].trim().to_string();
            let unit = header[idx + 1..end_idx].trim().to_string();
            return (name, Some(unit));
        }
    }

    // Try square brackets: "TWT [ms]"
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
fn try_exact_match(normalized: &str) -> Option<(CheckshotColumnType, f64)> {
    match normalized {
        // Measured Depth - exact matches
        "md" | "measureddepth" | "dept" | "depth" => {
            Some((CheckshotColumnType::MeasuredDepth, 0.95))
        }

        // True Vertical Depth - exact matches
        "tvd" | "trueverticaldepth" | "tvdss" | "truevertdepth" | "tvdkb" => {
            Some((CheckshotColumnType::TrueVerticalDepth, 0.95))
        }

        // Two-Way Time - exact matches
        "twt" | "twowaytime" | "twtms" | "twts" | "time" | "ttime" => {
            Some((CheckshotColumnType::TwoWayTime, 0.95))
        }

        // One-Way Time - exact matches
        "owt" | "onewaytime" | "owtms" | "owts" => {
            Some((CheckshotColumnType::OneWayTime, 0.90))
        }

        // Velocity - exact matches
        "velocity" | "vel" | "vint" | "intervalvelocity" | "vavg" | "avgvelocity" => {
            Some((CheckshotColumnType::Velocity, 0.90))
        }

        // Quality - exact matches
        "quality" | "qual" | "qc" | "flag" | "confidence" => {
            Some((CheckshotColumnType::Quality, 0.85))
        }

        _ => None,
    }
}

/// Try pattern-based matches
fn try_pattern_match(normalized: &str) -> Option<(CheckshotColumnType, f64)> {
    // Measured Depth patterns
    if normalized.contains("measureddepth") || (normalized.contains("meas") && normalized.contains("depth")) {
        return Some((CheckshotColumnType::MeasuredDepth, 0.85));
    }

    // True Vertical Depth patterns
    if normalized.contains("truevertical") || normalized.contains("tvd") {
        return Some((CheckshotColumnType::TrueVerticalDepth, 0.85));
    }

    // Two-Way Time patterns
    if (normalized.contains("two") && normalized.contains("way")) || normalized.contains("twoway") {
        return Some((CheckshotColumnType::TwoWayTime, 0.85));
    }
    if normalized.contains("time") && !normalized.contains("one") {
        return Some((CheckshotColumnType::TwoWayTime, 0.75));
    }

    // One-Way Time patterns
    if (normalized.contains("one") && normalized.contains("way")) || normalized.contains("oneway") {
        return Some((CheckshotColumnType::OneWayTime, 0.85));
    }

    // Velocity patterns
    if normalized.contains("velocity") || normalized.contains("veloc") {
        return Some((CheckshotColumnType::Velocity, 0.80));
    }
    if normalized.contains("interval") && (normalized.contains("vel") || normalized.contains("v")) {
        return Some((CheckshotColumnType::Velocity, 0.75));
    }

    None
}

/// Check if required columns are present for checkshot data
///
/// Returns Ok if MD, TVD, and TWT columns are detected with sufficient confidence.
pub fn validate_required_columns(columns: &[DetectedCheckColumn]) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let min_confidence = 0.5;

    let has_md = columns.iter().any(|c| {
        c.column_type == CheckshotColumnType::MeasuredDepth && c.confidence >= min_confidence
    });
    if !has_md {
        missing.push("Measured Depth (MD)".to_string());
    }

    let has_tvd = columns.iter().any(|c| {
        c.column_type == CheckshotColumnType::TrueVerticalDepth && c.confidence >= min_confidence
    });
    if !has_tvd {
        missing.push("True Vertical Depth (TVD)".to_string());
    }

    let has_twt = columns.iter().any(|c| {
        c.column_type == CheckshotColumnType::TwoWayTime && c.confidence >= min_confidence
    });
    if !has_twt {
        missing.push("Two-Way Time (TWT)".to_string());
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
            let (col_type, conf) = detect_checkshot_column_type(header);
            assert_eq!(col_type, CheckshotColumnType::MeasuredDepth, "Failed for: {}", header);
            assert!((conf - expected_conf).abs() < 0.1, "Confidence mismatch for: {}", header);
        }
    }

    #[test]
    fn test_detect_tvd() {
        let cases = [
            ("TVD", CheckshotColumnType::TrueVerticalDepth),
            ("TVDSS", CheckshotColumnType::TrueVerticalDepth),
            ("True_Vertical_Depth", CheckshotColumnType::TrueVerticalDepth),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_checkshot_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_twt() {
        let cases = [
            ("TWT", CheckshotColumnType::TwoWayTime),
            ("Two_Way_Time", CheckshotColumnType::TwoWayTime),
            ("TWT_MS", CheckshotColumnType::TwoWayTime),
            ("TIME", CheckshotColumnType::TwoWayTime),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_checkshot_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_detect_velocity() {
        let cases = [
            ("Velocity", CheckshotColumnType::Velocity),
            ("VINT", CheckshotColumnType::Velocity),
            ("Interval_Velocity", CheckshotColumnType::Velocity),
        ];

        for (header, expected_type) in cases {
            let (col_type, _) = detect_checkshot_column_type(header);
            assert_eq!(col_type, expected_type, "Failed for: {}", header);
        }
    }

    #[test]
    fn test_extract_unit() {
        let (name, unit) = extract_unit_from_header("MD (ft)");
        assert_eq!(name, "MD");
        assert_eq!(unit, Some("ft".to_string()));

        let (name2, unit2) = extract_unit_from_header("TWT [ms]");
        assert_eq!(name2, "TWT");
        assert_eq!(unit2, Some("ms".to_string()));

        let (name3, unit3) = extract_unit_from_header("TVD");
        assert_eq!(name3, "TVD");
        assert_eq!(unit3, None);
    }

    #[test]
    fn test_detect_with_unit() {
        let detected = detect_checkshot_column_with_unit("Measured Depth (ft)");
        assert_eq!(detected.column_type, CheckshotColumnType::MeasuredDepth);
        assert_eq!(detected.unit, Some("ft".to_string()));
    }

    #[test]
    fn test_validate_required_columns() {
        let valid_columns = vec![
            DetectedCheckColumn {
                original_name: "MD".to_string(),
                column_type: CheckshotColumnType::MeasuredDepth,
                confidence: 0.95,
                unit: None,
            },
            DetectedCheckColumn {
                original_name: "TVD".to_string(),
                column_type: CheckshotColumnType::TrueVerticalDepth,
                confidence: 0.95,
                unit: None,
            },
            DetectedCheckColumn {
                original_name: "TWT".to_string(),
                column_type: CheckshotColumnType::TwoWayTime,
                confidence: 0.95,
                unit: None,
            },
        ];

        assert!(validate_required_columns(&valid_columns).is_ok());

        let missing_twt = vec![
            DetectedCheckColumn {
                original_name: "MD".to_string(),
                column_type: CheckshotColumnType::MeasuredDepth,
                confidence: 0.95,
                unit: None,
            },
            DetectedCheckColumn {
                original_name: "TVD".to_string(),
                column_type: CheckshotColumnType::TrueVerticalDepth,
                confidence: 0.95,
                unit: None,
            },
        ];

        let result = validate_required_columns(&missing_twt);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Two-Way Time (TWT)".to_string()));
    }

    #[test]
    fn test_unknown_column() {
        let (col_type, conf) = detect_checkshot_column_type("SomeRandomColumn");
        assert_eq!(col_type, CheckshotColumnType::Unknown);
        assert_eq!(conf, 0.0);
    }
}
