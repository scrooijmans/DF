"""PDF parsing backends."""

from dataforge_parser.parsers.docling_parser import (
    DoclingParser,
    ParsedDocument,
    DocumentPage,
    create_eow_parser,
)

__all__ = ["DoclingParser", "ParsedDocument", "DocumentPage", "create_eow_parser"]
