"""OSDU schema mapper for DataForge integration."""

from datetime import datetime
from typing import Any
from uuid import uuid4


class OsduMapper:
    """Maps extracted data to OSDU-aligned schemas.

    This mapper transforms extracted data into formats compatible with
    DataForge's OSDU-aligned SQLite schema.
    """

    def __init__(self) -> None:
        """Initialize the mapper."""
        self.schema_version = "1.0.0"

    def map_well(self, well_header: dict[str, Any]) -> dict[str, Any]:
        """Map extracted well header to OSDU Well schema.

        Args:
            well_header: Extracted well header data.

        Returns:
            OSDU-aligned well record.
        """
        return {
            "id": str(uuid4()),
            "name": well_header.get("name"),
            "uwi": well_header.get("uwi"),
            "field": well_header.get("field"),
            "company": well_header.get("operator"),
            "operator": well_header.get("operator"),
            "spud_date": self._format_date(well_header.get("spud_date")),
            "surface_x": well_header.get("surface_x") or well_header.get("surface_longitude"),
            "surface_y": well_header.get("surface_y") or well_header.get("surface_latitude"),
            "surface_crs": well_header.get("surface_crs"),
            "country": well_header.get("country"),
            "state_province": well_header.get("state"),
            "county": well_header.get("county"),
            "depth_unit": well_header.get("depth_unit", "ft"),
            "min_depth": 0,
            "max_depth": well_header.get("total_depth"),
            "schema_version": self.schema_version,
        }

    def map_marker_set(
        self,
        markers: list[dict[str, Any]],
        well_id: str | None = None,
        set_name: str = "Formation Tops",
    ) -> dict[str, Any]:
        """Map extracted formations to OSDU WellboreMarkerSet schema.

        Args:
            markers: List of extracted formation markers.
            well_id: Optional well ID to associate with.
            set_name: Name for the marker set.

        Returns:
            OSDU-aligned marker set record with markers.
        """
        if not markers:
            return {"markers": []}

        depths = [m["measured_depth"] for m in markers if m.get("measured_depth")]

        return {
            "id": str(uuid4()),
            "well_id": well_id,
            "name": set_name,
            "interpretation_type": "formation",
            "depth_unit": "ft",
            "min_depth": min(depths) if depths else None,
            "max_depth": max(depths) if depths else None,
            "marker_count": len(markers),
            "schema_version": self.schema_version,
            "markers": [
                {
                    "id": str(uuid4()),
                    "name": m.get("name"),
                    "measured_depth": m.get("measured_depth"),
                    "true_vertical_depth": m.get("true_vertical_depth"),
                    "true_vertical_depth_ss": m.get("tvd_subsea"),
                    "thickness": m.get("thickness"),
                    "marker_type": m.get("marker_type", "formation"),
                    "quality": m.get("quality", "confirmed"),
                    "comments": m.get("comments"),
                }
                for m in markers
            ],
        }

    def map_survey_run(
        self,
        survey_data: dict[str, Any],
        well_id: str | None = None,
    ) -> dict[str, Any]:
        """Map extracted survey to OSDU WellboreTrajectory schema.

        Args:
            survey_data: Extracted survey data.
            well_id: Optional well ID to associate with.

        Returns:
            OSDU-aligned survey run record with columns.
        """
        stations = survey_data.get("stations", [])

        return {
            "id": str(uuid4()),
            "well_id": well_id,
            "survey_type": survey_data.get("survey_type"),
            "survey_company": survey_data.get("survey_company"),
            "survey_date": survey_data.get("survey_date"),
            "azimuth_reference": survey_data.get("azimuth_reference", "true_north"),
            "magnetic_declination": survey_data.get("magnetic_declination"),
            "grid_convergence": survey_data.get("grid_convergence"),
            "calculation_method": "minimum_curvature",
            "top_md": survey_data.get("top_md"),
            "bottom_md": survey_data.get("bottom_md"),
            "station_count": len(stations),
            "md_unit": survey_data.get("md_unit", "ft"),
            "schema_version": self.schema_version,
            "columns": self._build_trajectory_columns(stations),
        }

    def _build_trajectory_columns(
        self, stations: list[dict[str, Any]]
    ) -> list[dict[str, Any]]:
        """Build trajectory column records from survey stations.

        Args:
            stations: List of survey stations.

        Returns:
            List of column definitions with values.
        """
        if not stations:
            return []

        columns = []
        column_types = [
            ("md", "md", True, False),
            ("inclination", "inclination", True, False),
            ("azimuth", "azimuth", True, False),
            ("tvd", "tvd", False, True),
            ("ns", "ns", False, True),
            ("ew", "ew", False, True),
            ("dls", "dls", False, True),
        ]

        for col_type, col_name, is_input, is_calculated in column_types:
            values = [s.get(col_type) for s in stations]
            # Only include column if it has non-null values
            if any(v is not None for v in values):
                non_null = [v for v in values if v is not None]
                columns.append({
                    "id": str(uuid4()),
                    "column_type": col_type,
                    "column_name": col_name.upper(),
                    "is_input": is_input,
                    "is_calculated": is_calculated,
                    "min_value": min(non_null) if non_null else None,
                    "max_value": max(non_null) if non_null else None,
                    "mean_value": sum(non_null) / len(non_null) if non_null else None,
                    "sample_count": len(values),
                    "null_count": values.count(None),
                    "values": values,
                })

        return columns

    def _format_date(self, date_str: str | None) -> str | None:
        """Format a date string to ISO 8601.

        Args:
            date_str: Date string in various formats.

        Returns:
            ISO 8601 formatted date or None.
        """
        if not date_str:
            return None

        # Try common date formats
        formats = [
            "%m/%d/%Y",
            "%d/%m/%Y",
            "%Y-%m-%d",
            "%m-%d-%Y",
            "%d-%m-%Y",
        ]

        for fmt in formats:
            try:
                dt = datetime.strptime(date_str, fmt)
                return dt.strftime("%Y-%m-%d")
            except ValueError:
                continue

        return date_str  # Return as-is if no format matches
