/**
 * Types for trajectory/survey data ingestion
 *
 * Corresponds to types in src-tauri/src/trajectory_commands.rs
 * Follows OSDU patterns for trajectory stations with input (MD, INC, AZI) and
 * calculated (TVD, NS, EW, DLS) columns.
 */

// Standard trajectory column types (matches TrajectoryColumnType in Rust with snake_case serialization)
export type TrajectoryColumnType =
	| 'measured_depth'
	| 'inclination'
	| 'azimuth'
	| 'true_vertical_depth'
	| 'north_south'
	| 'east_west'
	| 'dogleg_severity'
	| 'delta_x'
	| 'delta_y'
	| 'delta_z'
	| 'unknown'

// Survey types (matches SurveyType in Rust)
export type SurveyType = 'definitive' | 'mwd' | 'gyro' | 'preliminary' | 'composite'

// Response from parse_trajectory_for_ingest command
export interface ParseTrajectoryResponse {
	file_id: string
	file_name: string
	columns: TrajectoryColumnInfo[]
	row_count: number
	delimiter: string
	warnings: string[]
	has_required_columns: boolean
	missing_required: string[]
}

// Information about a column in the trajectory CSV
export interface TrajectoryColumnInfo {
	index: number
	header: string
	detected_type: TrajectoryColumnType
	confidence: number
	unit: string | null
	is_input: boolean
	is_calculated: boolean
	row_count: number
	null_count: number
	min_value: number | null
	max_value: number | null
	mean_value: number | null
}

// Configuration for a trajectory column (user overrides)
export interface TrajectoryColumnConfig {
	index: number
	column_type: TrajectoryColumnType | null
	unit: string | null
}

// Request for ingesting trajectory CSV files
export interface IngestTrajectoryRequest {
	file_paths: string[]
	target_well_id: string | null
	new_well_name: string | null
	column_configs: TrajectoryColumnConfig[]
	survey_type: SurveyType | null
	survey_company: string | null
	magnetic_declination: number | null
	grid_convergence: number | null
	md_unit: string | null
	angle_unit: string | null
}

// Response from ingest_trajectory_files command
export interface IngestTrajectoryResponse {
	success: boolean
	survey_run_id: string | null
	well_id: string | null
	well_created: boolean
	station_count: number
	column_count: number
	top_md: number
	bottom_md: number
	max_inclination: number
	max_dls: number
	error: string | null
}

// Summary of a trajectory/survey run
export interface TrajectorySummary {
	id: string
	source_filename: string
	survey_type: SurveyType | null
	calculation_method: string | null
	top_md: number | null
	bottom_md: number | null
	station_count: number | null
	md_unit: string
	imported_at: string
}

// Trajectory station data for visualization
export interface TrajectoryStationData {
	md: number[]
	inclination: number[]
	azimuth: number[]
	tvd: number[]
	ns: number[]
	ew: number[]
	dls: number[]
}

// Input column types (required for trajectory calculations)
export const INPUT_COLUMN_TYPES: TrajectoryColumnType[] = ['measured_depth', 'inclination', 'azimuth']

// Calculated column types (derived from minimum curvature)
export const CALCULATED_COLUMN_TYPES: TrajectoryColumnType[] = ['true_vertical_depth', 'north_south', 'east_west', 'dogleg_severity']

// Helper to get human-readable column name
export function getColumnDisplayName(type: TrajectoryColumnType): string {
	switch (type) {
		case 'measured_depth':
			return 'Measured Depth'
		case 'inclination':
			return 'Inclination'
		case 'azimuth':
			return 'Azimuth'
		case 'true_vertical_depth':
			return 'True Vertical Depth'
		case 'north_south':
			return 'North-South'
		case 'east_west':
			return 'East-West'
		case 'dogleg_severity':
			return 'Dogleg Severity'
		case 'delta_x':
			return 'Delta X'
		case 'delta_y':
			return 'Delta Y'
		case 'delta_z':
			return 'Delta Z'
		case 'unknown':
			return 'Unknown'
	}
}

// Helper to get survey type display name
export function getSurveyTypeDisplayName(type: SurveyType): string {
	switch (type) {
		case 'definitive':
			return 'Definitive'
		case 'mwd':
			return 'MWD (Measurement While Drilling)'
		case 'gyro':
			return 'Gyroscopic'
		case 'preliminary':
			return 'Preliminary'
		case 'composite':
			return 'Composite'
	}
}
