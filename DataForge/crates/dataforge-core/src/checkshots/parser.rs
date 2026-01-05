//! Checkshot CSV Parser
//!
//! Parses checkshot CSV files with automatic delimiter detection.
//! Handles various CSV formats commonly used in the industry.

use crate::models::CheckshotColumnType;
use crate::checkshots::detection::{detect_checkshot_column_with_unit, DetectedCheckColumn};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

/// Result of parsing a checkshot CSV file
#[derive(Debug, Clone)]
pub struct ParsedCheckshot {
    /// Detected columns with their types and metadata
    pub columns: Vec<ParsedCheckColumn>,
    /// Raw data rows (each row is a vector of string values)
    pub rows: Vec<Vec<String>>,
    /// Detected delimiter
    pub delimiter: char,
    /// Number of header rows skipped
    pub header_row_index: usize,
    /// Any parsing warnings
    pub warnings: Vec<String>,
}

/// A parsed column with detection results and data
#[derive(Debug, Clone)]
pub struct ParsedCheckColumn {
    /// Column index in the CSV
    pub index: usize,
    /// Original header name
    pub header: String,
    /// Detected column info
    pub detected: DetectedCheckColumn,
    /// Parsed numeric values (None for unparseable values)
    pub values: Vec<Option<f64>>,
    /// Statistics
    pub stats: ColumnStats,
}

/// Column statistics
#[derive(Debug, Clone, Default)]
pub struct ColumnStats {
    pub count: usize,
    pub null_count: usize,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub mean: Option<f64>,
}

impl ColumnStats {
    fn compute(values: &[Option<f64>]) -> Self {
        let valid_values: Vec<f64> = values.iter().filter_map(|v| *v).collect();
        let count = values.len();
        let null_count = count - valid_values.len();

        if valid_values.is_empty() {
            return ColumnStats {
                count,
                null_count,
                min: None,
                max: None,
                mean: None,
            };
        }

        let min = valid_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = valid_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = valid_values.iter().sum();
        let mean = sum / valid_values.len() as f64;

        ColumnStats {
            count,
            null_count,
            min: Some(min),
            max: Some(max),
            mean: Some(mean),
        }
    }
}

/// Parse a checkshot CSV from a reader
pub fn parse_checkshot_csv<R: Read>(reader: R) -> Result<ParsedCheckshot, String> {
    let buf_reader = BufReader::new(reader);
    let lines: Vec<String> = buf_reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if lines.is_empty() {
        return Err("Empty file".to_string());
    }

    // Detect delimiter from first few lines
    let delimiter = detect_delimiter(&lines)?;

    // Find header row (first non-empty row)
    let (header_row_index, headers) = find_header_row(&lines, delimiter)?;

    // Detect column types from headers
    let detected_columns: Vec<DetectedCheckColumn> =
        headers.iter().map(|h| detect_checkshot_column_with_unit(h)).collect();

    // Parse data rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut column_values: Vec<Vec<Option<f64>>> = vec![Vec::new(); headers.len()];

    for line in lines.iter().skip(header_row_index + 1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let values: Vec<String> = split_csv_line(trimmed, delimiter);

        // Pad or truncate to match header count
        let mut row_values: Vec<String> = values;
        row_values.resize(headers.len(), String::new());

        // Parse numeric values for each column
        for (i, val) in row_values.iter().enumerate() {
            let parsed = parse_numeric_value(val);
            column_values[i].push(parsed);
        }

        rows.push(row_values);
    }

    // Build parsed columns with statistics
    let mut warnings = Vec::new();
    let parsed_columns: Vec<ParsedCheckColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let values = &column_values[i];
            let stats = ColumnStats::compute(values);
            let detected = detected_columns[i].clone();

            // Warn if a required column type has many nulls
            if detected.column_type.is_required() && stats.null_count > 0 {
                warnings.push(format!(
                    "Column '{}' ({:?}) has {} null values",
                    header, detected.column_type, stats.null_count
                ));
            }

            ParsedCheckColumn {
                index: i,
                header: header.clone(),
                detected,
                values: values.clone(),
                stats,
            }
        })
        .collect();

    // Validate checkshot-specific constraints
    validate_checkshot_data(&parsed_columns, &mut warnings);

    Ok(ParsedCheckshot {
        columns: parsed_columns,
        rows,
        delimiter,
        header_row_index,
        warnings,
    })
}

/// Parse checkshot CSV from a file path
pub fn parse_checkshot_file(path: &str) -> Result<ParsedCheckshot, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    parse_checkshot_csv(file)
}

/// Validate checkshot-specific data constraints
fn validate_checkshot_data(columns: &[ParsedCheckColumn], warnings: &mut Vec<String>) {
    // Find MD, TVD, TWT columns
    let md_col = columns.iter().find(|c| c.detected.column_type == CheckshotColumnType::MeasuredDepth);
    let tvd_col = columns.iter().find(|c| c.detected.column_type == CheckshotColumnType::TrueVerticalDepth);
    let twt_col = columns.iter().find(|c| c.detected.column_type == CheckshotColumnType::TwoWayTime);

    // Check MD is non-negative
    if let Some(md) = md_col {
        if let Some(min) = md.stats.min {
            if min < 0.0 {
                warnings.push(format!("Measured Depth has negative values (min: {:.2})", min));
            }
        }
    }

    // Check TWT is non-negative
    if let Some(twt) = twt_col {
        if let Some(min) = twt.stats.min {
            if min < 0.0 {
                warnings.push(format!("Two-Way Time has negative values (min: {:.4})", min));
            }
        }
    }

    // Check TVD range (from OSDU: -10000 to 50000)
    if let Some(tvd) = tvd_col {
        if let Some(min) = tvd.stats.min {
            if min < -10000.0 {
                warnings.push(format!("TVD values below -10000 detected (min: {:.2})", min));
            }
        }
        if let Some(max) = tvd.stats.max {
            if max > 50000.0 {
                warnings.push(format!("TVD values above 50000 detected (max: {:.2})", max));
            }
        }
    }

    // Check TVD <= MD for each row (for deviated wells)
    if let (Some(md), Some(tvd)) = (md_col, tvd_col) {
        let violations: Vec<usize> = md.values.iter()
            .zip(tvd.values.iter())
            .enumerate()
            .filter_map(|(i, (md_val, tvd_val))| {
                match (md_val, tvd_val) {
                    (Some(md), Some(tvd)) if *tvd > *md + 0.01 => Some(i + 1), // +0.01 for float tolerance
                    _ => None,
                }
            })
            .collect();

        if !violations.is_empty() {
            let example_rows: Vec<String> = violations.iter().take(3).map(|r| r.to_string()).collect();
            warnings.push(format!(
                "TVD > MD detected in {} rows (e.g., rows: {}). This is unusual.",
                violations.len(),
                example_rows.join(", ")
            ));
        }
    }

    // Check TWT increases with depth (monotonic)
    if let (Some(md), Some(twt)) = (md_col, twt_col) {
        let mut decreases = 0;
        let mut prev_twt: Option<f64> = None;
        let mut prev_md: Option<f64> = None;

        for (md_val, twt_val) in md.values.iter().zip(twt.values.iter()) {
            if let (Some(curr_md), Some(curr_twt)) = (md_val, twt_val) {
                if let (Some(p_md), Some(p_twt)) = (prev_md, prev_twt) {
                    // If MD increased but TWT decreased
                    if *curr_md > p_md && *curr_twt < p_twt - 0.0001 {
                        decreases += 1;
                    }
                }
                prev_md = Some(*curr_md);
                prev_twt = Some(*curr_twt);
            }
        }

        if decreases > 0 {
            warnings.push(format!(
                "TWT decreases with increasing depth {} times. This may indicate data issues.",
                decreases
            ));
        }
    }
}

/// Detect the delimiter used in the CSV
fn detect_delimiter(lines: &[String]) -> Result<char, String> {
    let delimiters = [',', '\t', ';', '|'];
    let mut scores: HashMap<char, usize> = HashMap::new();

    // Sample first few non-empty lines
    let sample_lines: Vec<&String> = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .take(5)
        .collect();

    if sample_lines.is_empty() {
        return Err("No data lines found".to_string());
    }

    for delim in delimiters {
        // Count occurrences in each line
        let counts: Vec<usize> = sample_lines
            .iter()
            .map(|l| l.matches(delim).count())
            .collect();

        // Check if delimiter count is consistent and > 0
        if !counts.is_empty() && counts[0] > 0 {
            let first = counts[0];
            let consistent = counts.iter().all(|&c| c == first || c == 0);
            if consistent {
                scores.insert(delim, first);
            }
        }
    }

    // Pick delimiter with highest count
    scores
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(delim, _)| delim)
        .ok_or_else(|| "Could not detect delimiter".to_string())
}

/// Find the header row in the CSV
fn find_header_row(lines: &[String], delimiter: char) -> Result<(usize, Vec<String>), String> {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Parse the line
        let fields: Vec<String> = split_csv_line(trimmed, delimiter);

        // Check if most fields are non-numeric (header-like)
        let non_numeric_count = fields
            .iter()
            .filter(|f| !f.trim().is_empty() && parse_numeric_value(f).is_none())
            .count();

        // If more than half are non-numeric, treat as header
        if fields.len() > 0 && non_numeric_count > fields.len() / 2 {
            return Ok((i, fields));
        }
    }

    Err("Could not find header row".to_string())
}

/// Split a CSV line respecting quotes
fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            if in_quotes {
                // Check for escaped quote
                if chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                in_quotes = true;
            }
        } else if c == delimiter && !in_quotes {
            fields.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(c);
        }
    }

    fields.push(current.trim().to_string());
    fields
}

/// Parse a string as a numeric value
fn parse_numeric_value(s: &str) -> Option<f64> {
    let trimmed = s.trim();

    // Check for null-like values
    let lower = trimmed.to_lowercase();
    if lower.is_empty()
        || lower == "null"
        || lower == "nan"
        || lower == "na"
        || lower == "n/a"
        || lower == "-"
        || lower == "--"
        || lower == "-999"
        || lower == "-999.25"
        || lower == "-9999"
        || lower == "-9999.0"
    {
        return None;
    }

    // Try standard parse
    if let Ok(v) = trimmed.parse::<f64>() {
        return Some(v);
    }

    // Try removing thousand separators
    let no_commas = trimmed.replace(',', "");
    if let Ok(v) = no_commas.parse::<f64>() {
        return Some(v);
    }

    None
}

/// Get indices of required columns (MD, TVD, TWT)
pub fn get_required_column_indices(
    columns: &[ParsedCheckColumn],
) -> Result<(usize, usize, usize), String> {
    let md_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::MeasuredDepth)
        .ok_or("Missing Measured Depth (MD) column")?;

    let tvd_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::TrueVerticalDepth)
        .ok_or("Missing True Vertical Depth (TVD) column")?;

    let twt_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::TwoWayTime)
        .ok_or("Missing Two-Way Time (TWT) column")?;

    Ok((md_idx, tvd_idx, twt_idx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_csv() {
        let csv = "MD,TVD,TWT\n0,0,0\n100,100,0.05\n200,200,0.10\n";
        let reader = Cursor::new(csv);
        let result = parse_checkshot_csv(reader).unwrap();

        assert_eq!(result.columns.len(), 3);
        assert_eq!(result.rows.len(), 3);
        assert_eq!(result.delimiter, ',');

        // Check column detection
        assert_eq!(result.columns[0].detected.column_type, CheckshotColumnType::MeasuredDepth);
        assert_eq!(result.columns[1].detected.column_type, CheckshotColumnType::TrueVerticalDepth);
        assert_eq!(result.columns[2].detected.column_type, CheckshotColumnType::TwoWayTime);
    }

    #[test]
    fn test_parse_tab_delimited() {
        let csv = "MD\tTVD\tTWT\n0\t0\t0\n100\t100\t0.05\n";
        let reader = Cursor::new(csv);
        let result = parse_checkshot_csv(reader).unwrap();

        assert_eq!(result.delimiter, '\t');
        assert_eq!(result.columns.len(), 3);
    }

    #[test]
    fn test_parse_with_nulls() {
        let csv = "MD,TVD,TWT\n0,0,0\n100,-999,0.05\n200,200,null\n";
        let reader = Cursor::new(csv);
        let result = parse_checkshot_csv(reader).unwrap();

        // Check that nulls are detected
        assert!(result.columns[1].values[1].is_none());
        assert!(result.columns[2].values[2].is_none());
    }

    #[test]
    fn test_get_required_column_indices() {
        let csv = "MD,TVD,TWT,Quality\n0,0,0,good\n";
        let reader = Cursor::new(csv);
        let result = parse_checkshot_csv(reader).unwrap();

        let (md_idx, tvd_idx, twt_idx) = get_required_column_indices(&result.columns).unwrap();
        assert_eq!(md_idx, 0);
        assert_eq!(tvd_idx, 1);
        assert_eq!(twt_idx, 2);
    }

    #[test]
    fn test_column_statistics() {
        let csv = "MD,TVD,TWT\n0,0,0\n100,100,0.05\n200,200,0.10\n300,300,0.15\n";
        let reader = Cursor::new(csv);
        let result = parse_checkshot_csv(reader).unwrap();

        let md_stats = &result.columns[0].stats;
        assert_eq!(md_stats.count, 4);
        assert_eq!(md_stats.null_count, 0);
        assert!((md_stats.min.unwrap() - 0.0).abs() < 0.001);
        assert!((md_stats.max.unwrap() - 300.0).abs() < 0.001);
        assert!((md_stats.mean.unwrap() - 150.0).abs() < 0.001);
    }
}
