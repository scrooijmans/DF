//! Surface CSV Parser
//!
//! Parses CSV files containing 3D surface point data.
//! Supports multiple delimiters, multi-surface files, and automatic column detection.

use crate::models::{SpatialExtent, SurfaceColumnType};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

use super::detection::{detect_surface_column_with_unit, DetectedSurfaceColumn};

/// A single parsed surface point
#[derive(Debug, Clone)]
pub struct ParsedSurfacePoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Surface name (for multi-surface files)
    pub surface_name: Option<String>,
    /// Additional attributes as key-value pairs
    pub attributes: HashMap<String, String>,
    /// Original row index in the CSV
    pub row_index: usize,
}

/// A group of points belonging to a single surface
#[derive(Debug, Clone)]
pub struct SurfaceGroup {
    /// Surface name (or None for single-surface files)
    pub name: Option<String>,
    /// Points in this group
    pub points: Vec<ParsedSurfacePoint>,
    /// Spatial extent of this group
    pub extent: SpatialExtent,
}

/// Result of parsing a surface CSV file
#[derive(Debug, Clone)]
pub struct ParsedSurfaceFile {
    /// Unique ID for this parse operation
    pub file_id: String,
    /// Detected columns with types
    pub columns: Vec<DetectedSurfaceColumn>,
    /// Total row count (excluding header)
    pub row_count: usize,
    /// Detected delimiter character
    pub delimiter: char,
    /// Whether this file contains multiple surfaces
    pub is_multi_surface: bool,
    /// List of surface names (if multi-surface)
    pub surface_names: Vec<String>,
    /// Surface groups (points grouped by surface name)
    pub groups: Vec<SurfaceGroup>,
    /// Overall spatial extent
    pub extent: SpatialExtent,
    /// Whether required columns (X, Y, Z) were found
    pub has_required_columns: bool,
    /// List of missing required columns
    pub missing_required: Vec<String>,
}

/// Detect delimiter from a sample of the file
pub fn detect_delimiter(sample: &str) -> char {
    let delimiters = [',', '\t', ';', ' '];
    let mut counts: HashMap<char, usize> = HashMap::new();

    for line in sample.lines().take(10) {
        for delim in &delimiters {
            let count = line.matches(*delim).count();
            if count > 0 {
                *counts.entry(*delim).or_insert(0) += count;
            }
        }
    }

    // Find the delimiter with the most consistent count
    let mut best_delim = ',';
    let mut best_count = 0;

    for (delim, count) in counts {
        // Prefer comma if counts are close
        if count > best_count || (count == best_count && delim == ',') {
            best_count = count;
            best_delim = delim;
        }
    }

    best_delim
}

/// Check if the first line is likely a header (contains text, not just numbers)
fn is_header_row(line: &str, delimiter: char) -> bool {
    let parts: Vec<&str> = line.split(delimiter).collect();
    if parts.is_empty() {
        return false;
    }

    // If more than half the fields are non-numeric, it's probably a header
    let non_numeric_count = parts
        .iter()
        .filter(|p| {
            let trimmed = p.trim();
            !trimmed.is_empty() && trimmed.parse::<f64>().is_err()
        })
        .count();

    non_numeric_count > parts.len() / 2
}

/// Parse a surface CSV file
///
/// # Arguments
/// * `reader` - Reader for the CSV data
/// * `file_id` - Unique identifier for this parse operation
///
/// # Returns
/// * `ParsedSurfaceFile` containing all parsed data and metadata
pub fn parse_surface_csv<R: Read>(reader: R, file_id: String) -> Result<ParsedSurfaceFile, String> {
    let mut buf_reader = BufReader::new(reader);

    // Read entire file into memory for processing
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Detect delimiter
    let delimiter = detect_delimiter(&content);

    // Split into lines
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err("Empty file".to_string());
    }

    // Check for header row
    let has_header = is_header_row(lines[0], delimiter);
    let (headers, data_start) = if has_header {
        let headers: Vec<String> = lines[0]
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .collect();
        (headers, 1)
    } else {
        // Generate default headers
        let first_line_parts: Vec<&str> = lines[0].split(delimiter).collect();
        let headers: Vec<String> = (0..first_line_parts.len())
            .map(|i| format!("Column{}", i + 1))
            .collect();
        (headers, 0)
    };

    // Detect column types
    let columns: Vec<DetectedSurfaceColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| detect_surface_column_with_unit(i, h))
        .collect();

    // Find required column indices
    let x_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::X)
        .map(|c| c.index);
    let y_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::Y)
        .map(|c| c.index);
    let z_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::Z)
        .map(|c| c.index);
    let name_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::SurfaceName)
        .map(|c| c.index);

    // Check for required columns
    let mut missing_required = Vec::new();
    if x_idx.is_none() {
        missing_required.push("X".to_string());
    }
    if y_idx.is_none() {
        missing_required.push("Y".to_string());
    }
    if z_idx.is_none() {
        missing_required.push("Z".to_string());
    }

    let has_required_columns = missing_required.is_empty();

    // Parse data rows
    let mut points: Vec<ParsedSurfacePoint> = Vec::new();
    let mut overall_extent = SpatialExtent::empty();
    let mut surface_names_set: HashMap<String, bool> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate().skip(data_start) {
        let parts: Vec<&str> = line.split(delimiter).map(|s| s.trim()).collect();

        // Skip empty lines
        if parts.is_empty() || parts.iter().all(|p| p.is_empty()) {
            continue;
        }

        // Parse X, Y, Z if columns are found
        let x = x_idx
            .and_then(|i| parts.get(i))
            .and_then(|v| v.parse::<f64>().ok());
        let y = y_idx
            .and_then(|i| parts.get(i))
            .and_then(|v| v.parse::<f64>().ok());
        let z = z_idx
            .and_then(|i| parts.get(i))
            .and_then(|v| v.parse::<f64>().ok());

        // Get surface name if present
        let surface_name = name_idx
            .and_then(|i| parts.get(i))
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty());

        if let Some(ref name) = surface_name {
            surface_names_set.insert(name.clone(), true);
        }

        // Only include points with valid X, Y, Z
        if let (Some(x_val), Some(y_val), Some(z_val)) = (x, y, z) {
            // Collect additional attributes
            let mut attributes = HashMap::new();
            for (i, part) in parts.iter().enumerate() {
                if i != x_idx.unwrap_or(usize::MAX)
                    && i != y_idx.unwrap_or(usize::MAX)
                    && i != z_idx.unwrap_or(usize::MAX)
                    && i != name_idx.unwrap_or(usize::MAX)
                {
                    if let Some(col) = columns.get(i) {
                        if !part.is_empty() {
                            attributes.insert(col.original_name.clone(), part.to_string());
                        }
                    }
                }
            }

            overall_extent.expand(x_val, y_val, z_val);

            points.push(ParsedSurfacePoint {
                x: x_val,
                y: y_val,
                z: z_val,
                surface_name: surface_name.clone(),
                attributes,
                row_index: line_idx,
            });
        }
    }

    // Group points by surface name
    let surface_names: Vec<String> = surface_names_set.keys().cloned().collect();
    let is_multi_surface = surface_names.len() > 1;

    let groups = if is_multi_surface || !surface_names.is_empty() {
        // Group by surface name
        let mut groups_map: HashMap<Option<String>, Vec<ParsedSurfacePoint>> = HashMap::new();

        for point in points.clone() {
            groups_map
                .entry(point.surface_name.clone())
                .or_default()
                .push(point);
        }

        groups_map
            .into_iter()
            .map(|(name, group_points)| {
                let mut extent = SpatialExtent::empty();
                for p in &group_points {
                    extent.expand(p.x, p.y, p.z);
                }

                SurfaceGroup {
                    name,
                    points: group_points,
                    extent,
                }
            })
            .collect()
    } else {
        // Single surface (no name column or all same name)
        vec![SurfaceGroup {
            name: None,
            points: points.clone(),
            extent: overall_extent,
        }]
    };

    let row_count = lines.len() - data_start;

    Ok(ParsedSurfaceFile {
        file_id,
        columns,
        row_count,
        delimiter,
        is_multi_surface,
        surface_names,
        groups,
        extent: overall_extent,
        has_required_columns,
        missing_required,
    })
}

/// Parse only the header and detect columns without reading all data
///
/// Useful for preview before full ingestion
pub fn parse_surface_header<R: Read>(
    reader: R,
    file_id: String,
) -> Result<ParsedSurfaceFile, String> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    // Read first few lines
    let first_line = lines
        .next()
        .ok_or("Empty file")?
        .map_err(|e| format!("Failed to read first line: {}", e))?;

    // Detect delimiter from first line
    let delimiter = detect_delimiter(&first_line);

    // Check for header
    let has_header = is_header_row(&first_line, delimiter);

    let headers: Vec<String> = if has_header {
        first_line
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let parts: Vec<&str> = first_line.split(delimiter).collect();
        (0..parts.len())
            .map(|i| format!("Column{}", i + 1))
            .collect()
    };

    // Detect column types
    let columns: Vec<DetectedSurfaceColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| detect_surface_column_with_unit(i, h))
        .collect();

    // Check for required columns
    let mut missing_required = Vec::new();
    if !columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::X)
    {
        missing_required.push("X".to_string());
    }
    if !columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::Y)
    {
        missing_required.push("Y".to_string());
    }
    if !columns
        .iter()
        .any(|c| c.column_type == SurfaceColumnType::Z)
    {
        missing_required.push("Z".to_string());
    }

    let has_required_columns = missing_required.is_empty();

    // Count rows and detect surface names from a sample
    let mut row_count = if has_header { 0 } else { 1 };
    let mut surface_names_set: HashMap<String, bool> = HashMap::new();
    let mut extent = SpatialExtent::empty();

    let name_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::SurfaceName)
        .map(|c| c.index);
    let x_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::X)
        .map(|c| c.index);
    let y_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::Y)
        .map(|c| c.index);
    let z_idx = columns
        .iter()
        .find(|c| c.column_type == SurfaceColumnType::Z)
        .map(|c| c.index);

    for line_result in lines {
        if let Ok(line) = line_result {
            if !line.trim().is_empty() {
                row_count += 1;
                let parts: Vec<&str> = line.split(delimiter).map(|s| s.trim()).collect();

                // Extract surface name
                if let Some(idx) = name_idx {
                    if let Some(name) = parts.get(idx) {
                        if !name.is_empty() {
                            surface_names_set.insert(name.to_string(), true);
                        }
                    }
                }

                // Update extent
                let x = x_idx
                    .and_then(|i| parts.get(i))
                    .and_then(|v| v.parse::<f64>().ok());
                let y = y_idx
                    .and_then(|i| parts.get(i))
                    .and_then(|v| v.parse::<f64>().ok());
                let z = z_idx
                    .and_then(|i| parts.get(i))
                    .and_then(|v| v.parse::<f64>().ok());

                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    extent.expand(x, y, z);
                }
            }
        }
    }

    let surface_names: Vec<String> = surface_names_set.keys().cloned().collect();
    let is_multi_surface = surface_names.len() > 1;

    Ok(ParsedSurfaceFile {
        file_id,
        columns,
        row_count,
        delimiter,
        is_multi_surface,
        surface_names,
        groups: Vec::new(), // No groups for header-only parse
        extent,
        has_required_columns,
        missing_required,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_detect_delimiter_comma() {
        let sample = "X,Y,Z\n1.0,2.0,3.0\n4.0,5.0,6.0";
        assert_eq!(detect_delimiter(sample), ',');
    }

    #[test]
    fn test_detect_delimiter_tab() {
        let sample = "X\tY\tZ\n1.0\t2.0\t3.0";
        assert_eq!(detect_delimiter(sample), '\t');
    }

    #[test]
    fn test_detect_delimiter_semicolon() {
        let sample = "X;Y;Z\n1.0;2.0;3.0";
        assert_eq!(detect_delimiter(sample), ';');
    }

    #[test]
    fn test_is_header_row() {
        assert!(is_header_row("X,Y,Z", ','));
        assert!(is_header_row("Easting,Northing,Depth", ','));
        assert!(!is_header_row("1.0,2.0,3.0", ','));
        assert!(!is_header_row("100,200,300", ','));
    }

    #[test]
    fn test_parse_simple_csv() {
        let csv = "X,Y,Z\n1.0,2.0,3.0\n4.0,5.0,6.0\n7.0,8.0,9.0";
        let cursor = Cursor::new(csv);
        let result = parse_surface_csv(cursor, "test".to_string()).unwrap();

        assert_eq!(result.row_count, 3);
        assert_eq!(result.delimiter, ',');
        assert!(result.has_required_columns);
        assert!(!result.is_multi_surface);
        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0].points.len(), 3);

        // Check extent
        assert!((result.extent.min_x - 1.0).abs() < 0.01);
        assert!((result.extent.max_x - 7.0).abs() < 0.01);
        assert!((result.extent.min_z - 3.0).abs() < 0.01);
        assert!((result.extent.max_z - 9.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_multi_surface_csv() {
        let csv = "Surface,X,Y,Z\nTop,1.0,2.0,3.0\nTop,4.0,5.0,6.0\nBase,1.0,2.0,10.0\nBase,4.0,5.0,11.0";
        let cursor = Cursor::new(csv);
        let result = parse_surface_csv(cursor, "test".to_string()).unwrap();

        assert!(result.is_multi_surface);
        assert_eq!(result.surface_names.len(), 2);
        assert!(result.surface_names.contains(&"Top".to_string()));
        assert!(result.surface_names.contains(&"Base".to_string()));
        assert_eq!(result.groups.len(), 2);
    }

    #[test]
    fn test_parse_no_header() {
        let csv = "1.0,2.0,3.0\n4.0,5.0,6.0";
        let cursor = Cursor::new(csv);
        let result = parse_surface_csv(cursor, "test".to_string()).unwrap();

        // Should generate Column1, Column2, Column3 headers
        assert_eq!(result.columns.len(), 3);
        assert!(result.columns[0].original_name.starts_with("Column"));
    }

    #[test]
    fn test_parse_missing_columns() {
        let csv = "Name,Value\nPoint1,100\nPoint2,200";
        let cursor = Cursor::new(csv);
        let result = parse_surface_csv(cursor, "test".to_string()).unwrap();

        assert!(!result.has_required_columns);
        assert!(result.missing_required.contains(&"X".to_string()));
        assert!(result.missing_required.contains(&"Y".to_string()));
        assert!(result.missing_required.contains(&"Z".to_string()));
    }

    #[test]
    fn test_parse_with_attributes() {
        let csv = "X,Y,Z,Quality,Note\n1.0,2.0,3.0,Good,Test point";
        let cursor = Cursor::new(csv);
        let result = parse_surface_csv(cursor, "test".to_string()).unwrap();

        assert_eq!(result.groups[0].points.len(), 1);
        let point = &result.groups[0].points[0];
        assert!(point.attributes.contains_key("Quality"));
        assert!(point.attributes.contains_key("Note"));
    }
}
