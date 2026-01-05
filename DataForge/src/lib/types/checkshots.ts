/**
 * Types for checkshot (time-depth) data ingestion
 *
 * Corresponds to types in src-tauri/src/checkshot_commands.rs
 * Follows OSDU patterns for checkshot stations with MD, TVD, TWT columns.
 */

// Standard checkshot column types (matches CheckshotColumnType in Rust with snake_case serialization)
export type CheckshotColumnType =
	| 'measured_depth'
	| 'true_vertical_depth'
	| 'two_way_time'
	| 'velocity'
	| 'one_way_time'
	| 'quality'
	| 'unknown'

// Survey types for checkshots
export type CheckshotSurveyType = 'checkshot' | 'vsp' | 'sonic_log' | 'other'

// Response from parse_checkshot_for_ingest command
export interface ParseCheckResponse {
	file_id: string
	file_name: string
	columns: CheckColumnInfo[]
	row_count: number
	delimiter: string
	warnings: string[]
	has_required_columns: boolean
	missing_required: string[]
}

// Information about a column in the checkshot CSV
export interface CheckColumnInfo {
	index: number
	header: string
	detected_type: CheckshotColumnType
	confidence: number
	unit: string | null
	is_required: boolean
	is_calculated: boolean
	row_count: number
	null_count: number
	min_value: number | null
	max_value: number | null
	mean_value: number | null
}

// Configuration for a checkshot column (user overrides)
export interface CheckColumnConfig {
	index: number
	column_type: CheckshotColumnType
}

// Request for ingesting checkshot CSV files
export interface IngestCheckRequest {
	file_paths: string[]
	well_id: string | null
	well_name: string | null
	create_well: boolean
	column_configs: CheckColumnConfig[]
	survey_type: CheckshotSurveyType | null
	survey_company: string | null
	depth_unit: string
	time_unit: string
	datum: string | null
	calculate_velocity: boolean
}

// Response from ingest_checkshot_files command
export interface IngestCheckResponse {
	success: boolean
	checkshot_run_id: string | null
	well_id: string | null
	well_created: boolean
	station_count: number
	column_count: number
	min_md: number
	max_md: number
	min_tvd: number
	max_tvd: number
	min_twt: number
	max_twt: number
	error: string | null
}

// Summary of a checkshot run
export interface CheckshotSummary {
	id: string
	name: string | null
	survey_type: CheckshotSurveyType | null
	station_count: number | null
	min_md: number | null
	max_md: number | null
	min_twt: number | null
	max_twt: number | null
	depth_unit: string
	time_unit: string
	imported_at: string
}

// Full checkshot data for a run
export interface CheckshotData {
	id: string
	name: string | null
	survey_type: CheckshotSurveyType | null
	station_count: number
	depth_unit: string
	time_unit: string
	columns: CheckshotColumnData[]
}

// Column data for a checkshot run
export interface CheckshotColumnData {
	column_type: CheckshotColumnType
	column_name: string
	unit: string | null
	values: number[]
}

// Required columns for checkshot ingestion
export const REQUIRED_CHECKSHOT_COLUMNS: CheckshotColumnType[] = ['measured_depth', 'true_vertical_depth', 'two_way_time']

// Calculated column types
export const CALCULATED_CHECKSHOT_COLUMNS: CheckshotColumnType[] = ['velocity']

// Helper to get human-readable column name
export function getCheckColumnDisplayName(type: CheckshotColumnType): string {
	switch (type) {
		case 'measured_depth':
			return 'Measured Depth'
		case 'true_vertical_depth':
			return 'True Vertical Depth'
		case 'two_way_time':
			return 'Two-Way Time'
		case 'velocity':
			return 'Interval Velocity'
		case 'one_way_time':
			return 'One-Way Time'
		case 'quality':
			return 'Quality'
		case 'unknown':
			return 'Unknown'
	}
}

// Helper to get survey type display name
export function getCheckSurveyTypeDisplayName(type: CheckshotSurveyType): string {
	switch (type) {
		case 'checkshot':
			return 'Checkshot Survey'
		case 'vsp':
			return 'VSP (Vertical Seismic Profile)'
		case 'sonic_log':
			return 'Sonic Log'
		case 'other':
			return 'Other'
	}
}

// Survey type options for dropdowns
export const CHECKSHOT_SURVEY_TYPE_OPTIONS: { value: CheckshotSurveyType; label: string }[] = [
	{ value: 'checkshot', label: 'Checkshot Survey' },
	{ value: 'vsp', label: 'VSP' },
	{ value: 'sonic_log', label: 'Sonic Log' },
	{ value: 'other', label: 'Other' }
]

// Depth unit options
export const DEPTH_UNIT_OPTIONS = [
	{ value: 'ft', label: 'Feet (ft)' },
	{ value: 'm', label: 'Meters (m)' }
]

// Time unit options
export const TIME_UNIT_OPTIONS = [
	{ value: 's', label: 'Seconds (s)' },
	{ value: 'ms', label: 'Milliseconds (ms)' }
]

// Datum options
export const DATUM_OPTIONS = [
	{ value: 'kb', label: 'Kelly Bushing (KB)' },
	{ value: 'msl', label: 'Mean Sea Level (MSL)' },
	{ value: 'ground_level', label: 'Ground Level' }
]
