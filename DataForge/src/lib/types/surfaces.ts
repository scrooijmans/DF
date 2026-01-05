/**
 * Types for 3D surface data ingestion
 *
 * Corresponds to types in src-tauri/src/surface_commands.rs
 * Surfaces are workspace-level entities (horizons, faults, unconformities)
 * defined by X, Y, Z point data from CSV files.
 */

// Surface column types (matches SurfaceColumnType in Rust)
export type SurfaceColumnType =
	| 'x'
	| 'y'
	| 'z'
	| 'surface_name'
	| 'quality'
	| 'attribute'
	| 'unknown'

// Surface type classification (matches SurfaceType in Rust)
export type SurfaceType = 'horizon' | 'fault' | 'unconformity' | 'contact' | 'other'

// Z positive direction (matches ZPositiveDirection in Rust)
export type ZPositiveDirection = 'down' | 'up'

// Spatial extent bounding box
export interface SpatialExtent {
	min_x: number
	max_x: number
	min_y: number
	max_y: number
	min_z: number
	max_z: number
}

// Response from parse_surface_for_ingest command
export interface ParseSurfaceResponse {
	file_id: string
	file_name: string
	columns: SurfaceColumnInfo[]
	row_count: number
	delimiter: string
	is_multi_surface: boolean
	surface_names: string[]
	extent: SpatialExtent
	has_required_columns: boolean
	missing_required: string[]
	warnings: string[]
}

// Information about a column in the surface CSV
export interface SurfaceColumnInfo {
	index: number
	header: string
	detected_type: SurfaceColumnType
	confidence: number
	unit: string | null
	sample_values: string[]
}

// Configuration for a surface column (user overrides)
export interface SurfaceColumnConfig {
	index: number
	column_type: SurfaceColumnType | null
}

// Request for ingesting surface CSV files
export interface IngestSurfaceRequest {
	file_paths: string[]
	column_configs: SurfaceColumnConfig[]
	surface_name: string | null
	surface_type: SurfaceType | null
	crs: string | null
	xy_unit: string
	z_unit: string
	z_positive_direction: ZPositiveDirection
}

// Response from ingest_surface_files command
export interface IngestSurfaceResponse {
	success: boolean
	surface_ids: string[]
	surface_count: number
	total_points: number
	error: string | null
	warnings: string[]
}

// Summary of a surface
export interface SurfaceSummary {
	id: string
	name: string
	surface_type: SurfaceType | null
	point_count: number
	min_x: number
	max_x: number
	min_y: number
	max_y: number
	min_z: number
	max_z: number
	crs: string | null
	xy_unit: string
	z_unit: string
	imported_at: string
}

// Helper to get human-readable column name
export function getSurfaceColumnDisplayName(type: SurfaceColumnType): string {
	switch (type) {
		case 'x':
			return 'X (Easting)'
		case 'y':
			return 'Y (Northing)'
		case 'z':
			return 'Z (Depth/Elevation)'
		case 'surface_name':
			return 'Surface Name'
		case 'quality':
			return 'Quality'
		case 'attribute':
			return 'Attribute'
		case 'unknown':
			return 'Unknown'
	}
}

// Helper to get surface type display name
export function getSurfaceTypeDisplayName(type: SurfaceType): string {
	switch (type) {
		case 'horizon':
			return 'Horizon'
		case 'fault':
			return 'Fault'
		case 'unconformity':
			return 'Unconformity'
		case 'contact':
			return 'Contact'
		case 'other':
			return 'Other'
	}
}

// Helper to get z direction display name
export function getZDirectionDisplayName(dir: ZPositiveDirection): string {
	switch (dir) {
		case 'down':
			return 'Down (Depth)'
		case 'up':
			return 'Up (Elevation)'
	}
}

// Required column types for basic ingestion
export const REQUIRED_SURFACE_COLUMNS: SurfaceColumnType[] = ['x', 'y', 'z']

// Optional column types
export const OPTIONAL_SURFACE_COLUMNS: SurfaceColumnType[] = ['surface_name', 'quality', 'attribute']

// Common CRS options for surfaces
export const COMMON_CRS_OPTIONS = [
	{ value: 'EPSG:4326', label: 'WGS 84 (Lat/Lon)' },
	{ value: 'EPSG:32631', label: 'UTM Zone 31N' },
	{ value: 'EPSG:32632', label: 'UTM Zone 32N' },
	{ value: 'EPSG:32633', label: 'UTM Zone 33N' },
	{ value: 'EPSG:32634', label: 'UTM Zone 34N' },
	{ value: 'EPSG:2154', label: 'RGF93 / Lambert-93 (France)' },
	{ value: 'EPSG:27700', label: 'OSGB 1936 / British National Grid' },
	{ value: 'EPSG:3857', label: 'Web Mercator' }
]

// Unit options for coordinates
export const XY_UNIT_OPTIONS = [
	{ value: 'm', label: 'Meters (m)' },
	{ value: 'ft', label: 'Feet (ft)' },
	{ value: 'km', label: 'Kilometers (km)' }
]

export const Z_UNIT_OPTIONS = [
	{ value: 'm', label: 'Meters (m)' },
	{ value: 'ft', label: 'Feet (ft)' }
]

// Surface type options for dropdown
export const SURFACE_TYPE_OPTIONS: { value: SurfaceType; label: string }[] = [
	{ value: 'horizon', label: 'Horizon' },
	{ value: 'fault', label: 'Fault' },
	{ value: 'unconformity', label: 'Unconformity' },
	{ value: 'contact', label: 'Contact' },
	{ value: 'other', label: 'Other' }
]

// Z direction options
export const Z_DIRECTION_OPTIONS: { value: ZPositiveDirection; label: string }[] = [
	{ value: 'down', label: 'Down (Depth increases downward)' },
	{ value: 'up', label: 'Up (Elevation increases upward)' }
]
