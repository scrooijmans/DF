"""Command-line interface for DataForge-Parser."""

import json
import sys
from pathlib import Path

import click
from rich.console import Console
from rich.table import Table
from rich.panel import Panel

console = Console()


@click.group()
@click.version_option(version="0.1.0")
def main() -> None:
    """DataForge-Parser: Extract structured data from End-of-Well PDF reports."""
    pass


@main.command()
@click.argument("pdf_path", type=click.Path(exists=True, path_type=Path))
@click.option(
    "--output",
    "-o",
    type=click.Path(path_type=Path),
    help="Output JSON file path. Use '-' for stdout.",
)
@click.option(
    "--ocr/--no-ocr",
    default=False,
    help="Enable OCR for scanned documents.",
)
@click.option(
    "--validate/--no-validate",
    default=True,
    help="Validate extracted data against OSDU schemas.",
)
@click.option(
    "--verbose",
    "-v",
    is_flag=True,
    help="Show detailed progress and extraction info.",
)
def parse(
    pdf_path: Path,
    output: Path | None,
    ocr: bool,
    validate: bool,
    verbose: bool,
) -> None:
    """Parse a PDF document and extract structured data.

    PDF_PATH: Path to the PDF file to parse.

    Examples:

        dataforge-parser parse report.pdf -o output.json

        dataforge-parser parse report.pdf --ocr --verbose

        dataforge-parser parse report.pdf -o - | jq .
    """
    if verbose:
        console.print(f"[bold blue]Parsing:[/] {pdf_path}")
        console.print(f"[dim]OCR enabled: {ocr}[/]")
        console.print(f"[dim]Validation: {validate}[/]")

    try:
        from dataforge_parser.pipeline import ParsingPipeline

        # Create and run pipeline
        pipeline = ParsingPipeline(use_ocr=ocr, validate=validate)

        if verbose:
            console.print("[dim]Loading Docling parser...[/]")

        result = pipeline.process(pdf_path)

        if verbose:
            _print_extraction_summary(result)

        # Convert to dict for output
        output_data = result.to_dict()

        # Output results
        if output:
            if str(output) == "-":
                # Output to stdout (for piping to other tools)
                print(json.dumps(output_data, indent=2, default=str))
            else:
                with open(output, "w") as f:
                    json.dump(output_data, f, indent=2, default=str)
                console.print(f"[bold green]Output written to:[/] {output}")
        else:
            console.print_json(json.dumps(output_data, indent=2, default=str))

        # Exit with error if validation failed
        if not result.is_valid and validate:
            if verbose:
                console.print("[yellow]Warning: Validation errors present[/]")
            sys.exit(1)

    except ImportError as e:
        console.print(f"[bold red]Error:[/] Missing dependency - {e}")
        console.print("Install with: pip install dataforge-parser")
        console.print("For OCR support: pip install 'dataforge-parser[ocr]'")
        sys.exit(1)
    except FileNotFoundError as e:
        console.print(f"[bold red]Error:[/] {e}")
        sys.exit(1)
    except Exception as e:
        console.print(f"[bold red]Error:[/] {e}")
        if verbose:
            import traceback
            console.print(traceback.format_exc())
        sys.exit(1)


def _print_extraction_summary(result) -> None:
    """Print a summary of what was extracted."""
    from dataforge_parser.pipeline import PipelineResult

    if not isinstance(result, PipelineResult):
        return

    console.print()
    console.print(Panel.fit("[bold]Extraction Summary[/]", border_style="blue"))

    # Well header
    if result.well:
        well_name = result.well.get("name", "Unknown")
        console.print(f"  [green]Well Header:[/] {well_name}")
    else:
        console.print("  [yellow]Well Header:[/] Not found")

    # Markers
    if result.markers:
        count = result.markers.get("marker_count", 0)
        console.print(f"  [green]Formation Tops:[/] {count} markers")
    else:
        console.print("  [yellow]Formation Tops:[/] Not found")

    # Trajectory
    if result.trajectory:
        count = result.trajectory.get("station_count", 0)
        console.print(f"  [green]Survey Data:[/] {count} stations")
    else:
        console.print("  [yellow]Survey Data:[/] Not found")

    # Validation errors
    if result.validation_errors:
        console.print()
        console.print("[bold yellow]Validation Issues:[/]")
        for section, errors in result.validation_errors.items():
            console.print(f"  [yellow]{section}:[/]")
            for error in errors:
                console.print(f"    - {error}")

    console.print()


@main.command()
@click.argument("json_path", type=click.Path(exists=True, path_type=Path))
@click.option(
    "--schema",
    "-s",
    type=click.Choice(["well", "wellbore-marker-set", "wellbore-trajectory", "all"]),
    default="all",
    help="OSDU schema to validate against.",
)
def validate(json_path: Path, schema: str) -> None:
    """Validate extracted data against OSDU schemas.

    JSON_PATH: Path to the JSON file to validate.
    """
    console.print(f"[bold blue]Validating:[/] {json_path}")

    try:
        from dataforge_parser.validators.schema_validator import SchemaValidator

        with open(json_path) as f:
            data = json.load(f)

        validator = SchemaValidator()
        all_errors: dict[str, list[str]] = {}

        schemas_to_check = []
        if schema == "all":
            # Auto-detect which schemas to validate based on data keys
            if "well" in data:
                schemas_to_check.append(("well", data["well"]))
            if "markers" in data:
                schemas_to_check.append(("wellbore-marker-set", data["markers"]))
            if "trajectory" in data:
                schemas_to_check.append(("wellbore-trajectory", data["trajectory"]))
        else:
            schemas_to_check.append((schema, data))

        for schema_name, schema_data in schemas_to_check:
            console.print(f"[dim]Checking {schema_name}...[/]")
            errors = validator.validate(schema_data, schema_name)
            if errors:
                all_errors[schema_name] = errors

        if all_errors:
            console.print(f"[bold red]Validation failed:[/]")
            for section, errors in all_errors.items():
                console.print(f"  [yellow]{section}:[/]")
                for error in errors:
                    console.print(f"    - {error}")
            sys.exit(1)
        else:
            console.print("[bold green]Validation passed![/]")

    except Exception as e:
        console.print(f"[bold red]Error:[/] {e}")
        sys.exit(1)


@main.command()
@click.argument("pdf_path", type=click.Path(exists=True, path_type=Path))
def info(pdf_path: Path) -> None:
    """Show information about a PDF document without full parsing.

    PDF_PATH: Path to the PDF file to analyze.
    """
    console.print(f"[bold blue]Analyzing:[/] {pdf_path}")

    try:
        from dataforge_parser.parsers.docling_parser import DoclingParser

        parser = DoclingParser(accurate_tables=False)  # Fast mode for info
        document = parser.parse(pdf_path)

        console.print()
        console.print(Panel.fit("[bold]Document Information[/]", border_style="blue"))
        console.print(f"  [cyan]Pages:[/] {len(document.pages)}")
        console.print(f"  [cyan]Tables Found:[/] {len(document.all_tables)}")

        # Show table summary
        if document.all_tables:
            console.print()
            console.print("[bold]Tables:[/]")
            for i, table in enumerate(document.all_tables):
                headers = table.get("headers", [])
                row_count = table.get("row_count", len(table.get("rows", [])))
                console.print(f"  Table {i + 1}: {row_count} rows, columns: {headers[:5]}{'...' if len(headers) > 5 else ''}")

        # Show sections
        if document.pages and document.pages[0].sections:
            console.print()
            console.print("[bold]Sections Detected:[/]")
            for section in document.pages[0].sections[:10]:
                title = section.get("title", "Untitled")[:50]
                console.print(f"  - {title}")
            if len(document.pages[0].sections) > 10:
                console.print(f"  ... and {len(document.pages[0].sections) - 10} more")

    except ImportError as e:
        console.print(f"[bold red]Error:[/] Missing dependency - {e}")
        sys.exit(1)
    except Exception as e:
        console.print(f"[bold red]Error:[/] {e}")
        sys.exit(1)


@main.command(name="list")
def list_extractors() -> None:
    """List all available extractors and their status."""
    table = Table(title="Supported Extractors")
    table.add_column("Extractor", style="cyan")
    table.add_column("OSDU Kind", style="green")
    table.add_column("Target Table", style="blue")
    table.add_column("Status", style="yellow")

    extractors = [
        ("well-header", "master-data--Well", "wells", "[green]MVP[/]"),
        ("formations", "work-product-component--WellboreMarkerSet", "marker_sets", "[green]MVP[/]"),
        ("surveys", "work-product-component--WellboreTrajectory", "survey_runs", "[green]MVP[/]"),
        ("casing", "work-product-component--WellboreCompletion", "casing_strings", "[yellow]Planned[/]"),
        ("cement", "work-product-component--CementJob", "cement_jobs", "[yellow]Planned[/]"),
        ("mud", "work-product-component--MudLog", "mud_logs", "[yellow]Planned[/]"),
        ("tests", "work-product-component--WellTest", "well_tests", "[yellow]Planned[/]"),
    ]

    for name, kind, target, status in extractors:
        table.add_row(name, kind, target, status)

    console.print(table)

    console.print()
    console.print("[dim]MVP extractors are fully implemented.[/]")
    console.print("[dim]Planned extractors are in development.[/]")


if __name__ == "__main__":
    main()
