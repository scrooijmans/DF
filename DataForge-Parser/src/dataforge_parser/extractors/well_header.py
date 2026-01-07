"""Well header information extractor."""

import re
from dataclasses import dataclass
from typing import Any

from dataforge_parser.parsers.docling_parser import ParsedDocument


@dataclass
class WellHeader:
    """Extracted well header information."""

    name: str | None = None
    uwi: str | None = None
    api_number: str | None = None
    operator: str | None = None
    field: str | None = None
    county: str | None = None
    state: str | None = None
    country: str | None = None
    spud_date: str | None = None
    completion_date: str | None = None
    surface_latitude: float | None = None
    surface_longitude: float | None = None
    surface_x: float | None = None
    surface_y: float | None = None
    surface_crs: str | None = None
    kb_elevation: float | None = None
    ground_elevation: float | None = None
    total_depth: float | None = None
    depth_unit: str = "ft"

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "name": self.name,
            "uwi": self.uwi,
            "api_number": self.api_number,
            "operator": self.operator,
            "field": self.field,
            "county": self.county,
            "state": self.state,
            "country": self.country,
            "spud_date": self.spud_date,
            "completion_date": self.completion_date,
            "surface_latitude": self.surface_latitude,
            "surface_longitude": self.surface_longitude,
            "surface_x": self.surface_x,
            "surface_y": self.surface_y,
            "surface_crs": self.surface_crs,
            "kb_elevation": self.kb_elevation,
            "ground_elevation": self.ground_elevation,
            "total_depth": self.total_depth,
            "depth_unit": self.depth_unit,
        }


class WellHeaderExtractor:
    """Extracts well header information from EOW reports.

    Looks for common patterns in well header sections including:
    - Well name and identifiers (UWI, API)
    - Operator and field information
    - Location data (coordinates, county, state)
    - Key dates (spud, completion)
    - Elevation references (KB, ground level)
    """

    # Common header field patterns
    PATTERNS = {
        "name": [
            r"well\s*name\s*[:\-]?\s*(.+)",
            r"well\s*[:\-]?\s*(.+)",
        ],
        "uwi": [
            r"uwi\s*[:\-]?\s*(\d[\d\-]+)",
            r"unique\s*well\s*id(?:entifier)?\s*[:\-]?\s*(\d[\d\-]+)",
        ],
        "api_number": [
            r"api\s*(?:number|no\.?)?\s*[:\-]?\s*(\d[\d\-]+)",
        ],
        "operator": [
            r"operator\s*[:\-]?\s*(.+)",
            r"company\s*[:\-]?\s*(.+)",
        ],
        "field": [
            r"field\s*[:\-]?\s*(.+)",
        ],
        "county": [
            r"county\s*[:\-]?\s*(.+)",
        ],
        "state": [
            r"state\s*[:\-]?\s*(.+)",
            r"province\s*[:\-]?\s*(.+)",
        ],
        "country": [
            r"country\s*[:\-]?\s*(.+)",
        ],
        "spud_date": [
            r"spud\s*date\s*[:\-]?\s*(\d{1,2}[\/\-]\d{1,2}[\/\-]\d{2,4})",
            r"spud\s*[:\-]?\s*(\d{1,2}[\/\-]\d{1,2}[\/\-]\d{2,4})",
        ],
        "completion_date": [
            r"completion\s*date\s*[:\-]?\s*(\d{1,2}[\/\-]\d{1,2}[\/\-]\d{2,4})",
            r"completed\s*[:\-]?\s*(\d{1,2}[\/\-]\d{1,2}[\/\-]\d{2,4})",
        ],
        "kb_elevation": [
            r"kb\s*(?:elevation)?\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
            r"kelly\s*bushing\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
        ],
        "ground_elevation": [
            r"ground\s*(?:level|elevation)\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
            r"gl\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
        ],
        "total_depth": [
            r"(?:total|final)\s*depth\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
            r"td\s*[:\-]?\s*([\d,]+\.?\d*)\s*(?:ft|m)?",
        ],
    }

    def extract(self, document: ParsedDocument) -> dict[str, Any]:
        """Extract well header information from a parsed document.

        Args:
            document: Parsed PDF document.

        Returns:
            Dictionary with extracted well header fields.
        """
        header = WellHeader()
        text = document.full_text.lower()

        for field_name, patterns in self.PATTERNS.items():
            for pattern in patterns:
                match = re.search(pattern, text, re.IGNORECASE | re.MULTILINE)
                if match:
                    value = match.group(1).strip()
                    # Clean up the value
                    value = re.sub(r"\s+", " ", value)
                    value = value.rstrip(".,;:")

                    # Convert numeric fields
                    if field_name in ("kb_elevation", "ground_elevation", "total_depth"):
                        try:
                            value = float(value.replace(",", ""))
                        except ValueError:
                            continue

                    setattr(header, field_name, value)
                    break

        # Try to extract coordinates from tables
        self._extract_coordinates_from_tables(document, header)

        return header.to_dict()

    def _extract_coordinates_from_tables(
        self, document: ParsedDocument, header: WellHeader
    ) -> None:
        """Try to extract coordinate information from tables.

        Args:
            document: Parsed document.
            header: WellHeader to populate.
        """
        for table in document.all_tables:
            headers = [str(h).lower() for h in table.get("headers", [])]
            rows = table.get("rows", [])

            # Look for coordinate columns
            lat_idx = next(
                (i for i, h in enumerate(headers) if "lat" in h), None
            )
            lon_idx = next(
                (i for i, h in enumerate(headers) if "lon" in h), None
            )
            x_idx = next(
                (i for i, h in enumerate(headers) if h in ("x", "easting")), None
            )
            y_idx = next(
                (i for i, h in enumerate(headers) if h in ("y", "northing")), None
            )

            if rows:
                row = rows[0]  # First row usually has surface location
                if lat_idx is not None and lon_idx is not None:
                    try:
                        header.surface_latitude = float(row[lat_idx])
                        header.surface_longitude = float(row[lon_idx])
                    except (ValueError, IndexError):
                        pass
                if x_idx is not None and y_idx is not None:
                    try:
                        header.surface_x = float(row[x_idx])
                        header.surface_y = float(row[y_idx])
                    except (ValueError, IndexError):
                        pass
