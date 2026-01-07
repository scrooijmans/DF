"""Survey / trajectory data extractor."""

import re
from dataclasses import dataclass, field
from typing import Any

from dataforge_parser.parsers.docling_parser import ParsedDocument


@dataclass
class SurveyStation:
    """A single survey station."""

    md: float
    inclination: float | None = None
    azimuth: float | None = None
    tvd: float | None = None
    ns: float | None = None
    ew: float | None = None
    dls: float | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "md": self.md,
            "inclination": self.inclination,
            "azimuth": self.azimuth,
            "tvd": self.tvd,
            "ns": self.ns,
            "ew": self.ew,
            "dls": self.dls,
        }


@dataclass
class SurveyData:
    """Extracted survey / trajectory data."""

    stations: list[SurveyStation] = field(default_factory=list)
    survey_type: str | None = None
    survey_company: str | None = None
    survey_date: str | None = None
    azimuth_reference: str = "true_north"
    md_unit: str = "ft"
    angle_unit: str = "deg"
    magnetic_declination: float | None = None
    grid_convergence: float | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "stations": [s.to_dict() for s in self.stations],
            "survey_type": self.survey_type,
            "survey_company": self.survey_company,
            "survey_date": self.survey_date,
            "azimuth_reference": self.azimuth_reference,
            "md_unit": self.md_unit,
            "angle_unit": self.angle_unit,
            "magnetic_declination": self.magnetic_declination,
            "grid_convergence": self.grid_convergence,
            "station_count": len(self.stations),
            "top_md": self.stations[0].md if self.stations else None,
            "bottom_md": self.stations[-1].md if self.stations else None,
        }


class SurveysExtractor:
    """Extracts survey / trajectory data from EOW reports.

    Looks for:
    - Directional survey tables
    - Survey metadata (type, company, date)
    - Coordinate reference information
    """

    # Keywords that indicate a survey section
    SECTION_KEYWORDS = [
        "directional survey",
        "wellbore trajectory",
        "survey data",
        "deviation survey",
        "well path",
        "mwd survey",
        "gyro survey",
    ]

    # Common column header patterns
    COLUMN_PATTERNS = {
        "md": ["md", "measured depth", "depth"],
        "inc": ["inc", "inclination", "incl", "dip"],
        "azi": ["azi", "azimuth", "az", "direction"],
        "tvd": ["tvd", "true vertical depth", "vertical depth"],
        "ns": ["ns", "n/s", "north", "northing", "+n/-s"],
        "ew": ["ew", "e/w", "east", "easting", "+e/-w"],
        "dls": ["dls", "dogleg", "dog leg", "severity"],
    }

    # Survey type patterns
    SURVEY_TYPE_PATTERNS = {
        "definitive": ["definitive", "final", "as-drilled"],
        "mwd": ["mwd", "measurement while drilling"],
        "gyro": ["gyro", "gyroscopic"],
        "preliminary": ["preliminary", "initial"],
    }

    def extract(self, document: ParsedDocument) -> dict[str, Any]:
        """Extract survey data from a parsed document.

        Args:
            document: Parsed PDF document.

        Returns:
            Dictionary with survey data and metadata.
        """
        survey = SurveyData()

        # Extract stations from tables
        for table in document.all_tables:
            stations = self._extract_stations_from_table(table)
            if stations:
                survey.stations.extend(stations)

        # Extract metadata from text
        self._extract_metadata(document, survey)

        return survey.to_dict()

    def _extract_stations_from_table(
        self, table: dict[str, Any]
    ) -> list[SurveyStation]:
        """Extract survey stations from a table.

        Args:
            table: Table dictionary with headers and rows.

        Returns:
            List of SurveyStation objects.
        """
        stations = []
        headers = [str(h).lower().strip() for h in table.get("headers", [])]
        rows = table.get("rows", [])

        if not headers or not rows:
            return stations

        # Find column indices
        md_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["md"])
        inc_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["inc"])
        azi_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["azi"])
        tvd_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["tvd"])
        ns_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["ns"])
        ew_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["ew"])
        dls_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["dls"])

        # Must have at least MD column
        if md_idx is None:
            return stations

        for row in rows:
            try:
                md = self._parse_number(row[md_idx])
                if md is None:
                    continue

                station = SurveyStation(
                    md=md,
                    inclination=self._parse_number(row[inc_idx]) if inc_idx else None,
                    azimuth=self._parse_number(row[azi_idx]) if azi_idx else None,
                    tvd=self._parse_number(row[tvd_idx]) if tvd_idx else None,
                    ns=self._parse_number(row[ns_idx]) if ns_idx else None,
                    ew=self._parse_number(row[ew_idx]) if ew_idx else None,
                    dls=self._parse_number(row[dls_idx]) if dls_idx else None,
                )
                stations.append(station)
            except (IndexError, ValueError):
                continue

        return stations

    def _extract_metadata(self, document: ParsedDocument, survey: SurveyData) -> None:
        """Extract survey metadata from document text.

        Args:
            document: Parsed document.
            survey: SurveyData to populate.
        """
        text = document.full_text.lower()

        # Detect survey type
        for survey_type, patterns in self.SURVEY_TYPE_PATTERNS.items():
            for pattern in patterns:
                if pattern in text:
                    survey.survey_type = survey_type
                    break
            if survey.survey_type:
                break

        # Extract magnetic declination
        dec_match = re.search(
            r"magnetic\s*declination\s*[:\-]?\s*([\-\+]?\d+\.?\d*)",
            text,
            re.IGNORECASE,
        )
        if dec_match:
            try:
                survey.magnetic_declination = float(dec_match.group(1))
            except ValueError:
                pass

        # Extract grid convergence
        conv_match = re.search(
            r"grid\s*convergence\s*[:\-]?\s*([\-\+]?\d+\.?\d*)",
            text,
            re.IGNORECASE,
        )
        if conv_match:
            try:
                survey.grid_convergence = float(conv_match.group(1))
            except ValueError:
                pass

        # Detect azimuth reference
        if "true north" in text:
            survey.azimuth_reference = "true_north"
        elif "grid north" in text:
            survey.azimuth_reference = "grid_north"
        elif "magnetic north" in text:
            survey.azimuth_reference = "magnetic_north"

        # Extract survey company
        company_match = re.search(
            r"(?:survey|logging)\s*(?:company|contractor)\s*[:\-]?\s*(.+?)(?:\n|$)",
            text,
            re.IGNORECASE,
        )
        if company_match:
            survey.survey_company = company_match.group(1).strip()

    def _find_column_index(
        self, headers: list[str], patterns: list[str]
    ) -> int | None:
        """Find the index of a column matching any of the patterns.

        Args:
            headers: List of column headers.
            patterns: List of patterns to match.

        Returns:
            Column index or None if not found.
        """
        for i, header in enumerate(headers):
            for pattern in patterns:
                if pattern in header:
                    return i
        return None

    def _parse_number(self, value: Any) -> float | None:
        """Parse a numeric value.

        Args:
            value: Value to parse.

        Returns:
            Float or None if parsing fails.
        """
        if value is None:
            return None
        try:
            return float(str(value).replace(",", "").strip())
        except ValueError:
            return None
