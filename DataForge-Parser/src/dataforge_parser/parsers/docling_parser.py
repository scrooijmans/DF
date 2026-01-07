"""Docling-based PDF parser for high-accuracy document extraction."""

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


@dataclass
class DocumentPage:
    """Represents a single page from a parsed document."""

    page_number: int
    text: str
    tables: list[dict[str, Any]] = field(default_factory=list)
    sections: list[dict[str, Any]] = field(default_factory=list)


@dataclass
class ParsedDocument:
    """Represents a fully parsed document."""

    source_path: Path
    pages: list[DocumentPage] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)

    @property
    def full_text(self) -> str:
        """Return concatenated text from all pages."""
        return "\n\n".join(page.text for page in self.pages)

    @property
    def all_tables(self) -> list[dict[str, Any]]:
        """Return all tables from all pages."""
        tables = []
        for page in self.pages:
            tables.extend(page.tables)
        return tables


class DoclingParser:
    """PDF parser using IBM Docling for high-accuracy extraction.

    Docling provides:
    - 97.9% accuracy on table extraction
    - Layout-aware text extraction
    - Section detection
    - Multiple output formats (Markdown, JSON, DocTags)

    This parser is optimized for End-of-Well (EOW) reports with:
    - TableFormerMode.ACCURATE for complex tables
    - Cell matching for precise table structure
    - Optional OCR for scanned documents
    """

    def __init__(self, use_ocr: bool = False, accurate_tables: bool = True) -> None:
        """Initialize the Docling parser.

        Args:
            use_ocr: Whether to enable OCR for scanned documents.
            accurate_tables: Use high-accuracy table extraction mode.
        """
        self.use_ocr = use_ocr
        self.accurate_tables = accurate_tables
        self._converter = None

    def _get_converter(self):
        """Lazy-load the Docling converter with EOW-optimized settings."""
        if self._converter is None:
            try:
                from docling.document_converter import DocumentConverter, PdfFormatOption
                from docling.datamodel.pipeline_options import PdfPipelineOptions
                from docling.datamodel.base_models import InputFormat

                # Configure pipeline options for EOW reports
                pipeline_options = PdfPipelineOptions()

                # Enable high-accuracy table extraction
                pipeline_options.do_table_structure = True

                if self.accurate_tables:
                    try:
                        from docling.datamodel.pipeline_options import TableFormerMode

                        pipeline_options.table_structure_options.do_cell_matching = True
                        pipeline_options.table_structure_options.mode = TableFormerMode.ACCURATE
                    except ImportError:
                        pass

                # Enable OCR for scanned sections if requested
                if self.use_ocr:
                    pipeline_options.do_ocr = True

                self._converter = DocumentConverter(
                    format_options={
                        InputFormat.PDF: PdfFormatOption(pipeline_options=pipeline_options)
                    }
                )
            except ImportError:
                raise ImportError(
                    "Docling is not installed. Install with: pip install docling"
                )
        return self._converter

    def parse(self, pdf_path: Path | str) -> ParsedDocument:
        """Parse a PDF document.

        Args:
            pdf_path: Path to the PDF file.

        Returns:
            ParsedDocument with extracted text, tables, and metadata.
        """
        pdf_path = Path(pdf_path)
        if not pdf_path.exists():
            raise FileNotFoundError(f"PDF not found: {pdf_path}")

        converter = self._get_converter()

        # Convert the document
        result = converter.convert(str(pdf_path))

        # Get the document
        doc = result.document

        # Extract tables with enhanced metadata
        tables = self._extract_tables(doc)

        # Extract text and structure
        markdown_text = doc.export_to_markdown()

        # For now, treat as single page with full content
        page = DocumentPage(
            page_number=1,
            text=markdown_text,
            tables=tables,
            sections=self._extract_sections(doc),
        )

        return ParsedDocument(
            source_path=pdf_path,
            pages=[page],
            metadata={
                "page_count": 1,
                "table_count": len(tables),
                "has_tables": len(tables) > 0,
                "has_sections": len(page.sections) > 0,
                "ocr_enabled": self.use_ocr,
                "accurate_tables": self.accurate_tables,
            },
        )

    def _extract_tables(self, doc) -> list[dict[str, Any]]:
        """Extract tables from a Docling document with enhanced metadata.

        Args:
            doc: Docling document object.

        Returns:
            List of table dictionaries with headers, rows, and metadata.
        """
        tables = []

        try:
            # Try using doc.tables if available (Docling 2.x)
            if hasattr(doc, "tables"):
                for idx, table in enumerate(doc.tables):
                    table_data = self._process_table(table, doc, idx)
                    if table_data:
                        tables.append(table_data)
            else:
                # Fallback: iterate through document items
                idx = 0
                for item, level in doc.iterate_items():
                    if hasattr(item, "label") and "table" in str(item.label).lower():
                        table_data = self._process_table(item, doc, idx)
                        if table_data:
                            tables.append(table_data)
                            idx += 1
        except Exception:
            # If table extraction fails, return empty list
            pass

        return tables

    def _process_table(self, table, doc, idx: int) -> dict[str, Any] | None:
        """Process a single table into a structured dictionary.

        Args:
            table: Docling table object.
            doc: Parent document (needed for export_to_dataframe).
            idx: Table index.

        Returns:
            Table dictionary or None if processing fails.
        """
        try:
            table_data: dict[str, Any] = {
                "index": idx,
                "headers": [],
                "rows": [],
                "row_count": 0,
                "col_count": 0,
            }

            # Try to extract page number and bounding box
            if hasattr(table, "prov") and table.prov:
                prov = table.prov[0] if isinstance(table.prov, list) else table.prov
                if hasattr(prov, "page_no"):
                    table_data["page"] = prov.page_no
                if hasattr(prov, "bbox"):
                    table_data["bbox"] = {
                        "x0": prov.bbox.l,
                        "y0": prov.bbox.t,
                        "x1": prov.bbox.r,
                        "y1": prov.bbox.b,
                    }

            # Export to DataFrame for structured data
            if hasattr(table, "export_to_dataframe"):
                df = table.export_to_dataframe(doc=doc)
                table_data["headers"] = list(df.columns)
                table_data["rows"] = df.values.tolist()
                table_data["row_count"] = len(df)
                table_data["col_count"] = len(df.columns)
            elif hasattr(table, "data"):
                # Direct data access fallback
                if hasattr(table.data, "columns"):
                    table_data["headers"] = table.data.columns
                if hasattr(table.data, "values"):
                    table_data["rows"] = table.data.values

            return table_data if table_data["row_count"] > 0 else None

        except Exception:
            return None

    def _extract_sections(self, doc) -> list[dict[str, Any]]:
        """Extract section headers and content from a Docling document.

        Args:
            doc: Docling document object.

        Returns:
            List of section dictionaries with title and content.
        """
        sections = []

        try:
            current_section: dict[str, Any] | None = None
            content_buffer: list[str] = []

            for item, level in doc.iterate_items():
                label = str(getattr(item, "label", "")).lower()
                text = str(getattr(item, "text", "")).strip()

                if not text:
                    continue

                # Detect section headers
                is_heading = (
                    "heading" in label
                    or "title" in label
                    or "section" in label
                    or (hasattr(item, "level") and item.level <= 2)
                )

                if is_heading:
                    # Save previous section
                    if current_section:
                        current_section["content"] = "\n".join(content_buffer)
                        sections.append(current_section)
                        content_buffer = []

                    current_section = {
                        "title": text,
                        "level": level,
                        "content": "",
                    }
                elif current_section is not None:
                    content_buffer.append(text)

            # Don't forget the last section
            if current_section:
                current_section["content"] = "\n".join(content_buffer)
                sections.append(current_section)

        except Exception:
            pass

        return sections


def create_eow_parser(use_ocr: bool = False) -> DoclingParser:
    """Create a Docling parser optimized for End-of-Well reports.

    Args:
        use_ocr: Enable OCR for scanned documents.

    Returns:
        Configured DoclingParser instance.
    """
    return DoclingParser(use_ocr=use_ocr, accurate_tables=True)
