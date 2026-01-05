// Theme store using Svelte 5 runes
// Manages light/dark theme with system preference support

export type ThemeMode = 'light' | 'dark' | 'system'
export type ResolvedTheme = 'light' | 'dark'

const STORAGE_KEY = 'dataforge-theme'

// Theme state
let mode = $state<ThemeMode>('system')
let systemPrefersDark = $state(false)
let initialized = $state(false)

// Derived state
const resolvedTheme = $derived<ResolvedTheme>(
	mode === 'system' ? (systemPrefersDark ? 'dark' : 'light') : mode
)
const isDark = $derived(resolvedTheme === 'dark')

// Media query listener reference
let mediaQuery: MediaQueryList | null = null
let mediaQueryHandler: ((e: MediaQueryListEvent) => void) | null = null

// Apply theme to document
function applyTheme(dark: boolean): void {
	if (typeof document === 'undefined') return

	if (dark) {
		document.documentElement.classList.add('dark')
	} else {
		document.documentElement.classList.remove('dark')
	}
}

// Get system preference
function getSystemPreference(): boolean {
	if (typeof window === 'undefined') return false
	return window.matchMedia('(prefers-color-scheme: dark)').matches
}

// Set up system preference listener
function setupSystemListener(): void {
	if (typeof window === 'undefined') return

	// Clean up existing listener
	cleanupSystemListener()

	mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
	mediaQueryHandler = (e: MediaQueryListEvent) => {
		systemPrefersDark = e.matches
		// Only apply if in system mode
		if (mode === 'system') {
			applyTheme(e.matches)
		}
	}

	mediaQuery.addEventListener('change', mediaQueryHandler)
}

// Clean up system preference listener
function cleanupSystemListener(): void {
	if (mediaQuery && mediaQueryHandler) {
		mediaQuery.removeEventListener('change', mediaQueryHandler)
		mediaQuery = null
		mediaQueryHandler = null
	}
}

// Initialize theme from localStorage and set up listeners
function initialize(): void {
	if (initialized) return
	if (typeof window === 'undefined') return

	// Read stored preference
	const stored = localStorage.getItem(STORAGE_KEY)
	if (stored === 'light' || stored === 'dark' || stored === 'system') {
		mode = stored
	} else {
		mode = 'system'
	}

	// Get current system preference
	systemPrefersDark = getSystemPreference()

	// Set up listener for system preference changes
	setupSystemListener()

	// Apply theme (may already be applied by inline script, but ensures consistency)
	applyTheme(mode === 'system' ? systemPrefersDark : mode === 'dark')

	initialized = true
}

// Set theme mode
function setMode(newMode: ThemeMode): void {
	mode = newMode

	// Persist to localStorage
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, newMode)
	}

	// Apply theme immediately
	const dark = newMode === 'system' ? systemPrefersDark : newMode === 'dark'
	applyTheme(dark)
}

// Export the store
export const themeStore = {
	// State getters (reactive)
	get mode() {
		return mode
	},
	get resolvedTheme() {
		return resolvedTheme
	},
	get isDark() {
		return isDark
	},
	get initialized() {
		return initialized
	},

	// Actions
	initialize,
	setMode
}
