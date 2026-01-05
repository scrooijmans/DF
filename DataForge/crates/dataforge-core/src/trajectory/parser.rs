//! Trajectory CSV Parser
//!
//! Parses trajectory/survey CSV files with automatic delimiter detection.
//! Handles various CSV formats commonly used in the industry.

use crate::models::TrajectoryColumnType;
use crate::trajectory::detection::{detect_column_with_unit, DetectedColumn};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

/// Result of parsing a trajectory CSV file
#[derive(Debug, Clone)]
pub struct ParsedTrajectory {
    /// Detected columns with their types and metadata
    pub columns: Vec<ParsedTrajectoryColumn>,
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
pub struct ParsedTrajectoryColumn {
    /// Column index in the CSV
    pub index: usize,
    /// Original header name
    pub header: String,
    /// Detected column info
    pub detected: DetectedColumn,
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

/// Parse a trajectory CSV from a reader
pub fn parse_trajectory_csv<R: Read>(reader: R) -> Result<ParsedTrajectory, String> {
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
    let detected_columns: Vec<DetectedColumn> =
        headers.iter().map(|h| detect_column_with_unit(h)).collect();

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
    let parsed_columns: Vec<ParsedTrajectoryColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let values = &column_values[i];
            let stats = ColumnStats::compute(values);
            let detected = detected_columns[i].clone();

            // Warn if a required column type has many nulls
            if detected.column_type.is_input() && stats.null_count > 0 {
                warnings.push(format!(
                    "Column '{}' ({:?}) has {} null values",
                    header, detected.column_type, stats.null_count
                ));
            }

            ParsedTrajectoryColumn {
                index: i,
                header: header.clone(),
                detected,
                values: values.clone(),
                stats,
            }
        })
        .collect();

    Ok(ParsedTrajectory {
        columns: parsed_columns,
        rows,
        delimiter,
        header_row_index,
        warnings,
    })
}

/// Parse trajectory CSV from a file path
pub fn parse_trajectory_file(path: &str) -> Result<ParsedTrajectory, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    parse_trajectory_csv(file)
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

    // Pick delimiter with highest consistent count
    scores
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(delim, _)| delim)
        .ok_or_else(|| "Could not detect delimiter".to_string())
}

/// Find the header row in the file
fn find_header_row(lines: &[String], delimiter: char) -> Result<(usize, Vec<String>), String> {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip comment lines
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let values = split_csv_line(trimmed, delimiter);

        // Check if this looks like a header (contains text, not all numbers)
        let text_count = values
            .iter()
            .filter(|v| !v.is_empty() && parse_numeric_value(v).is_none())
            .count();

        // If at least half the values are text, assume it's a header
        if text_count > 0 && text_count >= values.len() / 2 {
            return Ok((i, values));
        }

        // If first non-empty line is all numbers, no header
        if values.iter().all(|v| v.is_empty() || parse_numeric_value(v).is_some()) {
            return Err("No header row found - first data row contains only numbers".to_string());
        }
    }

    Err("Could not find header row".to_string())
}

/// Split a CSV line handling quoted values
fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            if in_quotes {
                // Check for escaped quote
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                in_quotes = true;
            }
        } else if c == delimiter && !in_quotes {
            values.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(c);
        }
    }

    values.push(current.trim().to_string());
    values
}

/// Parse a string value to f64, handling various formats
fn parse_numeric_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();

    // Empty or null-like values
    if trimmed.is_empty()
        || trimmed.eq_ignore_ascii_case("null")
        || trimmed.eq_ignore_ascii_case("nan")
        || trimmed.eq_ignore_ascii_case("na")
        || trimmed.eq_ignore_ascii_case("n/a")
        || trimmed == "-"
        || trimmed == "--"
        || trimmed == "-999"
        || trimmed == "-999.25"
    {
        return None;
    }

    // Try standard parse
    if let Ok(v) = trimmed.parse::<f64>() {
        return Some(v);
    }

    // Try removing thousands separators (1,234.56)
    let no_commas = trimmed.replace(',', "");
    if let Ok(v) = no_commas.parse::<f64>() {
        return Some(v);
    }

    None
}

/// Get column indices for required trajectory columns
pub fn get_required_column_indices(
    columns: &[ParsedTrajectoryColumn],
) -> Result<RequiredColumnIndices, String> {
    let mut md_idx = None;
    let mut inc_idx = None;
    let mut azi_idx = None;

    for col in columns {
        match col.detected.column_type {
            TrajectoryColumnType::MeasuredDepth if col.detected.confidence >= 0.5 => {
                md_idx = Some(col.index);
            }
            TrajectoryColumnType::Inclination if col.detected.confidence >= 0.5 => {
                inc_idx = Some(col.index);
            }
            TrajectoryColumnType::Azimuth if col.detected.confidence >= 0.5 => {
                azi_idx = Some(col.index);
            }
            _ => {}
        }
    }

    match (md_idx, inc_idx, azi_idx) {
        (Some(md), Some(inc), Some(azi)) => Ok(RequiredColumnIndices {
            md_index: md,
            inc_index: inc,
            azi_index: azi,
        }),
        _ => {
            let mut missing = Vec::new();
            if md_idx.is_none() {
                missing.push("Measured Depth (MD)");
            }
            if inc_idx.is_none() {
                missing.push("Inclination (INC)");
            }
            if azi_idx.is_none() {
                missing.push("Azimuth (AZI)");
            }
            Err(format!("Missing required columns: {}", missing.join(", ")))
        }
    }
}

/// Indices of required columns for trajectory calculation
#[derive(Debug, Clone)]
pub struct RequiredColumnIndices {
    pub md_index: usize,
    pub inc_index: usize,
    pub azi_index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_delimiter_comma() {
        let lines = vec![
            "MD,INC,AZI".to_string(),
            "0,0,0".to_string(),
            "100,5,45".to_string(),
        ];
        assert_eq!(detect_delimiter(&lines).unwrap(), ',');
    }

    #[test]
    fn test_detect_delimiter_tab() {
        let lines = vec![
            "MD\tINC\tAZI".to_string(),
            "0\t0\t0".to_string(),
            "100\t5\t45".to_string(),
        ];
        assert_eq!(detect_delimiter(&lines).unwrap(), '\t');
    }

    #[test]
    fn test_detect_delimiter_semicolon() {
        let lines = vec![
            "MD;INC;AZI".to_string(),
            "0;0;0".to_string(),
            "100;5;45".to_string(),
        ];
        assert_eq!(detect_delimiter(&lines).unwrap(), ';');
    }

    #[test]
    fn test_parse_numeric_value() {
        assert_eq!(parse_numeric_value("123.45"), Some(123.45));
        assert_eq!(parse_numeric_value("  123  "), Some(123.0));
        assert_eq!(parse_numeric_value("1,234.56"), Some(1234.56));
        assert_eq!(parse_numeric_value(""), None);
        assert_eq!(parse_numeric_value("null"), None);
        assert_eq!(parse_numeric_value("NaN"), None);
        assert_eq!(parse_numeric_value("-999.25"), None);
        assert_eq!(parse_numeric_value("N/A"), None);
    }

    #[test]
    fn test_split_csv_line() {
        let result = split_csv_line("a,b,c", ',');
        assert_eq!(result, vec!["a", "b", "c"]);

        let result = split_csv_line("\"a,b\",c,d", ',');
        assert_eq!(result, vec!["a,b", "c", "d"]);

        let result = split_csv_line("a\tb\tc", '\t');
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_trajectory_csv() {
        let csv_data = "MD,INC,AZI,TVD\n0,0,0,0\n100,5.5,45.2,99.5\n200,10.1,50.3,198.2";
        let result = parse_trajectory_csv(csv_data.as_bytes()).unwrap();

        assert_eq!(result.columns.len(), 4);
        assert_eq!(result.rows.len(), 3);
        assert_eq!(result.delimiter, ',');

        // Check MD column
        assert_eq!(
            result.columns[0].detected.column_type,
            TrajectoryColumnType::MeasuredDepth
        );
        assert_eq!(result.columns[0].values.len(), 3);
        assert_eq!(result.columns[0].values[0], Some(0.0));
        assert_eq!(result.columns[0].values[1], Some(100.0));

        // Check INC column
        assert_eq!(
            result.columns[1].detected.column_type,
            TrajectoryColumnType::Inclination
        );

        // Check AZI column
        assert_eq!(
            result.columns[2].detected.column_type,
            TrajectoryColumnType::Azimuth
        );

        // Check TVD column
        assert_eq!(
            result.columns[3].detected.column_type,
            TrajectoryColumnType::TrueVerticalDepth
        );
    }

    #[test]
    fn test_parse_with_units_in_header() {
        let csv_data = "Measured Depth (ft),Inclination [deg],Azimuth (deg)\n0,0,0\n100,5,45";
        let result = parse_trajectory_csv(csv_data.as_bytes()).unwrap();

        assert_eq!(
            result.columns[0].detected.unit,
            Some("ft".to_string())
        );
        assert_eq!(
            result.columns[1].detected.unit,
            Some("deg".to_string())
        );
        assert_eq!(
            result.columns[0].detected.column_type,
            TrajectoryColumnType::MeasuredDepth
        );
    }

    #[test]
    fn test_column_stats() {
        let values = vec![Some(1.0), Some(2.0), Some(3.0), None, Some(4.0)];
        let stats = ColumnStats::compute(&values);

        assert_eq!(stats.count, 5);
        assert_eq!(stats.null_count, 1);
        assert_eq!(stats.min, Some(1.0));
        assert_eq!(stats.max, Some(4.0));
        assert_eq!(stats.mean, Some(2.5));
    }

    #[test]
    fn test_get_required_column_indices() {
        let csv_data = "MD,INC,AZI\n0,0,0\n100,5,45";
        let parsed = parse_trajectory_csv(csv_data.as_bytes()).unwrap();
        let indices = get_required_column_indices(&parsed.columns).unwrap();

        assert_eq!(indices.md_index, 0);
        assert_eq!(indices.inc_index, 1);
        assert_eq!(indices.azi_index, 2);
    }

    #[test]
    fn test_missing_required_columns() {
        let csv_data = "MD,TVD\n0,0\n100,99";
        let parsed = parse_trajectory_csv(csv_data.as_bytes()).unwrap();
        let result = get_required_column_indices(&parsed.columns);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Inclination"));
        assert!(err.contains("Azimuth"));
    }
}
