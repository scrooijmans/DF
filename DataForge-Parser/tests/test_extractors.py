"""Tests for data extractors."""

import pytest
from dataforge_parser.parsers.docling_parser import ParsedDocument, DocumentPage
from dataforge_parser.extractors.well_header import WellHeaderExtractor
from dataforge_parser.extractors.formations import FormationsExtractor
from dataforge_parser.extractors.surveys import SurveysExtractor


@pytest.fixture
def sample_document() -> ParsedDocument:
    """Create a sample document for testing."""
    text = """
    Well Information

    Well Name: Test Well #1
    UWI: 12345678901234
    API Number: 42-123-45678
    Operator: Test Oil Company
    Field: Test Field
    County: Test County
    State: Texas
    Country: USA
    Spud Date: 01/15/2024
    Total Depth: 10,500 ft
    KB Elevation: 1,234 ft

    Formation Tops

    The following formation tops were encountered:
    - Barnett Shale @ 8,500 ft
    - Ellenburger @ 10,200 ft

    Directional Survey

    Survey Type: Definitive
    Magnetic Declination: 3.5
    Grid Convergence: -0.8
    Azimuth Reference: True North
    """

    tables = [
        {
            "headers": ["Formation", "MD (ft)", "TVD (ft)", "TVDSS (ft)"],
            "rows": [
                ["Barnett Shale", "8500", "8450", "-7216"],
                ["Ellenburger", "10200", "10100", "-8866"],
            ],
        },
        {
            "headers": ["MD", "Inc", "Azi", "TVD", "NS", "EW", "DLS"],
            "rows": [
                ["0", "0", "0", "0", "0", "0", "0"],
                ["1000", "2.5", "45", "999.9", "17.7", "17.7", "2.5"],
                ["2000", "5.0", "45", "1998.5", "71.0", "71.0", "2.5"],
            ],
        },
    ]

    page = DocumentPage(
        page_number=1,
        text=text,
        tables=tables,
        sections=[],
    )

    return ParsedDocument(
        source_path="test.pdf",
        pages=[page],
        metadata={},
    )


class TestWellHeaderExtractor:
    """Tests for WellHeaderExtractor."""

    def test_extract_basic_fields(self, sample_document: ParsedDocument) -> None:
        """Test extraction of basic well header fields."""
        extractor = WellHeaderExtractor()
        result = extractor.extract(sample_document)

        assert result["name"] == "test well #1"
        assert result["operator"] == "test oil company"
        assert result["field"] == "test field"
        assert result["county"] == "test county"
        assert result["state"] == "texas"
        assert result["country"] == "usa"

    def test_extract_numeric_fields(self, sample_document: ParsedDocument) -> None:
        """Test extraction of numeric fields."""
        extractor = WellHeaderExtractor()
        result = extractor.extract(sample_document)

        assert result["total_depth"] == 10500.0
        assert result["kb_elevation"] == 1234.0


class TestFormationsExtractor:
    """Tests for FormationsExtractor."""

    def test_extract_from_table(self, sample_document: ParsedDocument) -> None:
        """Test extraction of formations from table."""
        extractor = FormationsExtractor()
        result = extractor.extract(sample_document)

        assert len(result) == 2

        barnett = result[0]
        assert barnett["name"] == "Barnett Shale"
        assert barnett["measured_depth"] == 8500.0
        assert barnett["true_vertical_depth"] == 8450.0

        ellenburger = result[1]
        assert ellenburger["name"] == "Ellenburger"
        assert ellenburger["measured_depth"] == 10200.0


class TestSurveysExtractor:
    """Tests for SurveysExtractor."""

    def test_extract_stations(self, sample_document: ParsedDocument) -> None:
        """Test extraction of survey stations."""
        extractor = SurveysExtractor()
        result = extractor.extract(sample_document)

        assert result["station_count"] == 3
        assert result["survey_type"] == "definitive"
        assert result["azimuth_reference"] == "true_north"
        assert result["magnetic_declination"] == 3.5
        assert result["grid_convergence"] == -0.8

    def test_extract_depth_range(self, sample_document: ParsedDocument) -> None:
        """Test extraction of depth range."""
        extractor = SurveysExtractor()
        result = extractor.extract(sample_document)

        assert result["top_md"] == 0.0
        assert result["bottom_md"] == 2000.0
