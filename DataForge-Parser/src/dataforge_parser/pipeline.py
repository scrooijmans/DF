"""End-of-Well report parsing pipeline orchestrator."""

from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Any

from dataforge_parser.parsers.docling_parser import ParsedDocument, create_eow_parser
from dataforge_parser.extractors.well_header import WellHeaderExtractor
from dataforge_parser.extractors.formations import FormationsExtractor
from dataforge_parser.extractors.surveys import SurveysExtractor
from dataforge_parser.mappers.osdu_mapper import OsduMapper
from dataforge_parser.validators.schema_validator import SchemaValidator


@dataclass
class PipelineResult:
    """Result from the parsing pipeline."""

    source_file: str
    parsed_at: str
    well: dict[str, Any] | None = None
    markers: dict[str, Any] | None = None
    trajectory: dict[str, Any] | None = None
    validation_errors: dict[str, list[str]] = field(default_factory=dict)
    metadata: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        result: dict[str, Any] = {
            "source": "dataforge-parser",
            "version": "1.0.0",
            "source_file": self.source_file,
            "parsed_at": self.parsed_at,
        }

        if self.well:
            result["well"] = self.well

        if self.markers:
            result["markers"] = self.markers

        if self.trajectory:
            result["trajectory"] = self.trajectory

        if self.validation_errors:
            result["validation_errors"] = self.validation_errors

        if self.metadata:
            result["metadata"] = self.metadata

        return result

    @property
    def is_valid(self) -> bool:
        """Check if all extracted data passed validation."""
        return len(self.validation_errors) == 0


class ParsingPipeline:
    """Orchestrates the End-of-Well report parsing pipeline.

    The pipeline:
    1. Parses PDF using Docling (high-accuracy table extraction)
    2. Extracts well header, formations, and survey data
    3. Maps extracted data to OSDU-aligned schemas
    4. Validates against schema definitions
    """

    def __init__(self, use_ocr: bool = False, validate: bool = True) -> None:
        """Initialize the parsing pipeline.

        Args:
            use_ocr: Enable OCR for scanned documents.
            validate: Whether to validate extracted data against schemas.
        """
        self.use_ocr = use_ocr
        self.validate = validate

        # Initialize components (lazy-loaded)
        self._parser = None
        self._well_extractor = WellHeaderExtractor()
        self._formation_extractor = FormationsExtractor()
        self._survey_extractor = SurveysExtractor()
        self._mapper = OsduMapper()
        self._validator = SchemaValidator()

    @property
    def parser(self):
        """Lazy-load the Docling parser."""
        if self._parser is None:
            self._parser = create_eow_parser(use_ocr=self.use_ocr)
        return self._parser

    def process(self, pdf_path: str | Path) -> PipelineResult:
        """Process an End-of-Well report PDF.

        Args:
            pdf_path: Path to the PDF file.

        Returns:
            PipelineResult with extracted and mapped data.
        """
        pdf_path = Path(pdf_path)

        # Initialize result
        result = PipelineResult(
            source_file=str(pdf_path),
            parsed_at=datetime.utcnow().isoformat() + "Z",
        )

        # Step 1: Parse PDF
        try:
            doc = self.parser.parse(pdf_path)
            result.metadata["table_count"] = doc.metadata.get("table_count", 0)
            result.metadata["ocr_enabled"] = doc.metadata.get("ocr_enabled", False)
        except Exception as e:
            result.validation_errors["parsing"] = [f"Failed to parse PDF: {e!s}"]
            return result

        # Step 2: Extract data
        well_data = self._extract_well_header(doc, result)
        formations = self._extract_formations(doc, result)
        surveys = self._extract_surveys(doc, result)

        # Step 3: Map to OSDU schemas
        if well_data:
            result.well = self._mapper.map_well(well_data)

        if formations:
            result.markers = self._mapper.map_marker_set(formations)

        if surveys:
            result.trajectory = self._mapper.map_survey_run(surveys)

        # Step 4: Validate (optional)
        if self.validate:
            self._validate_results(result)

        return result

    def _extract_well_header(
        self, doc: ParsedDocument, result: PipelineResult
    ) -> dict[str, Any] | None:
        """Extract well header data."""
        try:
            data = self._well_extractor.extract(doc)
            if data and data.get("name"):
                return data
        except Exception as e:
            result.validation_errors.setdefault("well_header", []).append(
                f"Extraction failed: {e!s}"
            )
        return None

    def _extract_formations(
        self, doc: ParsedDocument, result: PipelineResult
    ) -> list[dict[str, Any]] | None:
        """Extract formation tops data."""
        try:
            data = self._formation_extractor.extract(doc)
            if data:
                result.metadata["formation_count"] = len(data)
                return data
        except Exception as e:
            result.validation_errors.setdefault("formations", []).append(
                f"Extraction failed: {e!s}"
            )
        return None

    def _extract_surveys(
        self, doc: ParsedDocument, result: PipelineResult
    ) -> dict[str, Any] | None:
        """Extract directional survey data."""
        try:
            data = self._survey_extractor.extract(doc)
            if data and data.get("station_count", 0) > 0:
                result.metadata["station_count"] = data.get("station_count", 0)
                return data
        except Exception as e:
            result.validation_errors.setdefault("surveys", []).append(
                f"Extraction failed: {e!s}"
            )
        return None

    def _validate_results(self, result: PipelineResult) -> None:
        """Validate extracted data against OSDU schemas."""
        if result.well:
            errors = self._validator.validate(result.well, "well")
            if errors:
                result.validation_errors["well"] = errors

        if result.markers:
            errors = self._validator.validate(result.markers, "wellbore-marker-set")
            if errors:
                result.validation_errors["markers"] = errors

        if result.trajectory:
            errors = self._validator.validate(result.trajectory, "wellbore-trajectory")
            if errors:
                result.validation_errors["trajectory"] = errors


def parse_eow_report(
    pdf_path: str | Path,
    use_ocr: bool = False,
    validate: bool = True,
) -> dict[str, Any]:
    """Convenience function to parse an End-of-Well report.

    Args:
        pdf_path: Path to the PDF file.
        use_ocr: Enable OCR for scanned documents.
        validate: Whether to validate extracted data.

    Returns:
        Dictionary with extracted data in DataForge-compatible format.
    """
    pipeline = ParsingPipeline(use_ocr=use_ocr, validate=validate)
    result = pipeline.process(pdf_path)
    return result.to_dict()
