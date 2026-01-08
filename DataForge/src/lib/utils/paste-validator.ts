/**
 * Paste Validation Module
 *
 * Validates clipboard data against table schema before pasting.
 * Provides detailed error messages for type mismatches and constraint violations.
 */

export interface ColumnInfo {
	name: string
	data_type: string
	nullable: boolean
	primary_key: boolean
	foreign_key: { target_table: string; target_column: string } | null
}

export interface ValidationError {
	rowIndex: number
	columnName: string
	value: string
	message: string
}

export interface ValidationWarning {
	rowIndex: number
	columnName: string
	message: string
}

export interface ValidationResult {
	/** Whether all rows passed validation */
	isValid: boolean
	/** Successfully validated and converted rows */
	validRows: Record<string, unknown>[]
	/** Validation errors (blocking) */
	errors: ValidationError[]
	/** Validation warnings (non-blocking) */
	warnings: ValidationWarning[]
	/** Summary statistics */
	stats: {
		totalRows: number
		validCount: number
		errorCount: number
	}
}

/**
 * Get the default value for a column type
 */
export function getDefaultForType(dataType: string, nullable: boolean): unknown {
	if (nullable) {
		return null
	}

	const upperType = dataType.toUpperCase()

	if (upperType.includes('INT')) {
		return 0
	}
	if (
		upperType.includes('REAL') ||
		upperType.includes('FLOAT') ||
		upperType.includes('DOUBLE') ||
		upperType.includes('NUMERIC') ||
		upperType.includes('DECIMAL')
	) {
		return 0.0
	}
	if (upperType.includes('BOOL')) {
		return false
	}

	// TEXT/VARCHAR and default
	return ''
}

/**
 * Validate and convert a single value based on column type
 * Returns { success, value, error }
 */
function validateValue(
	value: string,
	column: ColumnInfo
): { success: boolean; value: unknown; error?: string } {
	const trimmedValue = value.trim()
	const upperType = column.data_type.toUpperCase()

	// Handle empty values
	if (trimmedValue === '' || trimmedValue.toLowerCase() === 'null') {
		if (!column.nullable) {
			return {
				success: false,
				value: null,
				error: `Required field cannot be empty`
			}
		}
		return { success: true, value: null }
	}

	// Integer types
	if (upperType.includes('INT')) {
		const parsed = parseInt(trimmedValue, 10)
		if (isNaN(parsed)) {
			return {
				success: false,
				value: null,
				error: `Expected integer, got '${trimmedValue}'`
			}
		}
		// Check for non-integer input (e.g., "3.14")
		if (trimmedValue.includes('.')) {
			return {
				success: false,
				value: null,
				error: `Expected integer, got decimal '${trimmedValue}'`
			}
		}
		return { success: true, value: parsed }
	}

	// Float/Real types
	if (
		upperType.includes('REAL') ||
		upperType.includes('FLOAT') ||
		upperType.includes('DOUBLE') ||
		upperType.includes('NUMERIC') ||
		upperType.includes('DECIMAL')
	) {
		const parsed = parseFloat(trimmedValue)
		if (isNaN(parsed)) {
			return {
				success: false,
				value: null,
				error: `Expected number, got '${trimmedValue}'`
			}
		}
		return { success: true, value: parsed }
	}

	// Boolean types
	if (upperType.includes('BOOL')) {
		const lower = trimmedValue.toLowerCase()
		if (lower === 'true' || lower === '1' || lower === 'yes') {
			return { success: true, value: true }
		}
		if (lower === 'false' || lower === '0' || lower === 'no') {
			return { success: true, value: false }
		}
		return {
			success: false,
			value: null,
			error: `Expected boolean (true/false/1/0/yes/no), got '${trimmedValue}'`
		}
	}

	// TEXT/String types (default) - always valid
	return { success: true, value: trimmedValue }
}

/**
 * Validate clipboard data against table schema
 *
 * @param clipboardRows - Parsed clipboard rows (2D string array)
 * @param columns - Editable columns to validate against (excluding PKs)
 * @returns Validation result with valid rows and any errors
 */
export function validateClipboardData(
	clipboardRows: string[][],
	columns: ColumnInfo[]
): ValidationResult {
	const errors: ValidationError[] = []
	const warnings: ValidationWarning[] = []
	const validRows: Record<string, unknown>[] = []

	for (let rowIndex = 0; rowIndex < clipboardRows.length; rowIndex++) {
		const clipboardRow = clipboardRows[rowIndex]
		const rowData: Record<string, unknown> = {}
		let rowHasError = false

		// Map clipboard columns to schema columns by position
		for (let colIndex = 0; colIndex < columns.length; colIndex++) {
			const column = columns[colIndex]
			const clipboardValue = clipboardRow[colIndex] ?? ''

			// If clipboard has fewer columns, use default
			if (colIndex >= clipboardRow.length) {
				rowData[column.name] = getDefaultForType(column.data_type, column.nullable)

				// Add warning for missing columns
				if (!column.nullable) {
					warnings.push({
						rowIndex,
						columnName: column.name,
						message: `Missing column, using default value`
					})
				}
				continue
			}

			// Validate and convert the value
			const result = validateValue(clipboardValue, column)

			if (!result.success) {
				errors.push({
					rowIndex,
					columnName: column.name,
					value: clipboardValue,
					message: result.error || 'Validation failed'
				})
				rowHasError = true
			}

			rowData[column.name] = result.value
		}

		// Only add row if it has no errors
		if (!rowHasError) {
			validRows.push(rowData)
		}
	}

	return {
		isValid: errors.length === 0,
		validRows,
		errors,
		warnings,
		stats: {
			totalRows: clipboardRows.length,
			validCount: validRows.length,
			errorCount: clipboardRows.length - validRows.length
		}
	}
}

/**
 * Format validation errors for display
 */
export function formatValidationErrors(errors: ValidationError[]): string {
	if (errors.length === 0) return ''

	const errorLines = errors.map(
		(e) => `Row ${e.rowIndex + 1}, "${e.columnName}": ${e.message}`
	)

	if (errors.length <= 5) {
		return errorLines.join('\n')
	}

	// Show first 5 errors and a count of remaining
	const shown = errorLines.slice(0, 5)
	const remaining = errors.length - 5
	return [...shown, `... and ${remaining} more error(s)`].join('\n')
}
