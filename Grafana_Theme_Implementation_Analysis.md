# Grafana Light/Dark Theming Implementation: End-to-End Analysis

## Executive Summary

Grafana's theming system provides runtime light/dark/system theme switching through a layered architecture: **design tokens** define visual properties, **React context** propagates theme state, **CSS variables** enable instant visual updates, and a **multi-level preference system** persists user choices. The system prioritizes User > Team > Organization > Server defaults, ensuring individual preferences override organization-wide settings.

---

## 1. Theme Model: State Representation & Design Tokens

### 1.1 Theme State Values

Grafana supports three theme states:

```typescript
type ThemeMode = 'light' | 'dark' | 'system';
```

- **`light`**: Forces light theme
- **`dark`**: Forces dark theme
- **`system`**: Follows OS preference via `prefers-color-scheme` media query

### 1.2 GrafanaTheme2 Interface

The `GrafanaTheme2` interface is the primary theme object structure:

```typescript
// packages/grafana-data/src/themes/types.ts
interface GrafanaTheme2 {
  // Theme identity
  name: string; // 'Grafana Light' | 'Grafana Dark'
  isDark: boolean; // true for dark theme
  isLight: boolean; // true for light theme

  // Design tokens
  colors: ThemeColors;
  spacing: ThemeSpacing;
  typography: ThemeTypography;
  breakpoints: ThemeBreakpoints;
  shadows: ThemeShadows;
  shape: ThemeShape;
  visualization: ThemeVisualizationColors;
  transitions: ThemeTransitions;
  zIndex: ThemeZIndex;

  // Component-specific tokens
  components: ThemeComponents;

  // Utility functions
  spacing: (
    top: number,
    right?: number,
    bottom?: number,
    left?: number
  ) => string;
}
```

### 1.3 Color Token Structure

Colors are organized hierarchically:

```typescript
// packages/grafana-data/src/themes/createColors.ts
interface ThemeColors {
  // Mode
  mode: 'light' | 'dark';

  // Primary palette
  primary: {
    main: string;
    shade: string;
    text: string;
    border: string;
    contrastText: string;
  };

  // Secondary palette
  secondary: {
    main: string;
    shade: string;
    text: string;
    border: string;
    contrastText: string;
  };

  // Semantic colors
  success: ColorSwatch;
  warning: ColorSwatch;
  error: ColorSwatch;
  info: ColorSwatch;

  // Background colors
  background: {
    canvas: string; // Page background
    primary: string; // Card/panel background
    secondary: string; // Nested element background
  };

  // Border colors
  border: {
    weak: string;
    medium: string;
    strong: string;
  };

  // Text colors
  text: {
    primary: string;
    secondary: string;
    disabled: string;
    link: string;
    maxContrast: string;
  };

  // Action colors (hover, selected, etc.)
  action: {
    hover: string;
    hoverOpacity: number;
    focus: string;
    selected: string;
    selectedBorder: string;
    disabledBackground: string;
    disabledText: string;
    disabledOpacity: number;
  };

  // Emphasis colors
  emphasize: (color: string, amount: number) => string;

  // Gradients
  gradients: {
    brandVertical: string;
    brandHorizontal: string;
  };
}
```

### 1.4 Theme Creation Functions

```typescript
// packages/grafana-data/src/themes/createTheme.ts
export function createTheme(
  options: ThemeOptions = {}
): GrafanaTheme2 {
  const { colors: colorsInput = {} } = options;

  // Determine theme mode
  const mode = colorsInput.mode ?? 'dark';
  const isDark = mode === 'dark';

  // Create color palette based on mode
  const colors = createColors(colorsInput);

  // Create spacing scale
  const spacing = createSpacing();

  // Create typography
  const typography = createTypography(colors);

  // Create shadows
  const shadows = createShadows(colors);

  // Create breakpoints
  const breakpoints = createBreakpoints();

  // Create component tokens
  const components = createComponents(colors, shadows);

  return {
    name: isDark ? 'Grafana Dark' : 'Grafana Light',
    isDark,
    isLight: !isDark,
    colors,
    spacing,
    typography,
    shadows,
    breakpoints,
    components,
    // ...other properties
  };
}
```

### 1.5 CSS Variable Mapping

Design tokens map to CSS custom properties:

```css
/* Generated CSS variables */
:root {
  /* Colors */
  --color-background-canvas: #111217;
  --color-background-primary: #181b1f;
  --color-background-secondary: #22252b;

  --color-text-primary: rgba(255, 255, 255, 0.9);
  --color-text-secondary: rgba(255, 255, 255, 0.6);
  --color-text-disabled: rgba(255, 255, 255, 0.38);

  --color-border-weak: rgba(255, 255, 255, 0.08);
  --color-border-medium: rgba(255, 255, 255, 0.15);
  --color-border-strong: rgba(255, 255, 255, 0.25);

  /* Spacing */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;

  /* Typography */
  --font-family: 'Inter', 'Helvetica', 'Arial', sans-serif;
  --font-size-sm: 12px;
  --font-size-md: 14px;
  --font-size-lg: 18px;

  /* Shadows */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.2);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.3);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.4);
}

/* Light theme overrides */
[data-theme='light'] {
  --color-background-canvas: #f4f5f5;
  --color-background-primary: #ffffff;
  --color-background-secondary: #f4f5f5;

  --color-text-primary: rgba(0, 0, 0, 0.87);
  --color-text-secondary: rgba(0, 0, 0, 0.54);
  --color-text-disabled: rgba(0, 0, 0, 0.38);

  --color-border-weak: rgba(0, 0, 0, 0.08);
  --color-border-medium: rgba(0, 0, 0, 0.15);
  --color-border-strong: rgba(0, 0, 0, 0.25);
}
```

---

## 2. Persistence Layers: Storage & Precedence Resolution

### 2.1 Multi-Level Preference Hierarchy

Grafana stores theme preferences at four levels with strict precedence:

```
┌─────────────────────────────────────────────────────────────┐
│                    PRECEDENCE HIERARCHY                      │
│                                                              │
│  ┌────────────────────┐                                     │
│  │  USER PREFERENCE   │  ← Highest priority                 │
│  │  (per-user setting)│                                     │
│  └─────────┬──────────┘                                     │
│            │ overrides                                       │
│  ┌─────────▼──────────┐                                     │
│  │  TEAM PREFERENCE   │                                     │
│  │  (team-wide default)│                                    │
│  └─────────┬──────────┘                                     │
│            │ overrides                                       │
│  ┌─────────▼──────────┐                                     │
│  │   ORG PREFERENCE   │                                     │
│  │  (org-wide default)│                                     │
│  └─────────┬──────────┘                                     │
│            │ overrides                                       │
│  ┌─────────▼──────────┐                                     │
│  │  SERVER DEFAULT    │  ← Lowest priority                  │
│  │  (grafana.ini)     │                                     │
│  └────────────────────┘                                     │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Storage Locations

#### User Preferences (Database)

```sql
-- User preferences table
CREATE TABLE user_preferences (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  org_id INTEGER NOT NULL REFERENCES org(id),
  team_id INTEGER,
  theme VARCHAR(20),           -- 'light' | 'dark' | 'system' | ''
  home_dashboard_id INTEGER,
  timezone VARCHAR(50),
  week_start VARCHAR(10),
  locale VARCHAR(20),
  navbar JSON,
  query_history JSON,
  created TIMESTAMP,
  updated TIMESTAMP,
  UNIQUE(user_id, org_id)
);
```

#### Organization Preferences (Database)

```sql
-- Organization preferences table
CREATE TABLE org_preferences (
  id SERIAL PRIMARY KEY,
  org_id INTEGER NOT NULL REFERENCES org(id) UNIQUE,
  theme VARCHAR(20),           -- 'light' | 'dark' | 'system' | ''
  home_dashboard_id INTEGER,
  timezone VARCHAR(50),
  week_start VARCHAR(10),
  created TIMESTAMP,
  updated TIMESTAMP
);
```

#### Server Configuration (grafana.ini)

```ini
# /etc/grafana/grafana.ini
[users]
# Default theme for new users
# Options: light, dark, system
default_theme = dark
```

### 2.3 Preferences API Endpoints

#### Get User Preferences

```http
GET /api/user/preferences
Authorization: Bearer <token>

Response:
{
  "theme": "dark",
  "homeDashboardId": 0,
  "timezone": "browser",
  "weekStart": "monday",
  "locale": "",
  "navbar": {
    "bookmarkUrls": []
  },
  "queryHistory": {
    "homeTab": ""
  }
}
```

#### Update User Preferences

```http
PUT /api/user/preferences
Authorization: Bearer <token>
Content-Type: application/json

{
  "theme": "light",
  "homeDashboardId": 0,
  "timezone": "browser",
  "weekStart": "monday"
}

Response:
{
  "message": "Preferences updated"
}
```

#### Patch User Preferences (Partial Update)

```http
PATCH /api/user/preferences
Authorization: Bearer <token>
Content-Type: application/json

{
  "theme": "dark"
}

Response:
{
  "message": "Preferences updated"
}
```

#### Get Organization Preferences

```http
GET /api/org/preferences
Authorization: Bearer <token>

Response:
{
  "theme": "",
  "homeDashboardId": 0,
  "timezone": "",
  "weekStart": ""
}
```

#### Update Organization Preferences

```http
PUT /api/org/preferences
Authorization: Bearer <token>
Content-Type: application/json

{
  "theme": "dark",
  "homeDashboardId": 0,
  "timezone": "utc",
  "weekStart": "monday"
}

Response:
{
  "message": "Preferences updated"
}
```

### 2.4 Effective Theme Resolution

```typescript
// public/app/core/services/PreferencesService.ts
interface PreferencesService {
  // Load preferences with precedence resolution
  load(): Promise<Preferences>;

  // Save user preferences
  update(preferences: Partial<Preferences>): Promise<void>;

  // Get effective theme (resolved from all layers)
  getEffectiveTheme(): ThemeMode;
}

// Backend resolution logic (pkg/services/preference/prefimpl/pref.go)
function resolveEffectiveTheme(
  userPref: Preferences | null,
  teamPref: Preferences | null,
  orgPref: Preferences | null,
  serverDefault: string
): ThemeMode {
  // Check user preference first
  if (userPref?.theme && userPref.theme !== '') {
    return userPref.theme as ThemeMode;
  }

  // Check team preference
  if (teamPref?.theme && teamPref.theme !== '') {
    return teamPref.theme as ThemeMode;
  }

  // Check organization preference
  if (orgPref?.theme && orgPref.theme !== '') {
    return orgPref.theme as ThemeMode;
  }

  // Fall back to server default
  return (serverDefault as ThemeMode) || 'dark';
}
```

### 2.5 System Theme Detection

When theme is set to `'system'`:

```typescript
// public/app/core/utils/themeUtils.ts
function getSystemTheme(): 'light' | 'dark' {
  // Use CSS media query to detect OS preference
  const prefersDark = window.matchMedia(
    '(prefers-color-scheme: dark)'
  ).matches;
  return prefersDark ? 'dark' : 'light';
}

// Listen for system theme changes
function watchSystemTheme(
  callback: (theme: 'light' | 'dark') => void
): () => void {
  const mediaQuery = window.matchMedia(
    '(prefers-color-scheme: dark)'
  );

  const handler = (e: MediaQueryListEvent) => {
    callback(e.matches ? 'dark' : 'light');
  };

  mediaQuery.addEventListener('change', handler);

  return () => {
    mediaQuery.removeEventListener('change', handler);
  };
}
```

---

## 3. Frontend Architecture: React Providers & Hooks

### 3.1 Theme Context Structure

```typescript
// packages/grafana-ui/src/themes/ThemeContext.tsx
import { createContext, useContext } from 'react';
import { GrafanaTheme2 } from '@grafana/data';

// Create theme context
const ThemeContext = createContext<
  GrafanaTheme2 | undefined
>(undefined);

// Theme context provider component
interface ThemeContextProviderProps {
  value: GrafanaTheme2;
  children: React.ReactNode;
}

export const ThemeContextProvider: React.FC<
  ThemeContextProviderProps
> = ({ value, children }) => {
  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  );
};

// Hook to access theme
export function useTheme2(): GrafanaTheme2 {
  const theme = useContext(ThemeContext);

  if (!theme) {
    throw new Error(
      'useTheme2 must be used within a ThemeContextProvider'
    );
  }

  return theme;
}
```

### 3.2 useStyles2 Hook (Memoized Styling)

```typescript
// packages/grafana-ui/src/themes/ThemeContext.tsx
import { useMemo } from 'react';
import { css, CSSObject } from '@emotion/css';

type StylesCreator<T> = (theme: GrafanaTheme2) => T;

export function useStyles2<T>(
  getStyles: StylesCreator<T>
): T {
  const theme = useTheme2();

  // Memoize styles based on theme reference
  return useMemo(
    () => getStyles(theme),
    [theme, getStyles]
  );
}

// Usage example
const MyComponent: React.FC = () => {
  const styles = useStyles2(getStyles);

  return (
    <div className={styles.container}>
      <span className={styles.text}>Themed content</span>
    </div>
  );
};

const getStyles = (theme: GrafanaTheme2) => ({
  container: css({
    backgroundColor: theme.colors.background.primary,
    padding: theme.spacing(2),
    borderRadius: theme.shape.radius.default,
    border: `1px solid ${theme.colors.border.weak}`,
  }),
  text: css({
    color: theme.colors.text.primary,
    fontSize: theme.typography.body.fontSize,
  }),
});
```

### 3.3 Global Theme Provider (App Root)

```typescript
// public/app/core/components/AppChrome/AppChrome.tsx
import { ThemeContextProvider } from '@grafana/ui';
import { config } from '@grafana/runtime';
import { createTheme } from '@grafana/data';

interface AppChromeProps {
  children: React.ReactNode;
}

export const AppChrome: React.FC<AppChromeProps> = ({
  children,
}) => {
  // Get theme from global config (resolved from preferences)
  const theme = useMemo(() => {
    return createTheme({
      colors: {
        mode: config.theme2.isDark ? 'dark' : 'light',
      },
    });
  }, [config.theme2]);

  return (
    <ThemeContextProvider value={theme}>
      <div
        className='grafana-app'
        data-theme={theme.isDark ? 'dark' : 'light'}
      >
        {children}
      </div>
    </ThemeContextProvider>
  );
};
```

### 3.4 Theme State Management

```typescript
// public/app/core/services/context_srv.ts
interface ContextService {
  user: {
    id: number;
    isSignedIn: boolean;
    theme: ThemeMode;
    // ... other user properties
  };

  // Update theme
  setTheme(theme: ThemeMode): void;
}

// public/app/core/reducers/appReducer.ts
interface AppState {
  theme: ThemeMode;
  effectiveTheme: 'light' | 'dark'; // Resolved theme (handles 'system')
}

const appReducer = (
  state: AppState,
  action: AppAction
): AppState => {
  switch (action.type) {
    case 'SET_THEME':
      return {
        ...state,
        theme: action.payload.theme,
        effectiveTheme: resolveEffectiveTheme(
          action.payload.theme
        ),
      };
    default:
      return state;
  }
};

function resolveEffectiveTheme(
  theme: ThemeMode
): 'light' | 'dark' {
  if (theme === 'system') {
    return getSystemTheme();
  }
  return theme;
}
```

### 3.5 Component Re-rendering on Theme Change

When the theme changes:

1. **Context Update**: `ThemeContextProvider` receives new theme value
2. **Consumer Re-render**: All components using `useTheme2()` or `useStyles2()` re-render
3. **Style Recalculation**: `useStyles2` memoization invalidates, styles are recalculated
4. **DOM Update**: React applies new className values with updated styles

```typescript
// How useStyles2 triggers re-render
export function useStyles2<T>(
  getStyles: StylesCreator<T>
): T {
  const theme = useTheme2(); // Subscribes to ThemeContext

  // When theme changes:
  // 1. useContext(ThemeContext) returns new theme reference
  // 2. useMemo dependency [theme] changes
  // 3. getStyles(theme) is called with new theme
  // 4. New styles object is returned
  // 5. Component re-renders with new className
  return useMemo(
    () => getStyles(theme),
    [theme, getStyles]
  );
}
```

---

## 4. Call Stack: Theme Switch Flow

### 4.1 Complete Call Stack Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ USER SWITCHES THEME IN UI                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
│
├─> 1. USER ACTION
│   │   User clicks theme toggle in Profile Settings or preferences dropdown
│   │
│   └─> PreferencesForm.onThemeChange('dark')
│       │
│       └─> handleThemeChange(newTheme: ThemeMode)
│           │
│           ├─> // Update local form state
│           │   setFormState({ ...formState, theme: newTheme })
│           │
│           └─> // Trigger save
│               onSubmit()
│
├─> 2. API PERSISTENCE
│   │
│   └─> PreferencesService.update({ theme: 'dark' })
│       │
│       ├─> // Build API request
│       │   const request = {
│       │     method: 'PATCH',
│       │     url: '/api/user/preferences',
│       │     body: { theme: 'dark' }
│       │   }
│       │
│       └─> fetch(request)
│           │
│           └─> Backend: pkg/api/preferences.go
│               │
│               ├─> UpdateUserPreferences()
│               │   │
│               │   ├─> Validate theme value
│               │   │   if (!['light', 'dark', 'system', ''].includes(theme)) {
│               │   │     return error('Invalid theme')
│               │   │   }
│               │   │
│               │   ├─> Update database
│               │   │   UPDATE user_preferences
│               │   │   SET theme = 'dark', updated = NOW()
│               │   │   WHERE user_id = ? AND org_id = ?
│               │   │
│               │   └─> Return success
│               │       { message: 'Preferences updated' }
│               │
│               └─> Response: 200 OK
│
├─> 3. FRONTEND STATE UPDATE
│   │
│   └─> On API success:
│       │
│       ├─> // Update global config
│       │   config.bootData.user.theme = 'dark'
│       │
│       ├─> // Dispatch Redux action
│       │   dispatch({
│       │     type: 'SET_THEME',
│       │     payload: { theme: 'dark' }
│       │   })
│       │
│       └─> // Trigger theme refresh
│           refreshTheme()
│
├─> 4. THEME CONTEXT UPDATE
│   │
│   └─> refreshTheme()
│       │
│       ├─> // Resolve effective theme
│       │   const effectiveTheme = theme === 'system'
│       │     ? getSystemTheme()
│       │     : theme
│       │
│       ├─> // Create new theme object
│       │   const newTheme = createTheme({
│       │     colors: { mode: effectiveTheme }
│       │   })
│       │
│       ├─> // Update config.theme2
│       │   config.theme2 = newTheme
│       │
│       └─> // Update ThemeContext provider
│           setTheme(newTheme)
│           │
│           └─> ThemeContextProvider re-renders with new value
│
├─> 5. COMPONENT RE-RENDERING
│   │
│   └─> React propagates new theme through context
│       │
│       ├─> All useTheme2() consumers detect change
│       │   │
│       │   └─> Components re-render
│       │
│       └─> All useStyles2() consumers recalculate styles
│           │
│           ├─> useMemo dependency [theme] changed
│           │
│           └─> getStyles(newTheme) called
│               │
│               └─> New Emotion CSS classes generated
│
└─> 6. CSS APPLICATION
    │
    ├─> // Update data-theme attribute on root
    │   document.documentElement.setAttribute('data-theme', 'dark')
    │
    ├─> // CSS variables cascade through DOM
    │   [data-theme="dark"] {
    │     --color-background-canvas: #111217;
    │     --color-text-primary: rgba(255, 255, 255, 0.9);
    │     /* ... all theme variables ... */
    │   }
    │
    └─> // Emotion-generated classes apply new values
        .css-abc123 {
          background-color: var(--color-background-canvas);
          color: var(--color-text-primary);
        }
```

### 4.2 Sequence Diagram

```
┌─────────┐  ┌──────────────┐  ┌─────────────┐  ┌──────────────┐  ┌────────────┐
│  User   │  │ PrefsForm    │  │ PrefsAPI    │  │ ThemeContext │  │ Components │
└────┬────┘  └──────┬───────┘  └──────┬──────┘  └──────┬───────┘  └─────┬──────┘
     │              │                 │                │                │
     │ Click toggle │                 │                │                │
     │─────────────>│                 │                │                │
     │              │                 │                │                │
     │              │ PATCH /api/user/preferences      │                │
     │              │────────────────>│                │                │
     │              │                 │                │                │
     │              │                 │ Update DB      │                │
     │              │                 │───────┐        │                │
     │              │                 │       │        │                │
     │              │                 │<──────┘        │                │
     │              │                 │                │                │
     │              │     200 OK      │                │                │
     │              │<────────────────│                │                │
     │              │                 │                │                │
     │              │ setTheme(newTheme)               │                │
     │              │────────────────────────────────->│                │
     │              │                 │                │                │
     │              │                 │                │ Context update │
     │              │                 │                │───────────────>│
     │              │                 │                │                │
     │              │                 │                │                │ Re-render
     │              │                 │                │                │───────┐
     │              │                 │                │                │       │
     │ UI Updated   │                 │                │                │<──────┘
     │<─────────────────────────────────────────────────────────────────│
     │              │                 │                │                │
```

### 4.3 Code Implementation

```typescript
// public/app/features/profile/UserPreferencesForm.tsx
const UserPreferencesForm: React.FC = () => {
  const [preferences, setPreferences] =
    useState<Preferences | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // Load preferences on mount
  useEffect(() => {
    loadPreferences();
  }, []);

  const loadPreferences = async () => {
    const prefs = await getBackendSrv().get(
      '/api/user/preferences'
    );
    setPreferences(prefs);
  };

  const handleThemeChange = async (theme: ThemeMode) => {
    setIsLoading(true);

    try {
      // 1. Persist to backend
      await getBackendSrv().patch('/api/user/preferences', {
        theme,
      });

      // 2. Update local state
      setPreferences({ ...preferences!, theme });

      // 3. Trigger global theme update
      await refreshTheme(theme);

      // 4. Show success notification
      appEvents.emit(AppEvents.alertSuccess, [
        'Theme updated',
      ]);
    } catch (error) {
      appEvents.emit(AppEvents.alertError, [
        'Failed to update theme',
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form>
      <Field label='UI Theme'>
        <RadioButtonGroup
          options={[
            { label: 'Dark', value: 'dark' },
            { label: 'Light', value: 'light' },
            { label: 'System', value: 'system' },
          ]}
          value={preferences?.theme || 'dark'}
          onChange={handleThemeChange}
          disabled={isLoading}
        />
      </Field>
    </form>
  );
};

// public/app/core/utils/ConfigProvider.tsx
export async function refreshTheme(
  theme: ThemeMode
): Promise<void> {
  // Resolve effective theme
  const effectiveTheme =
    theme === 'system' ? getSystemTheme() : theme;

  // Create new theme object
  const newTheme = createTheme({
    colors: { mode: effectiveTheme },
  });

  // Update global config
  config.theme2 = newTheme;
  config.bootData.user.theme = theme;

  // Update DOM attribute for CSS variable cascade
  document.documentElement.setAttribute(
    'data-theme',
    effectiveTheme
  );

  // Emit theme changed event for any listeners
  appEvents.emit(ThemeChangedEvent, { theme: newTheme });
}
```

---

## 5. Plugin Implications: Theme Compatibility

### 5.1 Accessing Theme in Plugins

Panel plugins access the theme via hooks:

```typescript
// Example panel plugin
import { PanelProps } from '@grafana/data';
import { useTheme2, useStyles2 } from '@grafana/ui';
import { css } from '@emotion/css';

interface MyPanelOptions {
  showLegend: boolean;
}

export const MyPanel: React.FC<
  PanelProps<MyPanelOptions>
> = (props) => {
  const { data, options, width, height } = props;
  const theme = useTheme2();
  const styles = useStyles2(getStyles);

  return (
    <div className={styles.container}>
      <div
        className={styles.chart}
        style={{
          width,
          height,
          // Use theme colors directly
          borderColor: theme.colors.border.medium,
        }}
      >
        {/* Chart content */}
      </div>
    </div>
  );
};

const getStyles = (theme: GrafanaTheme2) => ({
  container: css({
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
  }),
  chart: css({
    flex: 1,
    backgroundColor: theme.colors.background.secondary,
    border: `1px solid ${theme.colors.border.weak}`,
    borderRadius: theme.shape.radius.default,
  }),
});
```

### 5.2 Theme Tokens for Visualizations

For chart/visualization colors:

```typescript
// Access visualization-specific colors
const theme = useTheme2();

// Get palette colors for data series
const seriesColors = theme.visualization.palette;
// Returns: ['#7EB26D', '#EAB839', '#6ED0E0', '#EF843C', ...]

// Get specific visualization colors
const colors = {
  // Good for positive values
  green: theme.visualization.getColorByName('green'),

  // Good for negative values
  red: theme.visualization.getColorByName('red'),

  // Neutral
  blue: theme.visualization.getColorByName('blue'),

  // Get color by index (cycles through palette)
  series0: theme.visualization.palette[0],
  series1: theme.visualization.palette[1],
};
```

### 5.3 Best Practices for Plugin Theme Compatibility

```typescript
// ✅ DO: Use theme tokens
const styles = useStyles2((theme) => ({
  panel: css({
    backgroundColor: theme.colors.background.primary,
    color: theme.colors.text.primary,
    padding: theme.spacing(2),
    borderRadius: theme.shape.radius.default,
  }),
}));

// ❌ DON'T: Hardcode colors
const badStyles = css({
  backgroundColor: '#1a1a1a', // Won't adapt to light theme
  color: 'white', // Won't adapt to light theme
  padding: '16px', // Not using spacing scale
});

// ✅ DO: Use semantic color tokens
const semanticStyles = useStyles2((theme) => ({
  success: css({ color: theme.colors.success.text }),
  warning: css({ color: theme.colors.warning.text }),
  error: css({ color: theme.colors.error.text }),
}));

// ✅ DO: Support both themes in canvas/SVG rendering
function renderChart(
  ctx: CanvasRenderingContext2D,
  theme: GrafanaTheme2
) {
  // Background
  ctx.fillStyle = theme.colors.background.primary;
  ctx.fillRect(0, 0, width, height);

  // Grid lines
  ctx.strokeStyle = theme.colors.border.weak;
  ctx.lineWidth = 1;

  // Axis labels
  ctx.fillStyle = theme.colors.text.secondary;
  ctx.font = `${theme.typography.bodySmall.fontSize} ${theme.typography.fontFamily}`;

  // Data series
  data.forEach((series, i) => {
    ctx.strokeStyle =
      theme.visualization.palette[
        i % theme.visualization.palette.length
      ];
    // Draw series...
  });
}
```

### 5.4 Plugin Theme Testing

```typescript
// Test plugin renders correctly in both themes
import { render } from '@testing-library/react';
import { createTheme } from '@grafana/data';
import { ThemeContextProvider } from '@grafana/ui';

describe('MyPanel', () => {
  it('renders correctly in dark theme', () => {
    const darkTheme = createTheme({
      colors: { mode: 'dark' },
    });

    const { container } = render(
      <ThemeContextProvider value={darkTheme}>
        <MyPanel {...mockProps} />
      </ThemeContextProvider>
    );

    expect(container).toMatchSnapshot();
  });

  it('renders correctly in light theme', () => {
    const lightTheme = createTheme({
      colors: { mode: 'light' },
    });

    const { container } = render(
      <ThemeContextProvider value={lightTheme}>
        <MyPanel {...mockProps} />
      </ThemeContextProvider>
    );

    expect(container).toMatchSnapshot();
  });
});
```

---

## 6. Key Source Files & Directories

### 6.1 Theme Definition Files

```
packages/grafana-data/src/themes/
├── createTheme.ts          # Main theme factory function
├── createColors.ts         # Color palette generation
├── createSpacing.ts        # Spacing scale
├── createTypography.ts     # Typography definitions
├── createShadows.ts        # Shadow definitions
├── createBreakpoints.ts    # Responsive breakpoints
├── createComponents.ts     # Component-specific tokens
├── types.ts                # GrafanaTheme2 interface
├── palette.ts              # Base color palettes
└── index.ts                # Public exports
```

### 6.2 Theme Context & Hooks

```
packages/grafana-ui/src/themes/
├── ThemeContext.tsx        # ThemeContextProvider, useTheme2, useStyles2
├── GlobalStyles.tsx        # Global CSS reset and base styles
├── stylesFactory.ts        # Style utilities
└── index.ts                # Public exports
```

### 6.3 Preferences Management

```
public/app/
├── core/
│   ├── services/
│   │   ├── PreferencesService.ts    # Frontend preferences service
│   │   └── context_srv.ts           # User context including theme
│   ├── components/
│   │   └── AppChrome/
│   │       └── AppChrome.tsx        # Root theme provider
│   └── utils/
│       ├── ConfigProvider.tsx       # Config and theme initialization
│       └── themeUtils.ts            # Theme utility functions
│
├── features/
│   └── profile/
│       ├── UserPreferencesForm.tsx  # Theme selection UI
│       └── state/
│           └── reducers.ts          # Preferences state management
│
└── types/
    └── preferences.ts               # Preferences type definitions
```

### 6.4 Backend Preferences

```
pkg/
├── api/
│   └── preferences.go              # Preferences API endpoints
│
├── services/
│   └── preference/
│       ├── pref.go                 # Preferences service interface
│       └── prefimpl/
│           └── pref.go             # Preferences implementation
│
└── models/
    └── preferences.go              # Preferences data models
```

### 6.5 Configuration

```
conf/
├── defaults.ini                    # Default configuration
└── sample.ini                      # Sample configuration

# Theme-related config section:
# [users]
# default_theme = dark
```

---

## 7. Minimal Implementation for Runtime Theme Toggling

### 7.1 Required Components

To implement Grafana-style runtime theme toggling, you need:

```typescript
// 1. THEME TYPE DEFINITIONS
// ─────────────────────────────────────────────────────────────
type ThemeMode = 'light' | 'dark' | 'system';

interface Theme {
  name: string;
  isDark: boolean;
  isLight: boolean;
  colors: {
    mode: 'light' | 'dark';
    background: {
      canvas: string;
      primary: string;
      secondary: string;
    };
    text: {
      primary: string;
      secondary: string;
      disabled: string;
    };
    border: {
      weak: string;
      medium: string;
      strong: string;
    };
    // ... other color tokens
  };
  spacing: (factor: number) => string;
  // ... other tokens
}

// 2. THEME FACTORY
// ─────────────────────────────────────────────────────────────
function createTheme(mode: 'light' | 'dark'): Theme {
  const isDark = mode === 'dark';

  return {
    name: isDark ? 'Dark' : 'Light',
    isDark,
    isLight: !isDark,
    colors: {
      mode,
      background: isDark
        ? {
            canvas: '#111217',
            primary: '#181b1f',
            secondary: '#22252b',
          }
        : {
            canvas: '#f4f5f5',
            primary: '#ffffff',
            secondary: '#f4f5f5',
          },
      text: isDark
        ? {
            primary: 'rgba(255,255,255,0.9)',
            secondary: 'rgba(255,255,255,0.6)',
            disabled: 'rgba(255,255,255,0.38)',
          }
        : {
            primary: 'rgba(0,0,0,0.87)',
            secondary: 'rgba(0,0,0,0.54)',
            disabled: 'rgba(0,0,0,0.38)',
          },
      border: isDark
        ? {
            weak: 'rgba(255,255,255,0.08)',
            medium: 'rgba(255,255,255,0.15)',
            strong: 'rgba(255,255,255,0.25)',
          }
        : {
            weak: 'rgba(0,0,0,0.08)',
            medium: 'rgba(0,0,0,0.15)',
            strong: 'rgba(0,0,0,0.25)',
          },
    },
    spacing: (factor) => `${factor * 8}px`,
  };
}

// 3. THEME CONTEXT
// ─────────────────────────────────────────────────────────────
import {
  createContext,
  useContext,
  useMemo,
  useState,
  useCallback,
} from 'react';

interface ThemeContextValue {
  theme: Theme;
  themeMode: ThemeMode;
  setThemeMode: (mode: ThemeMode) => Promise<void>;
}

const ThemeContext =
  createContext<ThemeContextValue | null>(null);

export function useTheme(): Theme {
  const ctx = useContext(ThemeContext);
  if (!ctx)
    throw new Error(
      'useTheme must be used within ThemeProvider'
    );
  return ctx.theme;
}

export function useThemeMode(): [
  ThemeMode,
  (mode: ThemeMode) => Promise<void>
] {
  const ctx = useContext(ThemeContext);
  if (!ctx)
    throw new Error(
      'useThemeMode must be used within ThemeProvider'
    );
  return [ctx.themeMode, ctx.setThemeMode];
}

// 4. THEME PROVIDER
// ─────────────────────────────────────────────────────────────
interface ThemeProviderProps {
  children: React.ReactNode;
  initialMode?: ThemeMode;
}

export function ThemeProvider({
  children,
  initialMode = 'system',
}: ThemeProviderProps) {
  const [themeMode, setThemeModeState] =
    useState<ThemeMode>(initialMode);

  // Resolve effective theme
  const effectiveMode = useMemo(() => {
    if (themeMode === 'system') {
      return window.matchMedia(
        '(prefers-color-scheme: dark)'
      ).matches
        ? 'dark'
        : 'light';
    }
    return themeMode;
  }, [themeMode]);

  // Create theme object
  const theme = useMemo(
    () => createTheme(effectiveMode),
    [effectiveMode]
  );

  // Theme setter with persistence
  const setThemeMode = useCallback(
    async (mode: ThemeMode) => {
      // Update state immediately for instant feedback
      setThemeModeState(mode);

      // Persist to backend
      await fetch('/api/user/preferences', {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ theme: mode }),
      });

      // Update DOM attribute for CSS variables
      const effective =
        mode === 'system'
          ? window.matchMedia(
              '(prefers-color-scheme: dark)'
            ).matches
            ? 'dark'
            : 'light'
          : mode;
      document.documentElement.setAttribute(
        'data-theme',
        effective
      );
    },
    []
  );

  // Listen for system theme changes when mode is 'system'
  useEffect(() => {
    if (themeMode !== 'system') return;

    const mediaQuery = window.matchMedia(
      '(prefers-color-scheme: dark)'
    );
    const handler = () => setThemeModeState('system'); // Trigger re-render

    mediaQuery.addEventListener('change', handler);
    return () =>
      mediaQuery.removeEventListener('change', handler);
  }, [themeMode]);

  const value = useMemo(
    () => ({
      theme,
      themeMode,
      setThemeMode,
    }),
    [theme, themeMode, setThemeMode]
  );

  return (
    <ThemeContext.Provider value={value}>
      <div data-theme={effectiveMode}>{children}</div>
    </ThemeContext.Provider>
  );
}

// 5. STYLED HOOK (useStyles equivalent)
// ─────────────────────────────────────────────────────────────
type StylesCreator<T> = (theme: Theme) => T;

export function useStyles<T>(
  getStyles: StylesCreator<T>
): T {
  const theme = useTheme();
  return useMemo(
    () => getStyles(theme),
    [theme, getStyles]
  );
}

// 6. THEME TOGGLE COMPONENT
// ─────────────────────────────────────────────────────────────
export function ThemeToggle() {
  const [themeMode, setThemeMode] = useThemeMode();

  return (
    <select
      value={themeMode}
      onChange={(e) =>
        setThemeMode(e.target.value as ThemeMode)
      }
    >
      <option value='light'>Light</option>
      <option value='dark'>Dark</option>
      <option value='system'>System</option>
    </select>
  );
}

// 7. CSS VARIABLES (global styles)
// ─────────────────────────────────────────────────────────────
const globalStyles = `
  :root {
    --color-bg-canvas: #111217;
    --color-bg-primary: #181b1f;
    --color-text-primary: rgba(255, 255, 255, 0.9);
    --color-border-weak: rgba(255, 255, 255, 0.08);
  }
  
  [data-theme="light"] {
    --color-bg-canvas: #f4f5f5;
    --color-bg-primary: #ffffff;
    --color-text-primary: rgba(0, 0, 0, 0.87);
    --color-border-weak: rgba(0, 0, 0, 0.08);
  }
`;

// 8. PREFERENCES API SERVICE
// ─────────────────────────────────────────────────────────────
interface PreferencesService {
  getPreferences(): Promise<{ theme: ThemeMode }>;
  updatePreferences(prefs: {
    theme?: ThemeMode;
  }): Promise<void>;
}

const preferencesService: PreferencesService = {
  async getPreferences() {
    const response = await fetch('/api/user/preferences');
    return response.json();
  },

  async updatePreferences(prefs) {
    await fetch('/api/user/preferences', {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(prefs),
    });
  },
};
```

### 7.2 Usage Example

```typescript
// App.tsx
import {
  ThemeProvider,
  ThemeToggle,
  useTheme,
  useStyles,
} from './theme';
import { css } from '@emotion/css';

function App() {
  return (
    <ThemeProvider initialMode='system'>
      <Header />
      <MainContent />
    </ThemeProvider>
  );
}

function Header() {
  const styles = useStyles(getHeaderStyles);

  return (
    <header className={styles.header}>
      <h1 className={styles.title}>My App</h1>
      <ThemeToggle />
    </header>
  );
}

const getHeaderStyles = (theme: Theme) => ({
  header: css({
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: theme.spacing(2),
    backgroundColor: theme.colors.background.primary,
    borderBottom: `1px solid ${theme.colors.border.weak}`,
  }),
  title: css({
    color: theme.colors.text.primary,
    margin: 0,
  }),
});
```

---

## 8. Summary: Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         GRAFANA THEMING ARCHITECTURE                         │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                              PERSISTENCE LAYER                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌────────────┐│
│  │ User Prefs   │ >> │ Team Prefs   │ >> │ Org Prefs    │ >> │ Server Cfg ││
│  │ (highest)    │    │              │    │              │    │ (lowest)   ││
│  └──────────────┘    └──────────────┘    └──────────────┘    └────────────┘│
│         │                                                                    │
│         ▼                                                                    │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                    Effective Theme Resolution                         │  │
│  │  resolveTheme(user, team, org, server) → 'light' | 'dark' | 'system' │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              THEME DEFINITION                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐│
│  │                         createTheme(mode)                              ││
│  │                                                                        ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ││
│  │  │   Colors    │  │   Spacing   │  │ Typography  │  │   Shadows   │  ││
│  │  │ background  │  │ xs: 4px     │  │ fontFamily  │  │ sm, md, lg  │  ││
│  │  │ text        │  │ sm: 8px     │  │ fontSize    │  │             │  ││
│  │  │ border      │  │ md: 16px    │  │ fontWeight  │  │             │  ││
│  │  │ action      │  │ lg: 24px    │  │ lineHeight  │  │             │  ││
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  ││
│  │                                                                        ││
│  │                    → GrafanaTheme2 object                              ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              REACT CONTEXT                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐│
│  │                      ThemeContextProvider                              ││
│  │                                                                        ││
│  │    <ThemeContext.Provider value={theme}>                               ││
│  │      <div data-theme={theme.isDark ? 'dark' : 'light'}>                ││
│  │        {children}                                                      ││
│  │      </div>                                                            ││
│  │    </ThemeContext.Provider>                                            ││
│  │                                                                        ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                      │                                       │
│                    ┌─────────────────┼─────────────────┐                    │
│                    ▼                 ▼                 ▼                    │
│            ┌─────────────┐   ┌─────────────┐   ┌─────────────┐             │
│            │ useTheme2() │   │ useStyles2()│   │   Panel     │             │
│            │             │   │             │   │  Plugins    │             │
│            └─────────────┘   └─────────────┘   └─────────────┘             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CSS APPLICATION                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐│
│  │                         CSS Variables                                  ││
│  │                                                                        ││
│  │  [data-theme="dark"] {                                                 ││
│  │    --color-background-canvas: #111217;                                 ││
│  │    --color-text-primary: rgba(255, 255, 255, 0.9);                     ││
│  │  }                                                                     ││
│  │                                                                        ││
│  │  [data-theme="light"] {                                                ││
│  │    --color-background-canvas: #f4f5f5;                                 ││
│  │    --color-text-primary: rgba(0, 0, 0, 0.87);                          ││
│  │  }                                                                     ││
│  │                                                                        ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                      │                                       │
│                                      ▼                                       │
│  ┌────────────────────────────────────────────────────────────────────────┐│
│  │                      Emotion CSS-in-JS                                 ││
│  │                                                                        ││
│  │  const styles = useStyles2((theme) => ({                               ││
│  │    container: css({                                                    ││
│  │      backgroundColor: theme.colors.background.primary,                 ││
│  │      color: theme.colors.text.primary,                                 ││
│  │    }),                                                                 ││
│  │  }));                                                                  ││
│  │                                                                        ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Key Takeaways

1. **Theme State**: Three modes (`light`, `dark`, `system`) with `system` following OS preference
2. **Design Tokens**: Comprehensive token system covering colors, spacing, typography, shadows
3. **Precedence**: User > Team > Org > Server defaults
4. **React Integration**: Context + hooks (`useTheme2`, `useStyles2`) for theme access
5. **CSS Variables**: Enable instant theme switching without full re-render
6. **Plugin Compatibility**: Plugins use same hooks, ensuring automatic theme support
7. **Persistence**: REST API for preferences with immediate frontend update

The architecture ensures:

- **Instant visual feedback** on theme change
- **Persistence** across sessions
- **Hierarchical defaults** for organizational control
- **Plugin compatibility** through standardized hooks
- **Efficient re-rendering** via React context and memoization
