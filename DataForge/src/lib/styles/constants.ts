/**
 * Global Style Constants
 *
 * These constants enforce consistent, tight layouts across all components.
 * Import and use these values to ensure adherence to design guidelines.
 */

export const STYLE_CONSTANTS = {
	// Typography
	FONT_SIZE: {
		MAX: 'text-xs', // 12px - Maximum font size allowed
		DEFAULT: 'text-xs' // 12px - Default font size
	},

	// Spacing (minimal, tight layouts)
	SPACING: {
		PADDING: {
			MIN: 'p-1', // 4px
			SMALL: 'p-2', // 8px
			MAX: 'p-2' // Maximum padding allowed
		},
		MARGIN: {
			MIN: 'm-1', // 4px
			SMALL: 'm-2', // 8px
			MAX: 'm-2' // Maximum margin allowed
		},
		GAP: {
			MIN: 'gap-1', // 4px
			SMALL: 'gap-2', // 8px
			MAX: 'gap-2' // Maximum gap allowed
		},
		SPACE_Y: {
			MIN: 'space-y-1', // 4px
			SMALL: 'space-y-2', // 8px
			MAX: 'space-y-2' // Maximum space-y allowed
		}
	},

	// Component Widths
	WIDTH: {
		MULTI_SELECT_COLUMN: 'w-[20ch]', // ~40 characters per column (1ch ≈ 0.5em, 20ch ≈ 10em)
		MULTI_SELECT_TOTAL: 'w-[42ch]' // Total width for both columns + gap + buttons
	},

	// AG Grid Column Widths
	AG_GRID: {
		COLUMN_WIDTH: 150 // Consistent column width for all AG Grid tables (pixels)
	},

	// Sidebar Widths
	SIDEBAR: {
		WIDTH: 'w-64', // 256px - Standard sidebar width
		MIN_WIDTH: 'min-w-64' // Minimum sidebar width
	},

	// Text Truncation
	TEXT_TRUNCATE: 'truncate', // Tailwind class for text ellipsis
	TEXT_NO_WRAP: 'truncate whitespace-nowrap', // Text elision with no wrapping

	// Text Alignment
	TEXT_ALIGN: {
		LEFT: 'text-left' // Left-aligned text
	},

	// Borders
	BORDER: {
		DEFAULT: 'border border-transparent',
		ROUNDED: 'rounded',
		ROUNDED_SM: 'rounded-sm'
	},

	// Colors
	COLORS: {
		TEXT: {
			PRIMARY: 'text-gray-900',
			SECONDARY: 'text-gray-600',
			MUTED: 'text-gray-400',
			DISABLED: 'text-gray-300'
		},
		BG: {
			DEFAULT: 'bg-white',
			HOVER: 'hover:bg-gray-50',
			SELECTED: 'bg-blue-50',
			SELECTED_BORDER: 'border-blue-400'
		}
	}
} as const

/**
 * Helper function to get text truncation classes
 */
export function getTextTruncate(maxChars: number = 20): string {
	return `truncate max-w-[${maxChars}ch]`
}

/**
 * Helper function to get component base classes
 */
export function getComponentBaseClasses(): string {
	return `${STYLE_CONSTANTS.FONT_SIZE.DEFAULT} ${STYLE_CONSTANTS.SPACING.PADDING.SMALL}`
}

/**
 * Helper function to get list item classes
 */
export function getListItemClasses(isSelected: boolean = false): string {
	const base = `${STYLE_CONSTANTS.FONT_SIZE.DEFAULT} ${STYLE_CONSTANTS.SPACING.PADDING.MIN} cursor-pointer border-b border-gray-100 last:border-b-0`
	const hover = isSelected ? 'bg-blue-50' : 'hover:bg-gray-50'
	return `${base} ${hover}`
}
