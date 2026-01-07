"""JSON schema validator for OSDU-aligned data."""

from typing import Any


class SchemaValidator:
    """Validates extracted data against OSDU JSON schemas.

    This provides basic validation to ensure extracted data matches
    expected structure before insertion into DataForge.
    """

    # Basic schema definitions (simplified from full OSDU schemas)
    SCHEMAS = {
        "well": {
            "required": ["name"],
            "properties": {
                "id": {"type": "string"},
                "name": {"type": "string"},
                "uwi": {"type": ["string", "null"]},
                "field": {"type": ["string", "null"]},
                "company": {"type": ["string", "null"]},
                "operator": {"type": ["string", "null"]},
                "spud_date": {"type": ["string", "null"]},
                "surface_x": {"type": ["number", "null"]},
                "surface_y": {"type": ["number", "null"]},
                "surface_crs": {"type": ["string", "null"]},
                "country": {"type": ["string", "null"]},
                "state_province": {"type": ["string", "null"]},
                "county": {"type": ["string", "null"]},
                "depth_unit": {"type": "string"},
                "min_depth": {"type": ["number", "null"]},
                "max_depth": {"type": ["number", "null"]},
                "schema_version": {"type": "string"},
            },
        },
        "wellbore-marker-set": {
            "required": ["markers"],
            "properties": {
                "id": {"type": "string"},
                "well_id": {"type": ["string", "null"]},
                "name": {"type": ["string", "null"]},
                "interpretation_type": {"type": ["string", "null"]},
                "depth_unit": {"type": "string"},
                "min_depth": {"type": ["number", "null"]},
                "max_depth": {"type": ["number", "null"]},
                "marker_count": {"type": "integer"},
                "schema_version": {"type": "string"},
                "markers": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["name", "measured_depth"],
                        "properties": {
                            "name": {"type": "string"},
                            "measured_depth": {"type": "number"},
                            "true_vertical_depth": {"type": ["number", "null"]},
                            "marker_type": {"type": ["string", "null"]},
                            "quality": {"type": ["string", "null"]},
                        },
                    },
                },
            },
        },
        "wellbore-trajectory": {
            "required": ["station_count"],
            "properties": {
                "id": {"type": "string"},
                "well_id": {"type": ["string", "null"]},
                "survey_type": {"type": ["string", "null"]},
                "azimuth_reference": {"type": "string"},
                "top_md": {"type": ["number", "null"]},
                "bottom_md": {"type": ["number", "null"]},
                "station_count": {"type": "integer"},
                "md_unit": {"type": "string"},
                "schema_version": {"type": "string"},
                "columns": {"type": "array"},
            },
        },
    }

    def validate(self, data: dict[str, Any], schema_name: str) -> list[str]:
        """Validate data against a schema.

        Args:
            data: Data to validate.
            schema_name: Name of the schema to validate against.

        Returns:
            List of validation error messages (empty if valid).
        """
        errors: list[str] = []

        schema = self.SCHEMAS.get(schema_name)
        if not schema:
            return [f"Unknown schema: {schema_name}"]

        # Check required fields
        for field in schema.get("required", []):
            if field not in data:
                errors.append(f"Missing required field: {field}")
            elif data[field] is None:
                errors.append(f"Required field cannot be null: {field}")

        # Check property types
        properties = schema.get("properties", {})
        for field, value in data.items():
            if field in properties:
                field_schema = properties[field]
                type_errors = self._validate_type(field, value, field_schema)
                errors.extend(type_errors)

        return errors

    def _validate_type(
        self, field: str, value: Any, field_schema: dict[str, Any]
    ) -> list[str]:
        """Validate a field's type.

        Args:
            field: Field name.
            value: Field value.
            field_schema: Field's schema definition.

        Returns:
            List of type validation errors.
        """
        errors: list[str] = []

        expected_type = field_schema.get("type")
        if not expected_type:
            return errors

        # Handle union types (e.g., ["string", "null"])
        if isinstance(expected_type, list):
            valid = False
            for t in expected_type:
                if self._check_type(value, t):
                    valid = True
                    break
            if not valid:
                errors.append(
                    f"Field '{field}' has invalid type. "
                    f"Expected one of {expected_type}, got {type(value).__name__}"
                )
        else:
            if not self._check_type(value, expected_type):
                errors.append(
                    f"Field '{field}' has invalid type. "
                    f"Expected {expected_type}, got {type(value).__name__}"
                )

        # Validate array items
        if expected_type == "array" and isinstance(value, list):
            items_schema = field_schema.get("items", {})
            if items_schema:
                for i, item in enumerate(value):
                    item_errors = self._validate_item(f"{field}[{i}]", item, items_schema)
                    errors.extend(item_errors)

        return errors

    def _validate_item(
        self, path: str, item: Any, schema: dict[str, Any]
    ) -> list[str]:
        """Validate an array item.

        Args:
            path: Path to the item for error messages.
            item: Item to validate.
            schema: Item schema.

        Returns:
            List of validation errors.
        """
        errors: list[str] = []

        if schema.get("type") == "object" and isinstance(item, dict):
            # Check required fields
            for field in schema.get("required", []):
                if field not in item:
                    errors.append(f"Missing required field: {path}.{field}")

        return errors

    def _check_type(self, value: Any, expected: str) -> bool:
        """Check if a value matches an expected type.

        Args:
            value: Value to check.
            expected: Expected type name.

        Returns:
            True if type matches.
        """
        if expected == "null":
            return value is None
        elif expected == "string":
            return isinstance(value, str)
        elif expected == "number":
            return isinstance(value, (int, float)) and not isinstance(value, bool)
        elif expected == "integer":
            return isinstance(value, int) and not isinstance(value, bool)
        elif expected == "boolean":
            return isinstance(value, bool)
        elif expected == "array":
            return isinstance(value, list)
        elif expected == "object":
            return isinstance(value, dict)
        return False
