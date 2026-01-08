/**
 * Clipboard Parser Utility
 *
 * Parses TSV/CSV data from clipboard for pasting into AG Grid tables.
 * Handles data copied from Excel, Google Sheets, and CSV files.
 */

export interface ParsedClipboardData {
	/** Parsed rows as 2D string array */
	rows: string[][]
	/** Detected delimiter */
	delimiter: '\t' | ','
	/** Number of data rows */
	rowCount: number
	/** Maximum number of columns across all rows */
	columnCount: number
}

/**
 * Detect the delimiter used in clipboard text
 * Prioritizes tab (Excel/Sheets) over comma (CSV)
 */
export function detectDelimiter(text: string): '\t' | ',' {
	const firstLine = text.split('\n')[0] || ''

	// Count occurrences of potential delimiters
	const tabCount = (firstLine.match(/\t/g) || []).length
	const commaCount = (firstLine.match(/,/g) || []).length

	// Prefer tab if present (Excel/Sheets default)
	if (tabCount > 0) {
		return '\t'
	}

	// Fall back to comma
	return ','
}

/**
 * Parse a single CSV/TSV row, handling quoted values
 */
function parseRow(line: string, delimiter: '\t' | ','): string[] {
	const result: string[] = []
	let current = ''
	let inQuotes = false

	for (let i = 0; i < line.length; i++) {
		const char = line[i]
		const nextChar = line[i + 1]

		if (inQuotes) {
			if (char === '"') {
				if (nextChar === '"') {
					// Escaped quote
					current += '"'
					i++ // Skip next quote
				} else {
					// End of quoted value
					inQuotes = false
				}
			} else {
				current += char
			}
		} else {
			if (char === '"') {
				inQuotes = true
			} else if (char === delimiter) {
				result.push(current.trim())
				current = ''
			} else {
				current += char
			}
		}
	}

	// Add the last value
	result.push(current.trim())

	return result
}

/**
 * Parse clipboard text into structured data
 * Returns null if text is empty or invalid
 */
export function parseClipboardText(text: string): ParsedClipboardData | null {
	if (!text || text.trim() === '') {
		return null
	}

	const delimiter = detectDelimiter(text)

	// Split by newlines, handling both \r\n and \n
	let lines = text.split(/\r?\n/)

	// Remove trailing empty line (common when copying from Excel)
	if (lines.length > 0 && lines[lines.length - 1].trim() === '') {
		lines = lines.slice(0, -1)
	}

	if (lines.length === 0) {
		return null
	}

	// Parse each line
	const rows = lines.map((line) => parseRow(line, delimiter))

	// Calculate max column count
	const columnCount = Math.max(...rows.map((row) => row.length))

	return {
		rows,
		delimiter,
		rowCount: rows.length,
		columnCount
	}
}

/**
 * Convert a string value to the appropriate type based on column data type
 */
export function convertValue(
	value: string,
	dataType: string
): string | number | boolean | null {
	// Handle empty/null values
	if (value === '' || value.toLowerCase() === 'null') {
		return null
	}

	const upperType = dataType.toUpperCase()

	// Integer types
	if (upperType.includes('INT')) {
		const parsed = parseInt(value, 10)
		return isNaN(parsed) ? null : parsed
	}

	// Float/Real types
	if (
		upperType.includes('REAL') ||
		upperType.includes('FLOAT') ||
		upperType.includes('DOUBLE') ||
		upperType.includes('NUMERIC') ||
		upperType.includes('DECIMAL')
	) {
		const parsed = parseFloat(value)
		return isNaN(parsed) ? null : parsed
	}

	// Boolean types
	if (upperType.includes('BOOL')) {
		const lower = value.toLowerCase()
		if (lower === 'true' || lower === '1' || lower === 'yes') {
			return true
		}
		if (lower === 'false' || lower === '0' || lower === 'no') {
			return false
		}
		return null
	}

	// Text/String types (default)
	return value
}
