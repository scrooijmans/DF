"""Formation tops / well markers extractor."""

import re
from dataclasses import dataclass
from typing import Any

from dataforge_parser.parsers.docling_parser import ParsedDocument


@dataclass
class FormationMarker:
    """A single formation top / marker."""

    name: str
    measured_depth: float
    true_vertical_depth: float | None = None
    tvd_subsea: float | None = None
    thickness: float | None = None
    marker_type: str = "formation"
    quality: str = "confirmed"
    comments: str | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        return {
            "name": self.name,
            "measured_depth": self.measured_depth,
            "true_vertical_depth": self.true_vertical_depth,
            "tvd_subsea": self.tvd_subsea,
            "thickness": self.thickness,
            "marker_type": self.marker_type,
            "quality": self.quality,
            "comments": self.comments,
        }


class FormationsExtractor:
    """Extracts formation tops / well markers from EOW reports.

    Looks for:
    - Formation tops tables
    - Stratigraphy sections
    - Zone boundary descriptions
    """

    # Keywords that indicate a formation tops section
    SECTION_KEYWORDS = [
        "formation tops",
        "formation markers",
        "well tops",
        "stratigraphy",
        "lithology",
        "zone boundaries",
        "geological markers",
    ]

    # Common column header patterns
    COLUMN_PATTERNS = {
        "name": ["formation", "marker", "zone", "unit", "name", "horizon"],
        "md": ["md", "measured depth", "depth", "top md", "top depth"],
        "tvd": ["tvd", "true vertical", "tvd depth"],
        "tvdss": ["tvdss", "tvd ss", "tvd subsea", "ss"],
        "thickness": ["thickness", "gross", "net", "interval"],
    }

    def extract(self, document: ParsedDocument) -> list[dict[str, Any]]:
        """Extract formation markers from a parsed document.

        Args:
            document: Parsed PDF document.

        Returns:
            List of formation marker dictionaries.
        """
        markers: list[FormationMarker] = []

        # First, try to extract from tables
        for table in document.all_tables:
            table_markers = self._extract_from_table(table)
            markers.extend(table_markers)

        # If no tables found, try text-based extraction
        if not markers:
            text_markers = self._extract_from_text(document)
            markers.extend(text_markers)

        return [m.to_dict() for m in markers]

    def _extract_from_table(self, table: dict[str, Any]) -> list[FormationMarker]:
        """Extract markers from a table.

        Args:
            table: Table dictionary with headers and rows.

        Returns:
            List of FormationMarker objects.
        """
        markers = []
        headers = [str(h).lower().strip() for h in table.get("headers", [])]
        rows = table.get("rows", [])

        if not headers or not rows:
            return markers

        # Find column indices
        name_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["name"])
        md_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["md"])
        tvd_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["tvd"])
        tvdss_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["tvdss"])
        thickness_idx = self._find_column_index(headers, self.COLUMN_PATTERNS["thickness"])

        # Must have at least name and depth columns
        if name_idx is None or md_idx is None:
            return markers

        for row in rows:
            try:
                name = str(row[name_idx]).strip()
                if not name or name.lower() in ("", "nan", "none", "-"):
                    continue

                md = self._parse_number(row[md_idx])
                if md is None:
                    continue

                marker = FormationMarker(
                    name=name,
                    measured_depth=md,
                    true_vertical_depth=self._parse_number(row[tvd_idx]) if tvd_idx else None,
                    tvd_subsea=self._parse_number(row[tvdss_idx]) if tvdss_idx else None,
                    thickness=self._parse_number(row[thickness_idx]) if thickness_idx else None,
                )
                markers.append(marker)
            except (IndexError, ValueError):
                continue

        return markers

    def _extract_from_text(self, document: ParsedDocument) -> list[FormationMarker]:
        """Extract markers from text content when tables aren't available.

        Args:
            document: Parsed document.

        Returns:
            List of FormationMarker objects.
        """
        markers = []
        text = document.full_text

        # Look for patterns like "Formation Name @ 1234 ft" or "Formation Name: 1234 MD"
        patterns = [
            r"([A-Za-z][A-Za-z\s]+?)\s*[@:]\s*([\d,]+\.?\d*)\s*(?:ft|m|MD)?",
            r"([\d,]+\.?\d*)\s*(?:ft|m)?\s*[-:]\s*([A-Za-z][A-Za-z\s]+)",
        ]

        for pattern in patterns:
            for match in re.finditer(pattern, text, re.IGNORECASE):
                try:
                    groups = match.groups()
                    # Determine which group is name and which is depth
                    if groups[0][0].isalpha():
                        name = groups[0].strip()
                        depth = float(groups[1].replace(",", ""))
                    else:
                        depth = float(groups[0].replace(",", ""))
                        name = groups[1].strip()

                    # Filter out obvious non-formation names
                    if len(name) < 3 or name.lower() in ("the", "and", "for"):
                        continue

                    marker = FormationMarker(
                        name=name,
                        measured_depth=depth,
                        quality="uncertain",  # Text extraction is less reliable
                    )
                    markers.append(marker)
                except (ValueError, IndexError):
                    continue

        return markers

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
