/**
 * Ingest Store - Manages state for the multi-step data ingestion wizard
 *
 * Follows patterns from Databricks, CKAN, and Open Data Cube for file ingestion.
 * Supports multiple data source types with LAS files as the primary initial use case.
 */

import { invoke } from '@tauri-apps/api/core'
import type {
	ParseLasResponse,
	IngestLasRequest,
	IngestResponse,
	CurveConfig,
	MainCurveType
} from '$lib/types/ingest'
import type {
	ParseTrajectoryResponse,
	IngestTrajectoryRequest,
	IngestTrajectoryResponse,
	TrajectoryColumnInfo,
	TrajectoryColumnConfig,
	SurveyType
} from '$lib/types/trajectory'
import type {
	ParseMarkerResponse,
	IngestMarkerRequest,
	IngestMarkerResponse,
	MarkerColumnInfo,
	MarkerColumnConfig,
	WellMappingEntry,
	InterpretationType,
	MarkerColumnType
} from '$lib/types/markers'
import type {
	ParseSurfaceResponse,
	IngestSurfaceRequest,
	IngestSurfaceResponse,
	SurfaceColumnInfo,
	SurfaceColumnConfig,
	SurfaceColumnType,
	SurfaceType,
	ZPositiveDirection,
	SpatialExtent
} from '$lib/types/surfaces'
import type {
	ParseCheckResponse,
	IngestCheckRequest,
	IngestCheckResponse,
	CheckColumnInfo,
	CheckColumnConfig,
	CheckshotColumnType,
	CheckshotSurveyType
} from '$lib/types/checkshots'

// Data source types supported by the ingest wizard
export type DataSourceType =
	| 'las'
	| 'trajectory_csv'
	| 'markers_csv'
	| 'surface_csv'
	| 'checkshot_csv'
	| 'csv'
	| 'parquet'
	| 'excel'

// Ingest wizard steps
export type IngestStep = 'source' | 'upload' | 'mapping' | 'confirm'

// File status during upload/processing
export type FileStatus = 'pending' | 'uploading' | 'parsing' | 'ready' | 'error'

// Represents an uploaded file in the wizard
export interface IngestFile {
	id: string
	name: string
	size: number
	type: string
	status: FileStatus
	progress: number
	error?: string
	// LAS-specific metadata extracted during parsing
	lasMetadata?: LasFileMetadata
	// Trajectory-specific metadata extracted during parsing
	trajectoryMetadata?: TrajectoryFileMetadata
	// Marker-specific metadata extracted during parsing
	markerMetadata?: MarkerFileMetadata
	// Surface-specific metadata extracted during parsing
	surfaceMetadata?: SurfaceFileMetadata
	// Checkshot-specific metadata extracted during parsing
	checkMetadata?: CheckshotFileMetadata
	// Path to temporary file (after upload to Tauri backend)
	tempPath?: string
}

// LAS file metadata extracted during parsing
export interface LasFileMetadata {
	version: string
	wrap: boolean
	wellName: string
	uwi?: string
	company?: string
	field?: string
	country?: string
	startDepth: number
	stopDepth: number
	step: number
	depthUnit: string
	curves: LasCurveInfo[]
}

// Information about a curve in a LAS file
export interface LasCurveInfo {
	mnemonic: string
	unit: string
	description: string
	// Detection info from backend
	detectedType: MainCurveType
	confidence: number
	// Mapping configuration (user overrides)
	mappedName?: string
	mappedType?: MainCurveType
	mappedUnit?: string
	include: boolean
}

// Trajectory file metadata extracted during parsing
export interface TrajectoryFileMetadata {
	fileId: string
	delimiter: string
	rowCount: number
	columns: TrajectoryColumnMappingInfo[]
	hasRequiredColumns: boolean
	missingRequired: string[]
	warnings: string[]
	// Survey metadata (user inputs)
	surveyType?: SurveyType
	surveyCompany?: string
	magneticDeclination?: number
	gridConvergence?: number
	mdUnit: string
	angleUnit: string
}

// Information about a trajectory column with mapping
export interface TrajectoryColumnMappingInfo {
	index: number
	header: string
	detectedType: string
	confidence: number
	unit: string | null
	isInput: boolean
	isCalculated: boolean
	// Statistics
	rowCount: number
	nullCount: number
	minValue: number | null
	maxValue: number | null
	meanValue: number | null
	// User overrides
	mappedType?: string
	mappedUnit?: string
	include: boolean
}

// Marker file metadata extracted during parsing
export interface MarkerFileMetadata {
	fileId: string
	fileName: string
	delimiter: string
	rowCount: number
	hasHeader: boolean
	isMultiWell: boolean
	wellNames: string[]
	columns: MarkerColumnMappingInfo[]
	hasRequiredColumns: boolean
	missingRequired: string[]
	warnings: string[]
	// User inputs for marker set
	setName?: string
	interpretationType?: InterpretationType
	interpreter?: string
	depthUnit: string
	depthReference?: string
	autoCreateWells: boolean
	allowUnmappedWells: boolean
	wellMappings: WellMappingEntry[]
}

// Information about a marker column with mapping
export interface MarkerColumnMappingInfo {
	index: number
	header: string
	detectedType: MarkerColumnType
	confidence: number
	unit: string | null
	// Statistics
	sampleValues: string[]
	minValue: number | null
	maxValue: number | null
	meanValue: number | null
	// User overrides
	mappedType?: MarkerColumnType
	mappedUnit?: string
	include: boolean
}

// Surface file metadata extracted during parsing
export interface SurfaceFileMetadata {
	fileId: string
	fileName: string
	delimiter: string
	rowCount: number
	isMultiSurface: boolean
	surfaceNames: string[]
	extent: SpatialExtent
	columns: SurfaceColumnMappingInfo[]
	hasRequiredColumns: boolean
	missingRequired: string[]
	warnings: string[]
	// User inputs for surface
	surfaceName?: string
	surfaceType?: SurfaceType
	crs?: string
	xyUnit: string
	zUnit: string
	zPositiveDirection: ZPositiveDirection
}

// Information about a surface column with mapping
export interface SurfaceColumnMappingInfo {
	index: number
	header: string
	detectedType: SurfaceColumnType
	confidence: number
	unit: string | null
	// User overrides
	mappedType?: SurfaceColumnType
	include: boolean
}

// Checkshot file metadata extracted during parsing
export interface CheckshotFileMetadata {
	fileId: string
	fileName: string
	delimiter: string
	rowCount: number
	columns: CheckshotColumnMappingInfo[]
	hasRequiredColumns: boolean
	missingRequired: string[]
	warnings: string[]
	// User inputs for checkshot
	surveyType?: CheckshotSurveyType
	surveyCompany?: string
	depthUnit: string
	timeUnit: string
	datum?: string
	calculateVelocity: boolean
}

// Information about a checkshot column with mapping
export interface CheckshotColumnMappingInfo {
	index: number
	header: string
	detectedType: CheckshotColumnType
	confidence: number
	unit: string | null
	isRequired: boolean
	isCalculated: boolean
	// Statistics
	rowCount: number
	nullCount: number
	minValue: number | null
	maxValue: number | null
	meanValue: number | null
	// User overrides
	mappedType?: CheckshotColumnType
	include: boolean
}

// Column mapping for generic file types (CSV, Excel)
export interface ColumnMapping {
	sourceColumn: string
	targetColumn: string
	targetType: 'string' | 'float' | 'int' | 'datetime'
	transform?: string
}

// Ingest configuration for submission
export interface IngestConfig {
	sourceType: DataSourceType
	files: IngestFile[]
	// LAS-specific config
	lasCurveMapping?: LasCurveInfo[]
	// Generic column mapping
	columnMappings?: ColumnMapping[]
	// Target well (for curve data)
	targetWellId?: string
	createNewWell?: boolean
	newWellName?: string
}

// Wizard state
interface IngestState {
	currentStep: IngestStep
	sourceType: DataSourceType | null
	files: IngestFile[]
	config: Partial<IngestConfig>
	isSubmitting: boolean
	submitError: string | null
}

// Create the ingest store
function createIngestStore() {
	// Initialize state
	let state = $state<IngestState>({
		currentStep: 'source',
		sourceType: null,
		files: [],
		config: {},
		isSubmitting: false,
		submitError: null
	})

	// Step order for navigation
	const stepOrder: IngestStep[] = ['source', 'upload', 'mapping', 'confirm']

	// Get step index
	function getStepIndex(step: IngestStep): number {
		return stepOrder.indexOf(step)
	}

	// Check if step is accessible
	function canAccessStep(step: IngestStep): boolean {
		const currentIndex = getStepIndex(state.currentStep)
		const targetIndex = getStepIndex(step)

		// Can always go back
		if (targetIndex < currentIndex) return true

		// Can only go forward one step at a time if requirements are met
		if (targetIndex === currentIndex + 1) {
			switch (state.currentStep) {
				case 'source':
					return state.sourceType !== null
				case 'upload':
					return state.files.length > 0 && state.files.every((f) => f.status === 'ready')
				case 'mapping':
					return true // Mapping is optional, can proceed
				default:
					return false
			}
		}

		return targetIndex <= currentIndex
	}

	// Actions
	function setSourceType(type: DataSourceType) {
		state.sourceType = type
		state.config.sourceType = type
	}

	function goToStep(step: IngestStep) {
		if (canAccessStep(step)) {
			state.currentStep = step
		}
	}

	function nextStep() {
		const currentIndex = getStepIndex(state.currentStep)
		if (currentIndex < stepOrder.length - 1) {
			const nextStepName = stepOrder[currentIndex + 1]
			if (canAccessStep(nextStepName)) {
				state.currentStep = nextStepName
			}
		}
	}

	function previousStep() {
		const currentIndex = getStepIndex(state.currentStep)
		if (currentIndex > 0) {
			state.currentStep = stepOrder[currentIndex - 1]
		}
	}

	function addFile(file: IngestFile) {
		state.files.push(file)
	}

	function updateFile(id: string, updates: Partial<IngestFile>) {
		const index = state.files.findIndex((f) => f.id === id)
		if (index !== -1) {
			state.files[index] = { ...state.files[index], ...updates }
		}
	}

	function removeFile(id: string) {
		state.files = state.files.filter((f) => f.id !== id)
	}

	function clearFiles() {
		state.files = []
	}

	function updateCurveMapping(fileId: string, curveIndex: number, updates: Partial<LasCurveInfo>) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.lasMetadata?.curves[curveIndex]) {
			file.lasMetadata.curves[curveIndex] = {
				...file.lasMetadata.curves[curveIndex],
				...updates
			}
		}
	}

	function setTargetWell(wellId: string | null, createNew: boolean, newName?: string) {
		state.config.targetWellId = wellId ?? undefined
		state.config.createNewWell = createNew
		state.config.newWellName = newName
	}

	function reset() {
		state.currentStep = 'source'
		state.sourceType = null
		state.files = []
		state.config = {}
		state.isSubmitting = false
		state.submitError = null
	}

	/**
	 * Parse a LAS file using the Tauri backend
	 */
	async function parseLasFile(filePath: string): Promise<ParseLasResponse> {
		return invoke<ParseLasResponse>('parse_las_for_ingest', { filePath })
	}

	/**
	 * Parse a trajectory CSV file using the Tauri backend
	 */
	async function parseTrajectoryFile(filePath: string): Promise<ParseTrajectoryResponse> {
		return invoke<ParseTrajectoryResponse>('parse_trajectory_for_ingest', { filePath })
	}

	/**
	 * Parse a marker CSV file using the Tauri backend
	 */
	async function parseMarkerFile(filePath: string): Promise<ParseMarkerResponse> {
		return invoke<ParseMarkerResponse>('parse_markers_for_ingest', { filePath })
	}

	/**
	 * Parse a surface CSV file using the Tauri backend
	 */
	async function parseSurfaceFile(filePath: string): Promise<ParseSurfaceResponse> {
		return invoke<ParseSurfaceResponse>('parse_surface_for_ingest', { filePath })
	}

	/**
	 * Parse a checkshot CSV file using the Tauri backend
	 */
	async function parseCheckFile(filePath: string): Promise<ParseCheckResponse> {
		return invoke<ParseCheckResponse>('parse_checkshot_for_ingest', { filePath })
	}

	/**
	 * Process a file - uploads and parses it via Tauri
	 */
	async function processFile(ingestFile: IngestFile, filePath: string): Promise<void> {
		updateFile(ingestFile.id, { status: 'uploading', progress: 50 })

		try {
			// For LAS files, parse via Tauri
			if (state.sourceType === 'las') {
				updateFile(ingestFile.id, { status: 'parsing', progress: 75 })

				const result = await parseLasFile(filePath)

				// Convert ParseLasResponse to LasFileMetadata
				const lasMetadata: LasFileMetadata = {
					version: result.version,
					wrap: false,
					wellName: result.well_metadata.well_name,
					uwi: result.well_metadata.uwi ?? undefined,
					company: result.well_metadata.company ?? undefined,
					field: result.well_metadata.field ?? undefined,
					startDepth: result.well_metadata.start_depth,
					stopDepth: result.well_metadata.stop_depth,
					step: result.well_metadata.step,
					depthUnit: result.well_metadata.depth_unit,
					curves: result.curves.map((c) => ({
						mnemonic: c.mnemonic,
						unit: c.unit,
						description: c.description,
						detectedType: c.detected_type as MainCurveType,
						confidence: c.confidence,
						include: c.include
					}))
				}

				updateFile(ingestFile.id, {
					status: 'ready',
					progress: 100,
					lasMetadata,
					tempPath: filePath
				})
			} else if (state.sourceType === 'trajectory_csv') {
				// For trajectory CSV files, parse via Tauri
				updateFile(ingestFile.id, { status: 'parsing', progress: 75 })

				const result = await parseTrajectoryFile(filePath)

				// Convert ParseTrajectoryResponse to TrajectoryFileMetadata
				const trajectoryMetadata: TrajectoryFileMetadata = {
					fileId: result.file_id,
					delimiter: result.delimiter,
					rowCount: result.row_count,
					hasRequiredColumns: result.has_required_columns,
					missingRequired: result.missing_required,
					warnings: result.warnings,
					mdUnit: 'ft', // Default units
					angleUnit: 'deg',
					columns: result.columns.map((c) => ({
						index: c.index,
						header: c.header,
						detectedType: c.detected_type,
						confidence: c.confidence,
						unit: c.unit,
						isInput: c.is_input,
						isCalculated: c.is_calculated,
						rowCount: c.row_count,
						nullCount: c.null_count,
						minValue: c.min_value,
						maxValue: c.max_value,
						meanValue: c.mean_value,
						include: c.is_input // Auto-include input columns (MD, INC, AZI)
					}))
				}

				updateFile(ingestFile.id, {
					status: 'ready',
					progress: 100,
					trajectoryMetadata,
					tempPath: filePath
				})
			} else if (state.sourceType === 'markers_csv') {
				// For marker CSV files, parse via Tauri
				updateFile(ingestFile.id, { status: 'parsing', progress: 75 })

				const result = await parseMarkerFile(filePath)

				// Convert ParseMarkerResponse to MarkerFileMetadata
				const markerMetadata: MarkerFileMetadata = {
					fileId: result.file_id,
					fileName: result.file_name,
					delimiter: result.delimiter,
					rowCount: result.row_count,
					hasHeader: result.has_header,
					isMultiWell: result.is_multi_well,
					wellNames: result.well_names,
					hasRequiredColumns: result.has_required_columns,
					missingRequired: result.missing_required,
					warnings: result.warnings,
					depthUnit: 'ft', // Default unit
					autoCreateWells: false, // Default: don't auto-create wells
					allowUnmappedWells: false, // Default: require well mapping
					wellMappings: result.well_names.map((name) => ({
						well_name: name,
						well_id: null, // Will be matched in mapping step
						create_new: false
					})),
					columns: result.columns.map((c) => ({
						index: c.index,
						header: c.header,
						detectedType: c.detected_type,
						confidence: c.confidence,
						unit: c.unit,
						sampleValues: c.sample_values,
						minValue: c.min_value,
						maxValue: c.max_value,
						meanValue: c.mean_value,
						include: c.detected_type !== 'unknown' // Auto-include detected columns
					}))
				}

				updateFile(ingestFile.id, {
					status: 'ready',
					progress: 100,
					markerMetadata,
					tempPath: filePath
				})
			} else if (state.sourceType === 'surface_csv') {
				// For surface CSV files, parse via Tauri
				updateFile(ingestFile.id, { status: 'parsing', progress: 75 })

				const result = await parseSurfaceFile(filePath)

				// Convert ParseSurfaceResponse to SurfaceFileMetadata
				const surfaceMetadata: SurfaceFileMetadata = {
					fileId: result.file_id,
					fileName: result.file_name,
					delimiter: result.delimiter,
					rowCount: result.row_count,
					isMultiSurface: result.is_multi_surface,
					surfaceNames: result.surface_names,
					extent: result.extent,
					hasRequiredColumns: result.has_required_columns,
					missingRequired: result.missing_required,
					warnings: result.warnings,
					xyUnit: 'm', // Default units
					zUnit: 'm',
					zPositiveDirection: 'down',
					columns: result.columns.map((c) => ({
						index: c.index,
						header: c.header,
						detectedType: c.detected_type as SurfaceColumnType,
						confidence: c.confidence,
						unit: c.unit,
						include: c.detected_type !== 'unknown' // Auto-include detected columns
					}))
				}

				updateFile(ingestFile.id, {
					status: 'ready',
					progress: 100,
					surfaceMetadata,
					tempPath: filePath
				})
			} else if (state.sourceType === 'checkshot_csv') {
				// For checkshot CSV files, parse via Tauri
				updateFile(ingestFile.id, { status: 'parsing', progress: 75 })

				const result = await parseCheckFile(filePath)

				// Convert ParseCheckResponse to CheckshotFileMetadata
				const checkMetadata: CheckshotFileMetadata = {
					fileId: result.file_id,
					fileName: result.file_name,
					delimiter: result.delimiter,
					rowCount: result.row_count,
					hasRequiredColumns: result.has_required_columns,
					missingRequired: result.missing_required,
					warnings: result.warnings,
					depthUnit: 'ft', // Default units
					timeUnit: 's',
					calculateVelocity: false,
					columns: result.columns.map((c) => ({
						index: c.index,
						header: c.header,
						detectedType: c.detected_type as CheckshotColumnType,
						confidence: c.confidence,
						unit: c.unit,
						isRequired: c.is_required,
						isCalculated: c.is_calculated,
						rowCount: c.row_count,
						nullCount: c.null_count,
						minValue: c.min_value,
						maxValue: c.max_value,
						meanValue: c.mean_value,
						include: c.is_required // Auto-include required columns (MD, TVD, TWT)
					}))
				}

				updateFile(ingestFile.id, {
					status: 'ready',
					progress: 100,
					checkMetadata,
					tempPath: filePath
				})
			} else {
				// For other file types, just mark as ready for now
				updateFile(ingestFile.id, { status: 'ready', progress: 100, tempPath: filePath })
			}
		} catch (e) {
			updateFile(ingestFile.id, {
				status: 'error',
				error: e instanceof Error ? e.message : String(e)
			})
			throw e
		}
	}

	async function submit(): Promise<boolean> {
		state.isSubmitting = true
		state.submitError = null

		try {
			if (state.sourceType === 'trajectory_csv') {
				// Submit trajectory CSV files
				return await submitTrajectory()
			} else if (state.sourceType === 'markers_csv') {
				// Submit marker CSV files
				return await submitMarkers()
			} else if (state.sourceType === 'surface_csv') {
				// Submit surface CSV files
				return await submitSurfaces()
			} else if (state.sourceType === 'checkshot_csv') {
				// Submit checkshot CSV files
				return await submitCheckshots()
			} else {
				// Submit LAS files (default)
				return await submitLas()
			}
		} catch (e) {
			state.submitError = e instanceof Error ? e.message : String(e)
			return false
		} finally {
			state.isSubmitting = false
		}
	}

	async function submitLas(): Promise<boolean> {
		// Build curve configs from file metadata
		const curveConfigs: CurveConfig[] = state.files.flatMap((f) =>
			(f.lasMetadata?.curves ?? []).map((c) => ({
				mnemonic: c.mnemonic,
				mapped_name: c.mappedName ?? null,
				mapped_type: c.mappedType ?? c.detectedType ?? null,
				target_unit: c.mappedUnit ?? null,
				include: c.include
			}))
		)

		// Build request
		const request: IngestLasRequest = {
			file_paths: state.files.map((f) => f.tempPath!).filter(Boolean),
			target_well_id: state.config.targetWellId ?? null,
			new_well_name: state.config.newWellName ?? null,
			curve_configs: curveConfigs
		}

		console.log('Submitting LAS ingest request:', request)

		// Call Tauri command
		const response = await invoke<IngestResponse>('ingest_las_files', { request })

		if (!response.success) {
			throw new Error(response.errors.join('; ') || 'Ingestion failed')
		}

		console.log('LAS ingestion response:', response)
		return true
	}

	async function submitTrajectory(): Promise<boolean> {
		// Get the first file's trajectory metadata for survey settings
		const firstFile = state.files[0]
		const trajectoryMeta = firstFile?.trajectoryMetadata

		// Build column configs from file metadata
		const columnConfigs: TrajectoryColumnConfig[] = state.files.flatMap((f) =>
			(f.trajectoryMetadata?.columns ?? [])
				.filter((c) => c.include)
				.map((c) => ({
					index: c.index,
					column_type: (c.mappedType ?? c.detectedType) as TrajectoryColumnConfig['column_type'],
					unit: c.mappedUnit ?? c.unit
				}))
		)

		// Build request
		const request: IngestTrajectoryRequest = {
			file_paths: state.files.map((f) => f.tempPath!).filter(Boolean),
			target_well_id: state.config.targetWellId ?? null,
			new_well_name: state.config.newWellName ?? null,
			column_configs: columnConfigs,
			survey_type: (trajectoryMeta?.surveyType as SurveyType) ?? null,
			survey_company: trajectoryMeta?.surveyCompany ?? null,
			magnetic_declination: trajectoryMeta?.magneticDeclination ?? null,
			grid_convergence: trajectoryMeta?.gridConvergence ?? null,
			md_unit: trajectoryMeta?.mdUnit ?? null,
			angle_unit: trajectoryMeta?.angleUnit ?? null
		}

		console.log('Submitting trajectory ingest request:', request)

		// Call Tauri command
		const response = await invoke<IngestTrajectoryResponse>('ingest_trajectory_files', { request })

		if (!response.success) {
			throw new Error(response.error ?? 'Trajectory ingestion failed')
		}

		console.log('Trajectory ingestion response:', response)
		return true
	}

	async function submitMarkers(): Promise<boolean> {
		// Get the first file's marker metadata for set settings
		const firstFile = state.files[0]
		const markerMeta = firstFile?.markerMetadata

		// Build column configs from file metadata
		const columnConfigs: MarkerColumnConfig[] = state.files.flatMap((f) =>
			(f.markerMetadata?.columns ?? [])
				.filter((c) => c.include)
				.map((c) => ({
					index: c.index,
					column_type: (c.mappedType ?? c.detectedType) as MarkerColumnType,
					unit: c.mappedUnit ?? c.unit
				}))
		)

		// Build request
		const request: IngestMarkerRequest = {
			file_paths: state.files.map((f) => f.tempPath!).filter(Boolean),
			well_mappings: markerMeta?.wellMappings ?? [],
			column_configs: columnConfigs,
			set_name: markerMeta?.setName ?? null,
			interpretation_type: markerMeta?.interpretationType ?? null,
			interpreter: markerMeta?.interpreter ?? null,
			depth_unit: markerMeta?.depthUnit ?? null,
			depth_reference: markerMeta?.depthReference ?? null,
			auto_create_wells: markerMeta?.autoCreateWells ?? false,
			allow_unmapped_wells: markerMeta?.allowUnmappedWells ?? false
		}

		console.log('Submitting marker ingest request:', request)

		// Call Tauri command
		const response = await invoke<IngestMarkerResponse>('ingest_marker_files', { request })

		if (!response.success) {
			throw new Error(response.error ?? 'Marker ingestion failed')
		}

		console.log('Marker ingestion response:', response)
		return true
	}

	async function submitSurfaces(): Promise<boolean> {
		// Get the first file's surface metadata for settings
		const firstFile = state.files[0]
		const surfaceMeta = firstFile?.surfaceMetadata

		// Build column configs from file metadata
		const columnConfigs: SurfaceColumnConfig[] = state.files.flatMap((f) =>
			(f.surfaceMetadata?.columns ?? [])
				.filter((c) => c.include)
				.map((c) => ({
					index: c.index,
					column_type: (c.mappedType ?? c.detectedType) as SurfaceColumnType
				}))
		)

		// Build request
		const request: IngestSurfaceRequest = {
			file_paths: state.files.map((f) => f.tempPath!).filter(Boolean),
			column_configs: columnConfigs,
			surface_name: surfaceMeta?.surfaceName ?? null,
			surface_type: surfaceMeta?.surfaceType ?? null,
			crs: surfaceMeta?.crs ?? null,
			xy_unit: surfaceMeta?.xyUnit ?? 'm',
			z_unit: surfaceMeta?.zUnit ?? 'm',
			z_positive_direction: surfaceMeta?.zPositiveDirection ?? 'down'
		}

		console.log('Submitting surface ingest request:', request)

		// Call Tauri command
		const response = await invoke<IngestSurfaceResponse>('ingest_surface_files', { request })

		if (!response.success) {
			throw new Error(response.error ?? 'Surface ingestion failed')
		}

		console.log('Surface ingestion response:', response)
		return true
	}

	async function submitCheckshots(): Promise<boolean> {
		// Get the first file's checkshot metadata for settings
		const firstFile = state.files[0]
		const checkMeta = firstFile?.checkMetadata

		// Build column configs from file metadata
		const columnConfigs: CheckColumnConfig[] = state.files.flatMap((f) =>
			(f.checkMetadata?.columns ?? [])
				.filter((c) => c.include)
				.map((c) => ({
					index: c.index,
					column_type: (c.mappedType ?? c.detectedType) as CheckshotColumnType
				}))
		)

		// Build request
		const request: IngestCheckRequest = {
			file_paths: state.files.map((f) => f.tempPath!).filter(Boolean),
			well_id: state.config.targetWellId ?? null,
			well_name: state.config.newWellName ?? null,
			create_well: state.config.createNewWell ?? false,
			column_configs: columnConfigs,
			survey_type: checkMeta?.surveyType ?? null,
			survey_company: checkMeta?.surveyCompany ?? null,
			depth_unit: checkMeta?.depthUnit ?? 'ft',
			time_unit: checkMeta?.timeUnit ?? 's',
			datum: checkMeta?.datum ?? null,
			calculate_velocity: checkMeta?.calculateVelocity ?? false
		}

		console.log('Submitting checkshot ingest request:', request)

		// Call Tauri command
		const response = await invoke<IngestCheckResponse>('ingest_checkshot_files', { request })

		if (!response.success) {
			throw new Error(response.error ?? 'Checkshot ingestion failed')
		}

		console.log('Checkshot ingestion response:', response)
		return true
	}

	/**
	 * Update marker column mapping
	 */
	function updateMarkerColumnMapping(
		fileId: string,
		columnIndex: number,
		updates: Partial<MarkerColumnMappingInfo>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.markerMetadata) {
			const colIdx = file.markerMetadata.columns.findIndex((c) => c.index === columnIndex)
			if (colIdx !== -1) {
				file.markerMetadata.columns[colIdx] = {
					...file.markerMetadata.columns[colIdx],
					...updates
				}
			}
		}
	}

	/**
	 * Update marker set metadata
	 */
	function updateMarkerSetMetadata(
		fileId: string,
		updates: Partial<
			Pick<
				MarkerFileMetadata,
				| 'setName'
				| 'interpretationType'
				| 'interpreter'
				| 'depthUnit'
				| 'depthReference'
				| 'autoCreateWells'
				| 'allowUnmappedWells'
			>
		>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.markerMetadata) {
			file.markerMetadata = {
				...file.markerMetadata,
				...updates
			}
		}
	}

	/**
	 * Update well mapping for marker files
	 */
	function updateMarkerWellMapping(
		fileId: string,
		wellName: string,
		updates: Partial<WellMappingEntry>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.markerMetadata) {
			const mappingIdx = file.markerMetadata.wellMappings.findIndex(
				(m) => m.well_name === wellName
			)
			if (mappingIdx !== -1) {
				file.markerMetadata.wellMappings[mappingIdx] = {
					...file.markerMetadata.wellMappings[mappingIdx],
					...updates
				}
			}
		}
	}

	/**
	 * Update trajectory column mapping
	 */
	function updateTrajectoryColumnMapping(
		fileId: string,
		columnIndex: number,
		updates: Partial<TrajectoryColumnMappingInfo>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.trajectoryMetadata) {
			const colIdx = file.trajectoryMetadata.columns.findIndex((c) => c.index === columnIndex)
			if (colIdx !== -1) {
				file.trajectoryMetadata.columns[colIdx] = {
					...file.trajectoryMetadata.columns[colIdx],
					...updates
				}
			}
		}
	}

	/**
	 * Update trajectory survey metadata
	 */
	function updateTrajectorySurveyMetadata(
		fileId: string,
		updates: Partial<
			Pick<
				TrajectoryFileMetadata,
				| 'surveyType'
				| 'surveyCompany'
				| 'magneticDeclination'
				| 'gridConvergence'
				| 'mdUnit'
				| 'angleUnit'
			>
		>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.trajectoryMetadata) {
			file.trajectoryMetadata = {
				...file.trajectoryMetadata,
				...updates
			}
		}
	}

	/**
	 * Update surface column mapping
	 */
	function updateSurfaceColumnMapping(
		fileId: string,
		columnIndex: number,
		updates: Partial<SurfaceColumnMappingInfo>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.surfaceMetadata) {
			const colIdx = file.surfaceMetadata.columns.findIndex((c) => c.index === columnIndex)
			if (colIdx !== -1) {
				file.surfaceMetadata.columns[colIdx] = {
					...file.surfaceMetadata.columns[colIdx],
					...updates
				}
			}
		}
	}

	/**
	 * Update surface metadata
	 */
	function updateSurfaceMetadata(
		fileId: string,
		updates: Partial<
			Pick<
				SurfaceFileMetadata,
				| 'surfaceName'
				| 'surfaceType'
				| 'crs'
				| 'xyUnit'
				| 'zUnit'
				| 'zPositiveDirection'
			>
		>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.surfaceMetadata) {
			file.surfaceMetadata = {
				...file.surfaceMetadata,
				...updates
			}
		}
	}

	/**
	 * Update checkshot column mapping
	 */
	function updateCheckColumnMapping(
		fileId: string,
		columnIndex: number,
		updates: Partial<CheckshotColumnMappingInfo>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.checkMetadata) {
			const colIdx = file.checkMetadata.columns.findIndex((c) => c.index === columnIndex)
			if (colIdx !== -1) {
				file.checkMetadata.columns[colIdx] = {
					...file.checkMetadata.columns[colIdx],
					...updates
				}
			}
		}
	}

	/**
	 * Update checkshot metadata
	 */
	function updateCheckMetadata(
		fileId: string,
		updates: Partial<
			Pick<
				CheckshotFileMetadata,
				| 'surveyType'
				| 'surveyCompany'
				| 'depthUnit'
				| 'timeUnit'
				| 'datum'
				| 'calculateVelocity'
			>
		>
	) {
		const file = state.files.find((f) => f.id === fileId)
		if (file?.checkMetadata) {
			file.checkMetadata = {
				...file.checkMetadata,
				...updates
			}
		}
	}

	return {
		// State (readonly getters)
		get currentStep() {
			return state.currentStep
		},
		get sourceType() {
			return state.sourceType
		},
		get files() {
			return state.files
		},
		get config() {
			return state.config
		},
		get isSubmitting() {
			return state.isSubmitting
		},
		get submitError() {
			return state.submitError
		},
		get stepOrder() {
			return stepOrder
		},

		// Computed
		getStepIndex,
		canAccessStep,

		// Actions
		setSourceType,
		goToStep,
		nextStep,
		previousStep,
		addFile,
		updateFile,
		removeFile,
		clearFiles,
		updateCurveMapping,
		setTargetWell,
		reset,
		submit,
		// File processing
		parseLasFile,
		parseTrajectoryFile,
		parseMarkerFile,
		parseSurfaceFile,
		parseCheckFile,
		processFile,
		// Trajectory-specific
		updateTrajectoryColumnMapping,
		updateTrajectorySurveyMetadata,
		// Marker-specific
		updateMarkerColumnMapping,
		updateMarkerSetMetadata,
		updateMarkerWellMapping,
		// Surface-specific
		updateSurfaceColumnMapping,
		updateSurfaceMetadata,
		// Checkshot-specific
		updateCheckColumnMapping,
		updateCheckMetadata
	}
}

// Export singleton store
export const ingestStore = createIngestStore()
