"""SQLite writer for DataForge database integration."""

from pathlib import Path
from typing import Any


class SqliteWriter:
    """Writes parsed data to DataForge's SQLite database.

    This writer is designed to integrate with DataForge's existing
    database schema defined in db.rs.

    Note: This is a placeholder implementation. In production, this
    would either:
    1. Output JSON for DataForge to ingest via its existing workflow
    2. Write directly to SQLite if running as a sidecar

    The recommended approach is (1) - output JSON that matches
    DataForge's ingest types, letting the Rust backend handle
    actual database writes.
    """

    def __init__(self, db_path: Path | None = None) -> None:
        """Initialize the writer.

        Args:
            db_path: Optional path to SQLite database.
                    If None, outputs will be JSON only.
        """
        self.db_path = db_path
        self._conn = None

    def write_well(self, well_data: dict[str, Any]) -> str:
        """Write well data.

        Args:
            well_data: OSDU-aligned well record.

        Returns:
            Well ID.
        """
        # In JSON mode, just return the ID
        return well_data.get("id", "")

    def write_marker_set(
        self,
        marker_set: dict[str, Any],
        well_id: str,
        workspace_id: str,
        member_id: str,
    ) -> str:
        """Write marker set data.

        Args:
            marker_set: OSDU-aligned marker set with markers.
            well_id: Associated well ID.
            workspace_id: Workspace ID.
            member_id: Importing user's member ID.

        Returns:
            Marker set ID.
        """
        return marker_set.get("id", "")

    def write_survey_run(
        self,
        survey_run: dict[str, Any],
        well_id: str,
        workspace_id: str,
        member_id: str,
    ) -> str:
        """Write survey run data.

        Args:
            survey_run: OSDU-aligned survey run with columns.
            well_id: Associated well ID.
            workspace_id: Workspace ID.
            member_id: Importing user's member ID.

        Returns:
            Survey run ID.
        """
        return survey_run.get("id", "")

    def to_dataforge_ingest_format(
        self,
        well: dict[str, Any] | None = None,
        marker_set: dict[str, Any] | None = None,
        survey_run: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        """Convert parsed data to DataForge ingest format.

        This produces JSON that can be consumed by DataForge's
        existing ingest workflow (via Tauri IPC).

        Args:
            well: Optional well data.
            marker_set: Optional marker set data.
            survey_run: Optional survey run data.

        Returns:
            DataForge-compatible ingest payload.
        """
        payload: dict[str, Any] = {
            "source": "dataforge-parser",
            "version": "1.0.0",
        }

        if well:
            payload["well"] = {
                "name": well.get("name"),
                "uwi": well.get("uwi"),
                "field": well.get("field"),
                "company": well.get("company"),
                "operator": well.get("operator"),
                "spud_date": well.get("spud_date"),
                "surface_x": well.get("surface_x"),
                "surface_y": well.get("surface_y"),
                "surface_crs": well.get("surface_crs"),
                "country": well.get("country"),
                "state_province": well.get("state_province"),
                "county": well.get("county"),
            }

        if marker_set:
            payload["markers"] = {
                "set_name": marker_set.get("name"),
                "interpretation_type": marker_set.get("interpretation_type"),
                "depth_unit": marker_set.get("depth_unit", "ft"),
                "markers": [
                    {
                        "name": m.get("name"),
                        "measured_depth": m.get("measured_depth"),
                        "true_vertical_depth": m.get("true_vertical_depth"),
                        "marker_type": m.get("marker_type"),
                        "quality": m.get("quality"),
                    }
                    for m in marker_set.get("markers", [])
                ],
            }

        if survey_run:
            payload["trajectory"] = {
                "survey_type": survey_run.get("survey_type"),
                "azimuth_reference": survey_run.get("azimuth_reference"),
                "magnetic_declination": survey_run.get("magnetic_declination"),
                "grid_convergence": survey_run.get("grid_convergence"),
                "md_unit": survey_run.get("md_unit", "ft"),
                "stations": self._columns_to_stations(survey_run.get("columns", [])),
            }

        return payload

    def _columns_to_stations(
        self, columns: list[dict[str, Any]]
    ) -> list[dict[str, Any]]:
        """Convert column format back to stations.

        Args:
            columns: List of column definitions with values.

        Returns:
            List of station dictionaries.
        """
        if not columns:
            return []

        # Find MD column to determine station count
        md_col = next((c for c in columns if c.get("column_type") == "md"), None)
        if not md_col:
            return []

        sample_count = md_col.get("sample_count", 0)
        stations = []

        for i in range(sample_count):
            station: dict[str, Any] = {}
            for col in columns:
                col_type = col.get("column_type")
                values = col.get("values", [])
                if i < len(values):
                    station[col_type] = values[i]
            stations.append(station)

        return stations
