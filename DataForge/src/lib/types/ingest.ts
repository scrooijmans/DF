/**
 * Types for data ingestion
 *
 * Corresponds to types in src-tauri/src/ingest_commands.rs
 */

// Response from parse_las_for_ingest command
export interface ParseLasResponse {
	file_id: string
	file_name: string
	version: string
	well_metadata: WellMetadataResponse
	curves: CurveInfo[]
	row_count: number
}

// Well metadata extracted from LAS file
export interface WellMetadataResponse {
	well_name: string
	uwi: string | null
	company: string | null
	field: string | null
	location: string | null
	start_depth: number
	stop_depth: number
	step: number
	depth_unit: string
	null_value: number
}

// Information about a curve in the LAS file
export interface CurveInfo {
	mnemonic: string
	unit: string
	description: string
	detected_type: string
	confidence: number
	include: boolean
	row_count: number
	min_value: number | null
	max_value: number | null
}

// Request for ingesting LAS files
export interface IngestLasRequest {
	file_paths: string[]
	target_well_id: string | null
	new_well_name: string | null
	curve_configs: CurveConfig[]
}

// Standard curve types (matches MainCurveType in Rust)
export type MainCurveType =
	| 'GR'
	| 'RT'
	| 'RHOB'
	| 'NPHI'
	| 'CALI'
	| 'DT'
	| 'SP'
	| 'PE'
	| 'DEPTH'
	| 'OTHER'

// Configuration for a single curve
export interface CurveConfig {
	mnemonic: string
	mapped_name: string | null
	mapped_type: MainCurveType | null
	target_unit: string | null
	include: boolean
}

// Response from ingest_las_files command
export interface IngestResponse {
	success: boolean
	files_processed: number
	curves_imported: number
	data_points: number
	well_id: string | null
	parquet_paths: string[]
	errors: string[]
}

// Supported file type info
export interface FileTypeInfo {
	id: string
	name: string
	extensions: string[]
	description: string
	available: boolean
}

// Well summary for selection in ingest wizard
export interface WellSummary {
	id: string
	name: string
	uwi: string | null
	field: string | null
	curve_count: number
}

// Recent activity item (well or curve)
export interface RecentActivityItem {
	id: string
	name: string
	item_type: 'well' | 'curve'
	parent_name: string | null // Well name for curves
	parent_id: string | null // Well ID for curves
	size_bytes: number | null
	row_count: number | null
	updated_at: string
	created_at: string
}

// Workspace statistics for home page
export interface WorkspaceStats {
	well_count: number
	curve_count: number
	total_data_points: number
	total_size_bytes: number
}

// Well details for well detail page
export interface WellDetails {
	id: string
	name: string
	uwi: string | null
	field: string | null
	company: string | null
	location: string | null
	x: number | null
	y: number | null
	depth_unit: string
	depth_step: number
	depth_origin: number
	min_depth: number | null
	max_depth: number | null
	created_at: string
	updated_at: string
	curve_count: number
	total_data_points: number
	total_size_bytes: number
}
