"""DataForge-Parser: PDF document parser for End-of-Well reports.

This package provides tools for extracting structured data from End-of-Well (EOW)
PDF reports and mapping them to OSDU-aligned schemas compatible with DataForge.

Example usage:
    from dataforge_parser import parse_eow_report

    result = parse_eow_report("report.pdf")
    print(result["well"])
    print(result["markers"])
    print(result["trajectory"])

CLI usage:
    dataforge-parser parse report.pdf -o output.json
    dataforge-parser validate output.json
    dataforge-parser info report.pdf
"""

__version__ = "0.1.0"

from dataforge_parser.parsers.docling_parser import (
    DoclingParser,
    ParsedDocument,
    DocumentPage,
    create_eow_parser,
)
from dataforge_parser.pipeline import (
    ParsingPipeline,
    PipelineResult,
    parse_eow_report,
)
from dataforge_parser.extractors.well_header import WellHeaderExtractor
from dataforge_parser.extractors.formations import FormationsExtractor
from dataforge_parser.extractors.surveys import SurveysExtractor
from dataforge_parser.mappers.osdu_mapper import OsduMapper
from dataforge_parser.validators.schema_validator import SchemaValidator

__all__ = [
    # Version
    "__version__",
    # Main API
    "parse_eow_report",
    "ParsingPipeline",
    "PipelineResult",
    # Parser
    "DoclingParser",
    "ParsedDocument",
    "DocumentPage",
    "create_eow_parser",
    # Extractors
    "WellHeaderExtractor",
    "FormationsExtractor",
    "SurveysExtractor",
    # Mapper & Validator
    "OsduMapper",
    "SchemaValidator",
]
