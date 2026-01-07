"""Casing program extractor (placeholder for future implementation)."""

from typing import Any

from dataforge_parser.parsers.docling_parser import ParsedDocument


class CasingExtractor:
    """Extracts casing program data from EOW reports.

    TODO: Implement casing extraction for:
    - Casing strings (conductor, surface, intermediate, production)
    - Casing specifications (OD, weight, grade, connections)
    - Setting depths
    - Cement tops
    """

    def extract(self, document: ParsedDocument) -> list[dict[str, Any]]:
        """Extract casing data from a parsed document.

        Args:
            document: Parsed PDF document.

        Returns:
            List of casing string dictionaries.
        """
        # Placeholder implementation
        return []
