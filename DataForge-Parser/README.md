# DataForge-Parser

PDF document parser for End-of-Well (EOW) reports with OSDU schema mapping.

## Overview

DataForge-Parser extracts structured data from unstructured PDF well reports and maps it to OSDU-aligned schemas compatible with the DataForge SQLite database.

## Features

- **PDF Parsing**: Uses [Docling](https://github.com/docling-project/docling) for high-accuracy table and text extraction
- **Section Detection**: Automatically identifies common EOW report sections
- **OSDU Mapping**: Maps extracted data to OSDU work product component schemas
- **Schema Validation**: Validates output against JSON schemas before storage
- **CLI Interface**: Simple command-line tool for batch processing

## Installation

```bash
# Basic installation
pip install -e .

# With OCR support (for scanned documents)
pip install -e ".[ocr]"

# With LLM support (for semantic field mapping)
pip install -e ".[llm]"

# Development dependencies
pip install -e ".[dev]"
```

## Usage

### Command Line

```bash
# Parse a single PDF
dataforge-parser parse report.pdf --output output.json

# Parse with specific extractors
dataforge-parser parse report.pdf --extract well-header --extract formations

# Validate against OSDU schema
dataforge-parser validate output.json --schema wellbore-marker-set
```

### Python API

```python
from dataforge_parser import DoclingParser
from dataforge_parser.extractors import WellHeaderExtractor, FormationsExtractor
from dataforge_parser.mappers import OsduMapper

# Parse PDF
parser = DoclingParser()
document = parser.parse("report.pdf")

# Extract sections
well_header = WellHeaderExtractor().extract(document)
formations = FormationsExtractor().extract(document)

# Map to OSDU schema
mapper = OsduMapper()
well_record = mapper.map_well(well_header)
marker_set = mapper.map_marker_set(formations)
```

## Project Structure

```
DataForge-Parser/
├── src/dataforge_parser/
│   ├── __init__.py
│   ├── cli.py              # Click-based CLI
│   ├── parsers/            # PDF parsing backends
│   │   ├── docling_parser.py
│   │   └── unstructured_parser.py
│   ├── extractors/         # Section-specific extractors
│   │   ├── well_header.py
│   │   ├── casing.py
│   │   ├── formations.py
│   │   └── surveys.py
│   ├── mappers/            # OSDU schema mappers
│   │   └── osdu_mapper.py
│   ├── validators/         # JSON schema validation
│   │   └── schema_validator.py
│   └── storage/            # Output writers
│       └── sqlite_writer.py
├── schemas/                # OSDU JSON schemas (reference)
├── templates/              # Section detection templates
└── tests/
```

## Supported EOW Report Sections

| Section | OSDU Kind | Status |
|---------|-----------|--------|
| Well Header | `master-data--Well` | MVP |
| Formation Tops | `work-product-component--WellboreMarkerSet` | MVP |
| Survey Data | `work-product-component--WellboreTrajectory` | MVP |
| Casing Program | `work-product-component--WellboreCompletion` | Planned |
| Cement Jobs | `work-product-component--CementJob` | Planned |
| Mud Properties | `work-product-component--MudLog` | Planned |
| DST/Tests | `work-product-component--WellTest` | Planned |

## Integration with DataForge

DataForge-Parser outputs JSON that matches DataForge's ingest types. Integration options:

1. **CLI**: Invoke via Tauri `Command::new("dataforge-parser")`
2. **HTTP**: Start as FastAPI sidecar service
3. **Direct**: Import as Python module in a sidecar process

## License

MIT
