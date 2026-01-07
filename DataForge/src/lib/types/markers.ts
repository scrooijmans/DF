/**
 * Types for wellbore marker (well tops) data ingestion
 *
 * Corresponds to types in src-tauri/src/marker_commands.rs
 * Follows OSDU WellboreMarkerSet patterns with MarkerName and
 * MarkerMeasuredDepth as core required fields.
 */

// Marker column types (matches MarkerColumnType in Rust with snake_case serialization)
export type MarkerColumnType =
	| 'well_name'
	| 'well_uwi'
	| 'marker_name'
	| 'measured_depth'
	| 'true_vertical_depth'
	| 'true_vertical_depth_ss'
	| 'marker_type'
	| 'thickness'
	| 'quality'
	| 'interpreter'
	| 'pick_date'
	| 'comments'
	| 'color'
	| 'unknown'

// Interpretation types for marker sets
export type InterpretationType = 'formation' | 'sequence' | 'zone' | 'horizon' | 'other'

// Quality/confidence indicators
export type MarkerQuality = 'confirmed' | 'uncertain' | 'projected' | 'absent'

// Response from parse_markers_for_ingest command
export interface ParseMarkerResponse {
	file_id: string
	file_name: string
	columns: MarkerColumnInfo[]
	row_count: number
	delimiter: string
	has_header: boolean
	is_multi_well: boolean
	well_names: string[]
	warnings: string[]
	has_required_columns: boolean
	missing_required: string[]
}

// Information about a column in the marker CSV
export interface MarkerColumnInfo {
	index: number
	header: string
	detected_type: MarkerColumnType
	confidence: number
	unit: string | null
	sample_values: string[]
	min_value: number | null
	max_value: number | null
	mean_value: number | null
}

// Well mapping entry for ingestion
export interface WellMappingEntry {
	well_name: string
	well_id: string | null
	create_new: boolean
}

// Configuration for a marker column (user overrides)
export interface MarkerColumnConfig {
	index: number
	column_type: MarkerColumnType | null
	unit: string | null
}

// Confidence level for marker interpretations (OSDU pattern)
export type ConfidenceLevel = 'High' | 'Medium' | 'Low'

// Request for ingesting marker CSV files
export interface IngestMarkerRequest {
	file_paths: string[]
	well_mappings: WellMappingEntry[]
	column_configs: MarkerColumnConfig[]
	set_name: string | null
	interpretation_type: InterpretationType | null

	// OSDU WellboreMarkerSet fields
	interpreter: string | null
	interpretation_date: string | null // ISO 8601
	confidence_level: ConfidenceLevel | null

	depth_unit: string | null
	depth_reference: string | null
	auto_create_wells: boolean
	allow_unmapped_wells: boolean
}

// Response from ingest_marker_files command
export interface IngestMarkerResponse {
	success: boolean
	marker_set_ids: string[]
	marker_count: number
	well_count: number
	wells_created: number
	well_results: WellMarkerResult[]
	error: string | null
}

export interface WellMarkerResult {
	well_name: string
	well_id: string | null
	created: boolean
	marker_count: number
}

// Summary of a marker set
export interface MarkerSetSummary {
	id: string
	source_filename: string
	name: string | null
	interpretation_type: string | null
	marker_count: number | null
	min_depth: number | null
	max_depth: number | null
	depth_unit: string
	imported_at: string
}

// Marker data for display
export interface MarkerData {
	id: string
	name: string
	measured_depth: number
	tvd: number | null
	marker_type: string | null
	quality: string | null
	comments: string | null
}

// Helper to get human-readable column name
export function getMarkerColumnDisplayName(type: MarkerColumnType): string {
	switch (type) {
		case 'well_name':
			return 'Well Name'
		case 'well_uwi':
			return 'Well UWI/API'
		case 'marker_name':
			return 'Marker Name'
		case 'measured_depth':
			return 'Measured Depth'
		case 'true_vertical_depth':
			return 'True Vertical Depth'
		case 'true_vertical_depth_ss':
			return 'TVD Sub-Sea'
		case 'marker_type':
			return 'Marker Type'
		case 'thickness':
			return 'Thickness'
		case 'quality':
			return 'Quality'
		case 'interpreter':
			return 'Interpreter'
		case 'pick_date':
			return 'Pick Date'
		case 'comments':
			return 'Comments'
		case 'color':
			return 'Color'
		case 'unknown':
			return 'Unknown'
	}
}

// Helper to get interpretation type display name
export function getInterpretationTypeDisplayName(type: InterpretationType): string {
	switch (type) {
		case 'formation':
			return 'Formation Tops'
		case 'sequence':
			return 'Sequence Boundaries'
		case 'zone':
			return 'Zone Boundaries'
		case 'horizon':
			return 'Seismic Horizons'
		case 'other':
			return 'Other'
	}
}

// Required column types for basic ingestion
export const REQUIRED_MARKER_COLUMNS: MarkerColumnType[] = ['marker_name', 'measured_depth']

// Optional but useful column types
export const OPTIONAL_MARKER_COLUMNS: MarkerColumnType[] = [
	'well_name',
	'true_vertical_depth',
	'marker_type',
	'quality',
	'comments'
]
