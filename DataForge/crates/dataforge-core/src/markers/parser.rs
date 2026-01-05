//! Marker CSV Parser
//!
//! Parses marker/well tops CSV files with automatic delimiter detection.
//! Supports multi-well files where one file contains markers for multiple wells.

use crate::markers::detection::{detect_marker_column_with_unit, DetectedMarkerColumn};
use crate::models::MarkerColumnType;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

/// Result of parsing a marker CSV file
#[derive(Debug, Clone)]
pub struct ParsedMarkerFile {
    /// Detected columns with their types and metadata
    pub columns: Vec<ParsedMarkerColumn>,
    /// Parsed data rows
    pub rows: Vec<ParsedMarkerRow>,
    /// Detected delimiter
    pub delimiter: char,
    /// Whether file has a header row
    pub has_header: bool,
    /// Header row index (0 if first row is header)
    pub header_row_index: usize,
    /// Whether this appears to be a multi-well file
    pub is_multi_well: bool,
    /// Unique well names found (if multi-well)
    pub well_names: Vec<String>,
    /// Any parsing warnings
    pub warnings: Vec<String>,
}

/// A parsed column with detection results
#[derive(Debug, Clone)]
pub struct ParsedMarkerColumn {
    /// Column index in the CSV
    pub index: usize,
    /// Original header name (if present)
    pub header: String,
    /// Detected column info
    pub detected: DetectedMarkerColumn,
    /// Sample values for preview
    pub sample_values: Vec<String>,
    /// Statistics for numeric columns
    pub stats: Option<ColumnStats>,
}

/// A parsed marker row
#[derive(Debug, Clone)]
pub struct ParsedMarkerRow {
    /// Row index in original file
    pub row_index: usize,
    /// Well name (for multi-well files)
    pub well_name: Option<String>,
    /// Well UWI (if present)
    pub well_uwi: Option<String>,
    /// Marker name
    pub marker_name: Option<String>,
    /// Measured depth
    pub measured_depth: Option<f64>,
    /// True vertical depth (if present)
    pub tvd: Option<f64>,
    /// TVDSS (if present)
    pub tvdss: Option<f64>,
    /// Marker type
    pub marker_type: Option<String>,
    /// Thickness
    pub thickness: Option<f64>,
    /// Quality indicator
    pub quality: Option<String>,
    /// Interpreter
    pub interpreter: Option<String>,
    /// Comments
    pub comments: Option<String>,
    /// All raw values (for preservation)
    pub raw_values: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ColumnStats {
    pub count: usize,
    pub null_count: usize,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub mean: Option<f64>,
}

/// Parse a marker CSV from a reader
pub fn parse_marker_csv<R: Read>(reader: R) -> Result<ParsedMarkerFile, String> {
    let buf_reader = BufReader::new(reader);
    let lines: Vec<String> = buf_reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if lines.is_empty() {
        return Err("Empty file".to_string());
    }

    // Detect delimiter
    let delimiter = detect_delimiter(&lines)?;

    // Find header row and determine if file has headers
    let (header_row_index, headers, has_header) = find_header_row(&lines, delimiter)?;

    // Detect column types
    let detected_columns: Vec<DetectedMarkerColumn> = headers
        .iter()
        .map(|h| detect_marker_column_with_unit(h))
        .collect();

    // Get column indices for key fields
    let well_name_idx = find_column_index(&detected_columns, MarkerColumnType::WellName);
    let well_uwi_idx = find_column_index(&detected_columns, MarkerColumnType::WellUwi);
    let marker_name_idx = find_column_index(&detected_columns, MarkerColumnType::MarkerName);
    let md_idx = find_column_index(&detected_columns, MarkerColumnType::MeasuredDepth);
    let tvd_idx = find_column_index(&detected_columns, MarkerColumnType::TrueVerticalDepth);
    let tvdss_idx = find_column_index(&detected_columns, MarkerColumnType::TrueVerticalDepthSS);
    let type_idx = find_column_index(&detected_columns, MarkerColumnType::MarkerType);
    let thickness_idx = find_column_index(&detected_columns, MarkerColumnType::Thickness);
    let quality_idx = find_column_index(&detected_columns, MarkerColumnType::Quality);
    let interpreter_idx = find_column_index(&detected_columns, MarkerColumnType::Interpreter);
    let comments_idx = find_column_index(&detected_columns, MarkerColumnType::Comments);

    // Parse data rows
    let mut rows = Vec::new();
    let mut well_names_set = HashSet::new();
    let warnings = Vec::new();

    let data_start = if has_header { header_row_index + 1 } else { 0 };

    for (i, line) in lines.iter().enumerate().skip(data_start) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let values = split_csv_line(trimmed, delimiter);

        // Extract well name if present
        let well_name = well_name_idx.and_then(|idx| {
            values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
        });
        if let Some(ref wn) = well_name {
            well_names_set.insert(wn.clone());
        }

        let row = ParsedMarkerRow {
            row_index: i,
            well_name,
            well_uwi: well_uwi_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            marker_name: marker_name_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            measured_depth: md_idx.and_then(|idx| parse_f64(values.get(idx))),
            tvd: tvd_idx.and_then(|idx| parse_f64(values.get(idx))),
            tvdss: tvdss_idx.and_then(|idx| parse_f64(values.get(idx))),
            marker_type: type_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            thickness: thickness_idx.and_then(|idx| parse_f64(values.get(idx))),
            quality: quality_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            interpreter: interpreter_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            comments: comments_idx.and_then(|idx| {
                values.get(idx).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
            }),
            raw_values: values,
        };

        rows.push(row);
    }

    let well_names: Vec<String> = well_names_set.into_iter().collect();
    let is_multi_well = well_names.len() > 1 || well_name_idx.is_some();

    // Build parsed columns with sample values and stats
    let parsed_columns: Vec<ParsedMarkerColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let detected = detected_columns[i].clone();
            let sample_values: Vec<String> = rows
                .iter()
                .take(5)
                .filter_map(|r| r.raw_values.get(i).cloned())
                .collect();

            // Calculate stats for numeric columns
            let stats = if detected.column_type == MarkerColumnType::MeasuredDepth
                || detected.column_type == MarkerColumnType::TrueVerticalDepth
                || detected.column_type == MarkerColumnType::Thickness
            {
                Some(calculate_stats(&rows, i))
            } else {
                None
            };

            ParsedMarkerColumn {
                index: i,
                header: header.clone(),
                detected,
                sample_values,
                stats,
            }
        })
        .collect();

    Ok(ParsedMarkerFile {
        columns: parsed_columns,
        rows,
        delimiter,
        has_header,
        header_row_index,
        is_multi_well,
        well_names,
        warnings,
    })
}

/// Detect the delimiter used in the CSV
fn detect_delimiter(lines: &[String]) -> Result<char, String> {
    let delimiters = ['\t', ',', ';', '|'];
    let mut scores: HashMap<char, usize> = HashMap::new();

    // Sample first few non-empty lines
    let sample_lines: Vec<&String> = lines
        .iter()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .take(5)
        .collect();

    if sample_lines.is_empty() {
        return Err("No data lines found".to_string());
    }

    for delim in delimiters {
        let counts: Vec<usize> = sample_lines
            .iter()
            .map(|line| line.matches(delim).count())
            .collect();

        // Check consistency - all lines should have same number of delimiters
        if !counts.is_empty() && counts.iter().all(|&c| c == counts[0]) && counts[0] > 0 {
            scores.insert(delim, counts[0]);
        }
    }

    // Return delimiter with most occurrences
    if let Some((delim, _)) = scores.into_iter().max_by_key(|(_, count)| *count) {
        return Ok(delim);
    }

    // If no consistent delimiter found, try to detect based on first line
    let first_line = sample_lines[0];
    for delim in delimiters {
        if first_line.contains(delim) {
            return Ok(delim);
        }
    }

    // Default to tab for headerless marker files
    Ok('\t')
}

/// Find the header row and extract headers
fn find_header_row(
    lines: &[String],
    delimiter: char,
) -> Result<(usize, Vec<String>, bool), String> {
    // Get first non-empty, non-comment line
    let first_data_idx = lines
        .iter()
        .position(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .ok_or_else(|| "No data lines found".to_string())?;

    let first_line = &lines[first_data_idx];
    let values = split_csv_line(first_line, delimiter);

    // Check if first line looks like a header
    // Headers typically have text, not just numbers
    let is_header = is_likely_header(&values);

    if is_header {
        Ok((first_data_idx, values, true))
    } else {
        // No header - generate column names
        // For simple 2-column files (depth, marker), assume that pattern
        let generated: Vec<String> = if values.len() == 2 {
            // Common pattern: depth, marker_name
            vec!["Depth".to_string(), "Marker".to_string()]
        } else {
            values
                .iter()
                .enumerate()
                .map(|(i, _)| format!("Column{}", i + 1))
                .collect()
        };
        Ok((0, generated, false))
    }
}

/// Check if a row looks like a header row
fn is_likely_header(values: &[String]) -> bool {
    if values.is_empty() {
        return false;
    }

    // Count how many values are numeric
    let numeric_count = values
        .iter()
        .filter(|v| v.trim().parse::<f64>().is_ok())
        .count();

    // If most values are not numeric, it's likely a header
    let non_numeric_ratio = 1.0 - (numeric_count as f64 / values.len() as f64);

    // Also check for common header keywords
    let has_header_keywords = values.iter().any(|v| {
        let lower = v.to_lowercase();
        lower.contains("depth")
            || lower.contains("marker")
            || lower.contains("formation")
            || lower.contains("well")
            || lower.contains("top")
            || lower.contains("name")
    });

    non_numeric_ratio > 0.5 || has_header_keywords
}

/// Split a CSV line respecting quoted values
fn split_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
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
            result.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(c);
        }
    }
    result.push(current.trim().to_string());

    result
}

fn find_column_index(
    columns: &[DetectedMarkerColumn],
    col_type: MarkerColumnType,
) -> Option<usize> {
    columns
        .iter()
        .position(|c| c.column_type == col_type && c.confidence >= 0.5)
}

fn parse_f64(s: Option<&String>) -> Option<f64> {
    s?.trim().parse().ok()
}

fn calculate_stats(rows: &[ParsedMarkerRow], column_index: usize) -> ColumnStats {
    let values: Vec<f64> = rows
        .iter()
        .filter_map(|r| r.raw_values.get(column_index))
        .filter_map(|v| v.trim().parse::<f64>().ok())
        .collect();

    let null_count = rows.len() - values.len();

    if values.is_empty() {
        return ColumnStats {
            count: 0,
            null_count,
            min: None,
            max: None,
            mean: None,
        };
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sum: f64 = values.iter().sum();
    let mean = sum / values.len() as f64;

    ColumnStats {
        count: values.len(),
        null_count,
        min: Some(min),
        max: Some(max),
        mean: Some(mean),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_marker_file() {
        let csv = "30\tSeasurface\n553.6\tMFS11\n612.9\tFS11\n683.31\tMFS10\n";
        let cursor = Cursor::new(csv);
        let result = parse_marker_csv(cursor).unwrap();

        assert!(!result.has_header);
        assert_eq!(result.delimiter, '\t');
        assert_eq!(result.rows.len(), 4);
        assert!(!result.is_multi_well);

        // First row should have depth 30 and marker "Seasurface"
        assert_eq!(result.rows[0].measured_depth, Some(30.0));
        assert_eq!(
            result.rows[0].marker_name,
            Some("Seasurface".to_string())
        );
    }

    #[test]
    fn test_parse_with_headers() {
        let csv = "Depth,Formation,Type\n100.5,TopSand,formation\n200.3,BaseShale,sequence\n";
        let cursor = Cursor::new(csv);
        let result = parse_marker_csv(cursor).unwrap();

        assert!(result.has_header);
        assert_eq!(result.delimiter, ',');
        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.columns.len(), 3);

        assert_eq!(result.rows[0].measured_depth, Some(100.5));
        assert_eq!(result.rows[0].marker_name, Some("TopSand".to_string()));
        assert_eq!(result.rows[0].marker_type, Some("formation".to_string()));
    }

    #[test]
    fn test_parse_multi_well_file() {
        let csv = "Well,Depth,Marker\nF02-1,100,TopA\nF02-1,200,TopB\nF03-2,150,TopA\n";
        let cursor = Cursor::new(csv);
        let result = parse_marker_csv(cursor).unwrap();

        assert!(result.has_header);
        assert!(result.is_multi_well);
        assert_eq!(result.well_names.len(), 2);
        assert!(result.well_names.contains(&"F02-1".to_string()));
        assert!(result.well_names.contains(&"F03-2".to_string()));
    }

    #[test]
    fn test_delimiter_detection() {
        let tab_csv = "A\tB\tC\n1\t2\t3\n";
        let comma_csv = "A,B,C\n1,2,3\n";
        let semi_csv = "A;B;C\n1;2;3\n";

        assert_eq!(detect_delimiter(&[tab_csv.to_string()]).unwrap(), '\t');
        assert_eq!(detect_delimiter(&[comma_csv.to_string()]).unwrap(), ',');
        assert_eq!(detect_delimiter(&[semi_csv.to_string()]).unwrap(), ';');
    }

    #[test]
    fn test_quoted_values() {
        let values = split_csv_line("\"Hello, World\",123,\"Test\"", ',');
        assert_eq!(values, vec!["Hello, World", "123", "Test"]);
    }
}
