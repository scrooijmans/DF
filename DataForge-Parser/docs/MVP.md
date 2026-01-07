# DataForge-Parser MVP

## Overview

DataForge-Parser is a Python tool for extracting structured data from End-of-Well (EOW) PDF reports and mapping them to OSDU-aligned schemas compatible with DataForge's SQLite database.

## MVP Scope

### Data Types Extracted

| Data Type | Source Section | Target Table | Status |
|-----------|----------------|--------------|--------|
| Well Header | "Well Information" section | `wells` | Implemented |
| Formation Tops | "Formation Tops" table | `marker_sets` + `markers` | Implemented |
| Survey Data | "Directional Survey" table | `survey_runs` + `trajectory_columns` | Implemented |
| Casing Program | "Casing" table | `casing_strings` | Planned |
| Cement Jobs | "Cement" section | `cement_jobs` | Planned |

### Key Features

- **High-accuracy table extraction** using IBM Docling (97.9% accuracy)
- **OSDU-aligned output** matching DataForge's existing schema
- **Schema validation** before output
- **CLI interface** for standalone use
- **Python API** for programmatic integration

## Installation

```bash
cd DataForge-Parser
pip install -e .

# With development dependencies
pip install -e ".[dev]"

# With OCR support for scanned documents
pip install -e ".[ocr]"
```

## Usage

### CLI

```bash
# Parse a PDF and output JSON
dataforge-parser parse report.pdf -o output.json

# Parse with OCR enabled (for scanned documents)
dataforge-parser parse report.pdf --ocr -o output.json

# Parse with verbose output
dataforge-parser parse report.pdf -v -o output.json

# Output to stdout (for piping)
dataforge-parser parse report.pdf -o -

# Validate existing JSON
dataforge-parser validate output.json

# Quick document info
dataforge-parser info report.pdf

# List available extractors
dataforge-parser list
```

### Python API

```python
from dataforge_parser import parse_eow_report

# Simple usage
result = parse_eow_report("report.pdf")
print(result["well"])       # Well header data
print(result["markers"])    # Formation tops
print(result["trajectory"]) # Survey data

# With options
result = parse_eow_report(
    "report.pdf",
    use_ocr=True,      # Enable OCR for scanned documents
    validate=True,     # Validate against OSDU schemas
)

# Check for validation errors
if "validation_errors" in result:
    print("Validation issues:", result["validation_errors"])
```

### Pipeline API

```python
from dataforge_parser import ParsingPipeline

pipeline = ParsingPipeline(use_ocr=False, validate=True)
result = pipeline.process("report.pdf")

# Access structured result
print(result.well)          # Well header dict
print(result.markers)       # Marker set dict
print(result.trajectory)    # Trajectory dict
print(result.is_valid)      # True if no validation errors

# Convert to JSON-serializable dict
output = result.to_dict()
```

## Output Format

The parser outputs JSON in a format compatible with DataForge's ingest workflow:

```json
{
  "source": "dataforge-parser",
  "version": "1.0.0",
  "source_file": "report.pdf",
  "parsed_at": "2024-01-15T10:30:00Z",

  "well": {
    "name": "Test Well #1",
    "uwi": "12345678901234",
    "operator": "Test Oil Company",
    "field": "Test Field",
    "county": "Test County",
    "state_province": "Texas",
    "country": "USA",
    "spud_date": "2024-01-15",
    "depth_unit": "ft",
    "max_depth": 10500.0
  },

  "markers": {
    "set_name": "Formation Tops",
    "interpretation_type": "formation",
    "depth_unit": "ft",
    "marker_count": 2,
    "markers": [
      {
        "name": "Barnett Shale",
        "measured_depth": 8500.0,
        "true_vertical_depth": 8450.0,
        "marker_type": "formation"
      }
    ]
  },

  "trajectory": {
    "survey_type": "definitive",
    "azimuth_reference": "true_north",
    "magnetic_declination": 3.5,
    "grid_convergence": -0.8,
    "md_unit": "ft",
    "station_count": 3,
    "top_md": 0.0,
    "bottom_md": 2000.0,
    "columns": [...]
  }
}
```

## Architecture

```
DataForge-Parser/
├── src/dataforge_parser/
│   ├── __init__.py          # Public API exports
│   ├── cli.py               # Click-based CLI
│   ├── pipeline.py          # Pipeline orchestrator
│   ├── parsers/
│   │   └── docling_parser.py  # Docling PDF parser
│   ├── extractors/
│   │   ├── well_header.py   # Well header extraction
│   │   ├── formations.py    # Formation tops extraction
│   │   └── surveys.py       # Survey data extraction
│   ├── mappers/
│   │   └── osdu_mapper.py   # OSDU schema mapping
│   ├── validators/
│   │   └── schema_validator.py  # Schema validation
│   └── storage/
│       └── sqlite_writer.py # DataForge integration
└── tests/
    └── test_extractors.py   # Unit tests
```

### Pipeline Flow

```
PDF Document
    │
    ▼
┌─────────────────┐
│  Docling Parser │  (97.9% table accuracy)
│  - OCR support  │
│  - Table detect │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Extractors    │
│  - Well Header  │
│  - Formations   │
│  - Surveys      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  OSDU Mapper    │  (Schema alignment)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Validator     │  (Schema validation)
└────────┬────────┘
         │
         ▼
    JSON Output
```

## Integration with DataForge

### Option A: CLI (Current)

The DataForge Tauri backend can invoke the CLI:

```rust
#[tauri::command]
async fn parse_pdf(path: String) -> Result<Value, String> {
    let output = Command::new("dataforge-parser")
        .args(["parse", &path, "-o", "-"])
        .output()
        .map_err(|e| e.to_string())?;

    serde_json::from_slice(&output.stdout)
        .map_err(|e| e.to_string())
}
```

### Option B: Python Sidecar (Future)

For better performance with large files, a FastAPI server could be started as a Tauri sidecar process.

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| docling | >=2.0.0 | PDF parsing with high-accuracy tables |
| click | >=8.0.0 | CLI framework |
| pydantic | >=2.0.0 | Data validation |
| jsonschema | >=4.0.0 | Schema validation |
| rich | >=13.0.0 | Pretty terminal output |

### Optional

| Package | Version | Purpose |
|---------|---------|---------|
| unstructured | >=0.10.0 | Fallback parser |
| pytesseract | >=0.3.0 | OCR support |

## Testing

```bash
# Run tests
pytest

# Run with coverage
pytest --cov=dataforge_parser

# Run specific test
pytest tests/test_extractors.py -v
```

## Limitations

1. **Table Format Variability**: Different operators use different table formats. The extractors handle common patterns but may need tuning for specific formats.

2. **Scanned Documents**: OCR accuracy depends on scan quality. High-resolution scans work best.

3. **Complex Layouts**: Multi-column layouts and embedded images may affect extraction accuracy.

4. **Language**: Currently optimized for English-language reports.

## Future Enhancements

1. **Additional Extractors**: Casing, cement, mud logs, well tests
2. **LLM Integration**: Use Claude or local LLM for semantic field mapping
3. **Batch Processing**: Process multiple PDFs in parallel
4. **Custom Templates**: User-defined extraction patterns
5. **Direct SQLite Writing**: Write directly to DataForge database

## References

- [Docling Documentation](https://docling-project.github.io/docling/)
- [OSDU Data Definitions](https://community.opengroup.org/osdu/data/data-definitions)
- [DataForge Database Schema](../DataForge/crates/dataforge-core/src/db.rs)
