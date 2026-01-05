# VS Code Theme Switching Architecture Analysis

> **Focus**: Runtime theme toggling in Electron/TypeScript architecture

---

## Table of Contents

1. [Theme Model](#1-theme-model-color-themes--token-colors)
2. [Persistence](#2-persistence-settings-storage--restoration)
3. [Runtime Application](#3-runtime-application-css-variables--dom-styling)
4. [Call Stacks](#4-call-stacks)
5. [Extension Interface](#5-extension-interface)
6. [Key Source Files](#6-key-source-files-summary)
7. [Minimal Architecture for Well-Correlation App](#7-minimal-architecture-for-well-correlation-app)

---

## 1. Theme Model: Color Themes & Token Colors

### Theme Data Structure

VS Code represents themes through JSON files with three main sections:

```json
{
  "name": "Theme Name",
  "type": "dark",
  "colors": {
    "editor.background": "#1e1e1e",
    "sideBar.background": "#252526",
    "activityBar.background": "#333333",
    "statusBar.background": "#007acc"
  },
  "tokenColors": [
    {
      "scope": ["comment", "punctuation.definition.comment"],
      "settings": { "foreground": "#6A9955" }
    },
    {
      "scope": "keyword",
      "settings": { "foreground": "#569CD6", "fontStyle": "bold" }
    }
  ],
  "semanticTokenColors": {
    "variable.readonly": "#4FC1FF",
    "function.declaration": "#DCDCAA"
  }
}
```

### Color Categories

| Section | Purpose |
|---------|---------|
| `colors` | 600+ workbench UI color keys (sidebar, statusbar, editor chrome) |
| `tokenColors` | TextMate-style syntax highlighting rules |
| `semanticTokenColors` | Language-server-aware token colors (LSP semantic tokens) |

### Light/Dark Distinction

Themes declare their category via the `uiTheme` property in `package.json`:

| Value | Category |
|-------|----------|
| `vs` | Light theme |
| `vs-dark` | Dark theme |
| `hc-black` | High contrast dark |
| `hc-light` | High contrast light |

The theme `type` property influences:
- Default fallback colors when a key is missing
- Editor background/foreground defaults
- Selection/highlight color defaults

### Key Source Files for Theme Model

```
src/vs/workbench/services/themes/
├── browser/
│   └── workbenchThemeService.ts    # Main theme service implementation
├── common/
│   ├── colorThemeData.ts           # Theme data model (ColorThemeData class)
│   ├── colorThemeSchema.ts         # JSON schema validation
│   ├── themeConfiguration.ts       # Settings configuration
│   └── workbenchThemeService.ts    # IWorkbenchThemeService interface
└── test/

src/vs/platform/theme/
├── common/
│   ├── themeService.ts             # IThemeService base interface
│   ├── colorRegistry.ts            # Color key registry (registerColor)
│   └── styler.ts                   # Component styling utilities
└── browser/
    └── defaultStyles.ts            # Default component styles
```

---

## 2. Persistence: Settings Storage & Restoration

### Storage Location

```json
// User settings locations:
// Linux:   ~/.config/Code/User/settings.json
// macOS:   ~/Library/Application Support/Code/User/settings.json
// Windows: %APPDATA%\Code\User\settings.json

{
  "workbench.colorTheme": "Default Dark+",
  "workbench.preferredDarkColorTheme": "One Dark Pro",
  "workbench.preferredLightColorTheme": "Default Light+",
  "window.autoDetectColorScheme": true
}
```

### Settings Keys

| Setting | Purpose |
|---------|---------|
| `workbench.colorTheme` | Currently active theme |
| `workbench.preferredDarkColorTheme` | Theme for dark mode (when auto-detect enabled) |
| `workbench.preferredLightColorTheme` | Theme for light mode (when auto-detect enabled) |
| `workbench.colorCustomizations` | Per-theme color overrides |
| `editor.tokenColorCustomizations` | Per-theme token overrides |

### Per-Theme Customizations

```json
{
  "workbench.colorCustomizations": {
    "[Default Dark+]": {
      "sideBar.background": "#223355",
      "editor.background": "#1a1a2e"
    },
    "[Monokai]": {
      "statusBar.background": "#ff6b6b"
    }
  },
  "editor.tokenColorCustomizations": {
    "[Default Dark+]": {
      "comments": "#6a9955",
      "strings": "#ce9178"
    }
  }
}
```

### Restoration Flow

```
Application Start
  └─ settings.json 
     └─ ConfigurationService.getValue('workbench.colorTheme')
        └─ WorkbenchThemeService.restoreColorTheme()
           └─ Theme applied to DOM
```

---

## 3. Runtime Application: CSS Variables & DOM Styling

### CSS Variable Injection

VS Code injects theme colors as CSS custom properties on the `<body>` element:

```css
body {
  --vscode-editor-background: #1e1e1e;
  --vscode-editor-foreground: #d4d4d4;
  --vscode-sideBar-background: #252526;
  --vscode-activityBar-background: #333333;
  --vscode-statusBar-background: #007acc;
  --vscode-button-background: #0e639c;
  --vscode-button-foreground: #ffffff;
  --vscode-input-background: #3c3c3c;
  --vscode-input-foreground: #cccccc;
  --vscode-input-border: #3c3c3c;
  /* ... 600+ variables */
}
```

### Theming Participant Pattern

Components register styling callbacks via `registerThemingParticipant`:

```typescript
// src/vs/platform/theme/common/themeService.ts

import { registerThemingParticipant } from 'vs/platform/theme/common/themeService';
import { editorBackground, editorForeground } from 'vs/platform/theme/common/colorRegistry';

registerThemingParticipant((theme: IColorTheme, collector: ICssStyleCollector) => {
  // Get color from theme (with fallback)
  const editorBg = theme.getColor(editorBackground);
  const editorFg = theme.getColor(editorForeground);
  
  if (editorBg) {
    collector.addRule(`.monaco-editor { background-color: ${editorBg}; }`);
  }
  if (editorFg) {
    collector.addRule(`.monaco-editor { color: ${editorFg}; }`);
  }
});
```

### Theme Change Propagation

When theme changes:

1. **CSS Variables Updated**: All `--vscode-*` properties updated on `<body>`
2. **Theming Participants Re-invoked**: All registered callbacks regenerate CSS rules
3. **Style Tag Updated**: Collected CSS rules injected into `<style id="vscode-theme-styles">`
4. **Browser Repaints**: Affected elements automatically repaint

### Token Colorization (Editor-Specific)

Editor tokens use a separate path:

```
Theme Change
  └─ TokenizationRegistry updated
     └─ TokenThemeData recomputed
        └─ Grammar → Color mappings rebuilt
           └─ Monaco ViewLines re-renders
              └─ Each token span gets new color
```

---

## 4. Call Stacks

### Startup Restoration Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│ 1. Electron main process starts                                     │
│    └─ src/vs/code/electron-main/main.ts                             │
├─────────────────────────────────────────────────────────────────────┤
│ 2. Workbench initializes                                            │
│    └─ src/vs/workbench/browser/workbench.ts                         │
│       └─ startup() → initServices()                                 │
├─────────────────────────────────────────────────────────────────────┤
│ 3. WorkbenchThemeService created                                    │
│    └─ src/vs/workbench/services/themes/browser/                     │
│       workbenchThemeService.ts                                      │
│       └─ constructor()                                              │
│          └─ this.restoreColorTheme()                                │
├─────────────────────────────────────────────────────────────────────┤
│ 4. Configuration read                                               │
│    └─ configurationService.getValue('workbench.colorTheme')         │
│       └─ Returns "Default Dark+" (or stored value)                  │
├─────────────────────────────────────────────────────────────────────┤
│ 5. Theme data loaded                                                │
│    └─ ColorThemeData.createLoadedEmptyTheme()                       │
│    └─ ColorThemeData.fromExtensionTheme() or fromStorageData()      │
│       └─ Parses JSON, resolves includes, builds color map           │
├─────────────────────────────────────────────────────────────────────┤
│ 6. CSS variables injected                                           │
│    └─ this.applyTheme(colorTheme)                                   │
│       └─ this.updateDynamicCSSRules(theme)                          │
│          └─ Iterates colorRegistry, sets --vscode-* vars            │
│       └─ this.updateTokenColorCustomizations(theme)                 │
├─────────────────────────────────────────────────────────────────────┤
│ 7. Event fired                                                      │
│    └─ this._onDidColorThemeChange.fire(colorTheme)                  │
│       └─ All subscribers re-render                                  │
└─────────────────────────────────────────────────────────────────────┘
```

### Runtime Toggle Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│ 1. User triggers theme change                                       │
│    └─ Command Palette: "Preferences: Color Theme"                   │
│    └─ OR: Settings UI toggle                                        │
│    └─ OR: workbench.action.selectTheme command                      │
│    └─ OR: Keyboard shortcut (if bound)                              │
├─────────────────────────────────────────────────────────────────────┤
│ 2. Command handler invoked                                          │
│    └─ src/vs/workbench/contrib/themes/browser/                      │
│       themes.contribution.ts                                        │
│       └─ SelectColorThemeAction.run()                               │
│          └─ Shows QuickPick with available themes                   │
├─────────────────────────────────────────────────────────────────────┤
│ 3. User selects theme from picker                                   │
│    └─ themeService.setColorTheme(themeId, settingsTarget)           │
│       └─ settingsTarget: 'user' | 'workspace' | 'memory'            │
├─────────────────────────────────────────────────────────────────────┤
│ 4. Theme loaded (if not cached)                                     │
│    └─ ColorThemeData.ensureLoaded()                                 │
│       └─ Fetches JSON from extension/builtin path                   │
│       └─ Resolves "include" references (theme inheritance)          │
│       └─ Builds tokenColors → TokenThemeData                        │
├─────────────────────────────────────────────────────────────────────┤
│ 5. Settings updated (persisted)                                     │
│    └─ configurationService.updateValue(                             │
│         'workbench.colorTheme',                                     │
│         theme.settingsId,                                           │
│         ConfigurationTarget.USER                                    │
│       )                                                             │
│       └─ Writes to settings.json asynchronously                     │
├─────────────────────────────────────────────────────────────────────┤
│ 6. Theme applied to DOM                                             │
│    └─ this.applyTheme(theme)                                        │
│       └─ updateDynamicCSSRules(theme)                               │
│          └─ document.body.style.setProperty(                        │
│               '--vscode-editor-background',                         │
│               theme.getColor(editorBackground).toString()           │
│             )                                                       │
│       └─ runThemingParticipants(theme, cssCollector)                │
│          └─ Regenerates component-specific CSS                      │
│       └─ injectStyleSheet(collectedCSS)                             │
├─────────────────────────────────────────────────────────────────────┤
│ 7. Event broadcast to all subscribers                               │
│    └─ this._onDidColorThemeChange.fire(theme)                       │
│       └─ Monaco editor updates tokenization colors                  │
│       └─ Tree views re-render icons                                 │
│       └─ Status bar updates background                              │
│       └─ Activity bar icons update                                  │
│       └─ All subscribed components repaint                          │
└─────────────────────────────────────────────────────────────────────┘
```

### OS Theme Auto-Detection Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│ 1. OS theme changes (e.g., macOS Dark Mode toggle)                  │
│    └─ Electron detects via nativeTheme.on('updated')                │
├─────────────────────────────────────────────────────────────────────┤
│ 2. IPC message sent to renderer                                     │
│    └─ 'vscode:osColorSchemeChanged'                                 │
├─────────────────────────────────────────────────────────────────────┤
│ 3. ThemeService handles OS change                                   │
│    └─ Check: window.autoDetectColorScheme === true?                 │
│       └─ Yes: Switch to preferredDark/LightColorTheme               │
│       └─ No: Ignore                                                 │
├─────────────────────────────────────────────────────────────────────┤
│ 4. Theme applied (same as runtime toggle)                           │
│    └─ setColorTheme(preferredTheme, 'memory')                       │
│       └─ Note: 'memory' target = don't persist to settings          │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. Extension Interface

### Theme Extension package.json

```json
{
  "name": "my-awesome-theme",
  "displayName": "My Awesome Theme",
  "version": "1.0.0",
  "publisher": "my-publisher",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Themes"],
  "contributes": {
    "themes": [
      {
        "label": "My Dark Theme",
        "uiTheme": "vs-dark",
        "path": "./themes/my-dark-color-theme.json"
      },
      {
        "label": "My Light Theme",
        "uiTheme": "vs",
        "path": "./themes/my-light-color-theme.json"
      },
      {
        "label": "My High Contrast",
        "uiTheme": "hc-black",
        "path": "./themes/my-hc-color-theme.json"
      }
    ]
  }
}
```

### Theme JSON File Structure

```json
{
  "$schema": "vscode://schemas/color-theme",
  "name": "My Dark Theme",
  "type": "dark",
  "include": "./base-theme.json",
  "colors": {
    "editor.background": "#1a1a2e",
    "editor.foreground": "#eaeaea",
    "activityBar.background": "#16213e",
    "activityBar.foreground": "#e94560",
    "sideBar.background": "#1a1a2e",
    "sideBar.foreground": "#c9c9c9",
    "statusBar.background": "#0f3460",
    "statusBar.foreground": "#ffffff",
    "titleBar.activeBackground": "#16213e",
    "titleBar.activeForeground": "#ffffff"
  },
  "tokenColors": [
    {
      "name": "Comments",
      "scope": ["comment", "punctuation.definition.comment"],
      "settings": {
        "foreground": "#6a6a8a",
        "fontStyle": "italic"
      }
    },
    {
      "name": "Keywords",
      "scope": ["keyword", "storage.type", "storage.modifier"],
      "settings": {
        "foreground": "#e94560"
      }
    },
    {
      "name": "Strings",
      "scope": ["string", "string.quoted"],
      "settings": {
        "foreground": "#a7c957"
      }
    },
    {
      "name": "Functions",
      "scope": ["entity.name.function", "support.function"],
      "settings": {
        "foreground": "#4fc3f7"
      }
    }
  ],
  "semanticHighlighting": true,
  "semanticTokenColors": {
    "variable.readonly": "#80cbc4",
    "property.readonly": "#80cbc4",
    "function.declaration": "#82aaff",
    "class.declaration": "#ffcb6b"
  }
}
```

### Theme Registration Flow

```
Extension Installation
  └─ ExtensionService scans package.json
     └─ Finds contributes.themes array
        └─ For each theme entry:
           └─ ThemeRegistry.registerTheme({
                id: generateThemeId(extension, theme),
                label: theme.label,
                uiTheme: theme.uiTheme,
                path: resolvePath(extension, theme.path),
                extensionData: extension
              })
           └─ Creates ColorThemeData stub (lazy-loaded)
           └─ Adds to availableThemes list
           └─ Theme appears in picker immediately
```

### Theme Loading (Lazy)

```
User Selects Theme
  └─ ColorThemeData.ensureLoaded()
     └─ Check: isLoaded === true?
        └─ Yes: Return cached data
        └─ No: Continue loading
     └─ fs.readFile(themePath)
     └─ JSON.parse(content)
     └─ Resolve "include" chain (recursive)
        └─ Merge base theme colors
        └─ Override with current theme colors
     └─ Validate against colorThemeSchema
     └─ Build TokenThemeData from tokenColors
     └─ Cache parsed data
     └─ Mark isLoaded = true
```

---

## 6. Key Source Files Summary

### Core Theme Service

| File | Purpose |
|------|---------|
| `src/vs/workbench/services/themes/browser/workbenchThemeService.ts` | Main service: theme loading, application, events |
| `src/vs/workbench/services/themes/common/colorThemeData.ts` | `ColorThemeData` class: theme model, JSON parsing |
| `src/vs/workbench/services/themes/common/themeConfiguration.ts` | Settings schema, defaults, validation |
| `src/vs/workbench/services/themes/common/workbenchThemeService.ts` | `IWorkbenchThemeService` interface definition |

### Platform Theme Infrastructure

| File | Purpose |
|------|---------|
| `src/vs/platform/theme/common/colorRegistry.ts` | Color key registration (`registerColor`) |
| `src/vs/platform/theme/common/themeService.ts` | `IThemeService` interface, `registerThemingParticipant` |
| `src/vs/platform/theme/browser/defaultStyles.ts` | Default component styles |
| `src/vs/platform/theme/common/styler.ts` | Styling utility functions |

### Commands & UI

| File | Purpose |
|------|---------|
| `src/vs/workbench/contrib/themes/browser/themes.contribution.ts` | Commands: `selectTheme`, toggle actions |
| `src/vs/workbench/contrib/preferences/browser/settingsEditor2.ts` | Settings UI theme picker |

### Editor Tokenization

| File | Purpose |
|------|---------|
| `src/vs/editor/common/languages/supports/tokenization.ts` | Token colorization for editor |
| `src/vs/editor/standalone/common/themes.ts` | Standalone editor themes |

---

## 7. Minimal Architecture for Well-Correlation App

For a well-correlation-style application with **global theme + per-view styling**:

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Application                              │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐                     │
│  │  Theme Service  │───▶│  Color Registry │                     │
│  │  (Svelte Store) │    │  (Token Defs)   │                     │
│  └────────┬────────┘    └─────────────────┘                     │
│           │                                                      │
│           ▼                                                      │
│  ┌─────────────────┐                                            │
│  │  CSS Variables  │ ◀── Injected on :root                      │
│  │  (--app-*)      │                                            │
│  └────────┬────────┘                                            │
│           │                                                      │
│           ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    UI Components                            ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    ││
│  │  │ Sidebar  │  │  Chart   │  │ WellLog  │  │  Panel   │    ││
│  │  │          │  │  View    │  │  Track   │  │          │    ││
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘    ││
│  │       │              │             │             │          ││
│  │       └──────────────┴─────────────┴─────────────┘          ││
│  │                          │                                   ││
│  │                  Uses CSS vars OR                            ││
│  │                  $currentTheme store                         ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Core Implementation

#### 1. Theme Model (`src/lib/theme/types.ts`)

```typescript
export type ThemeType = 'light' | 'dark';

export interface ITheme {
  id: string;
  name: string;
  type: ThemeType;
  colors: Record<string, string>;
}

// Define all color tokens upfront (like VS Code's colorRegistry)
export const COLOR_TOKENS = {
  // App-level colors
  'app.background': { light: '#ffffff', dark: '#1e1e1e' },
  'app.foreground': { light: '#333333', dark: '#cccccc' },
  'app.border': { light: '#e0e0e0', dark: '#404040' },
  
  // Panel colors
  'panel.background': { light: '#f5f5f5', dark: '#252526' },
  'panel.foreground': { light: '#333333', dark: '#cccccc' },
  'panel.border': { light: '#e0e0e0', dark: '#3c3c3c' },
  'panel.headerBackground': { light: '#e8e8e8', dark: '#2d2d2d' },
  
  // Sidebar colors
  'sidebar.background': { light: '#f3f3f3', dark: '#252526' },
  'sidebar.foreground': { light: '#333333', dark: '#cccccc' },
  'sidebar.activeItem': { light: '#0066cc', dark: '#4fc3f7' },
  
  // Chart/visualization colors
  'chart.background': { light: '#ffffff', dark: '#1e1e1e' },
  'chart.gridLine': { light: '#e0e0e0', dark: '#404040' },
  'chart.axisLine': { light: '#333333', dark: '#808080' },
  'chart.axisLabel': { light: '#666666', dark: '#999999' },
  
  // Well log specific colors
  'wellLog.trackBackground': { light: '#fafafa', dark: '#2d2d2d' },
  'wellLog.trackBorder': { light: '#cccccc', dark: '#4a4a4a' },
  'wellLog.depthScale': { light: '#333333', dark: '#cccccc' },
  'wellLog.curveDefault': { light: '#0066cc', dark: '#4fc3f7' },
  'wellLog.curveSecondary': { light: '#cc6600', dark: '#ffb74d' },
  'wellLog.fillPositive': { light: '#4caf50', dark: '#81c784' },
  'wellLog.fillNegative': { light: '#f44336', dark: '#e57373' },
  
  // Status/feedback colors
  'status.info': { light: '#0288d1', dark: '#29b6f6' },
  'status.success': { light: '#388e3c', dark: '#66bb6a' },
  'status.warning': { light: '#f57c00', dark: '#ffa726' },
  'status.error': { light: '#d32f2f', dark: '#ef5350' },
  
  // Interactive elements
  'button.background': { light: '#0066cc', dark: '#0e639c' },
  'button.foreground': { light: '#ffffff', dark: '#ffffff' },
  'button.hoverBackground': { light: '#0052a3', dark: '#1177bb' },
  'input.background': { light: '#ffffff', dark: '#3c3c3c' },
  'input.foreground': { light: '#333333', dark: '#cccccc' },
  'input.border': { light: '#cccccc', dark: '#3c3c3c' },
} as const;

export type ColorToken = keyof typeof COLOR_TOKENS;
```

#### 2. Theme Service (`src/lib/theme/themeService.ts`)

```typescript
import { writable, derived, get } from 'svelte/store';
import type { ITheme, ThemeType, ColorToken } from './types';
import { COLOR_TOKENS } from './types';

// Built-in themes
const BUILTIN_THEMES: ITheme[] = [
  {
    id: 'default-light',
    name: 'Default Light',
    type: 'light',
    colors: Object.fromEntries(
      Object.entries(COLOR_TOKENS).map(([key, val]) => [key, val.light])
    ),
  },
  {
    id: 'default-dark',
    name: 'Default Dark',
    type: 'dark',
    colors: Object.fromEntries(
      Object.entries(COLOR_TOKENS).map(([key, val]) => [key, val.dark])
    ),
  },
];

// Stores
const themes = writable<Map<string, ITheme>>(
  new Map(BUILTIN_THEMES.map(t => [t.id, t]))
);
const currentThemeId = writable<string>('default-dark');

// Derived store for current theme
export const currentTheme = derived(
  [currentThemeId, themes],
  ([$id, $themes]) => $themes.get($id) ?? BUILTIN_THEMES[1]
);

// Derived store for theme type (for conditional styling)
export const themeType = derived(currentTheme, $theme => $theme.type);

// Available themes list
export const availableThemes = derived(themes, $themes => 
  Array.from($themes.values())
);

/**
 * Set the active theme
 */
export function setTheme(themeId: string): void {
  const $themes = get(themes);
  if (!$themes.has(themeId)) {
    console.warn(`Theme "${themeId}" not found`);
    return;
  }
  
  currentThemeId.set(themeId);
  persistTheme(themeId);
  applyThemeToDom(themeId);
}

/**
 * Toggle between light and dark themes
 */
export function toggleTheme(): void {
  const $current = get(currentTheme);
  const $themes = get(themes);
  
  // Find a theme of opposite type
  const oppositeType: ThemeType = $current.type === 'dark' ? 'light' : 'dark';
  const oppositeTheme = Array.from($themes.values())
    .find(t => t.type === oppositeType);
  
  if (oppositeTheme) {
    setTheme(oppositeTheme.id);
  }
}

/**
 * Register a custom theme
 */
export function registerTheme(theme: ITheme): void {
  themes.update($themes => {
    $themes.set(theme.id, theme);
    return $themes;
  });
}

/**
 * Get a specific color from current theme
 */
export function getColor(token: ColorToken): string {
  const $theme = get(currentTheme);
  return $theme.colors[token] ?? COLOR_TOKENS[token][$theme.type];
}

/**
 * Apply theme colors to DOM as CSS variables
 */
function applyThemeToDom(themeId: string): void {
  const $themes = get(themes);
  const theme = $themes.get(themeId);
  if (!theme) return;
  
  const root = document.documentElement;
  
  // Set theme type attribute (for CSS selectors)
  root.setAttribute('data-theme', theme.type);
  
  // Inject all color tokens as CSS variables
  for (const [token, value] of Object.entries(theme.colors)) {
    // Convert token name to CSS variable format
    // e.g., "wellLog.trackBackground" → "--wellLog-trackBackground"
    const cssVar = `--${token.replace(/\./g, '-')}`;
    root.style.setProperty(cssVar, value);
  }
  
  // Dispatch custom event for components that need programmatic updates
  window.dispatchEvent(new CustomEvent('themechange', { 
    detail: { theme } 
  }));
}

/**
 * Persist theme selection to localStorage
 */
function persistTheme(themeId: string): void {
  try {
    localStorage.setItem('app.colorTheme', themeId);
  } catch (e) {
    console.warn('Failed to persist theme:', e);
  }
}

/**
 * Restore theme from localStorage on startup
 */
export function restoreTheme(): void {
  try {
    const stored = localStorage.getItem('app.colorTheme');
    const themeId = stored ?? 'default-dark';
    
    currentThemeId.set(themeId);
    applyThemeToDom(themeId);
  } catch (e) {
    console.warn('Failed to restore theme:', e);
    applyThemeToDom('default-dark');
  }
}

/**
 * Listen for OS theme changes (optional)
 */
export function setupOsThemeListener(): () => void {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  
  const handler = (e: MediaQueryListEvent) => {
    const prefersDark = e.matches;
    const $themes = get(themes);
    
    // Find appropriate theme for OS preference
    const targetTheme = Array.from($themes.values())
      .find(t => t.type === (prefersDark ? 'dark' : 'light'));
    
    if (targetTheme) {
      setTheme(targetTheme.id);
    }
  };
  
  mediaQuery.addEventListener('change', handler);
  
  // Return cleanup function
  return () => mediaQuery.removeEventListener('change', handler);
}
```

#### 3. CSS Variables Base (`src/app.css`)

```css
/* Base CSS variables with fallback values */
:root {
  /* App-level */
  --app-background: #1e1e1e;
  --app-foreground: #cccccc;
  --app-border: #404040;
  
  /* Panels */
  --panel-background: #252526;
  --panel-foreground: #cccccc;
  --panel-border: #3c3c3c;
  --panel-headerBackground: #2d2d2d;
  
  /* Sidebar */
  --sidebar-background: #252526;
  --sidebar-foreground: #cccccc;
  --sidebar-activeItem: #4fc3f7;
  
  /* Charts */
  --chart-background: #1e1e1e;
  --chart-gridLine: #404040;
  --chart-axisLine: #808080;
  --chart-axisLabel: #999999;
  
  /* Well logs */
  --wellLog-trackBackground: #2d2d2d;
  --wellLog-trackBorder: #4a4a4a;
  --wellLog-depthScale: #cccccc;
  --wellLog-curveDefault: #4fc3f7;
  --wellLog-curveSecondary: #ffb74d;
  --wellLog-fillPositive: #81c784;
  --wellLog-fillNegative: #e57373;
  
  /* Status */
  --status-info: #29b6f6;
  --status-success: #66bb6a;
  --status-warning: #ffa726;
  --status-error: #ef5350;
  
  /* Interactive */
  --button-background: #0e639c;
  --button-foreground: #ffffff;
  --button-hoverBackground: #1177bb;
  --input-background: #3c3c3c;
  --input-foreground: #cccccc;
  --input-border: #3c3c3c;
}

/* Light theme overrides via data attribute */
[data-theme="light"] {
  --app-background: #ffffff;
  --app-foreground: #333333;
  --app-border: #e0e0e0;
  
  --panel-background: #f5f5f5;
  --panel-foreground: #333333;
  --panel-border: #e0e0e0;
  --panel-headerBackground: #e8e8e8;
  
  --sidebar-background: #f3f3f3;
  --sidebar-foreground: #333333;
  --sidebar-activeItem: #0066cc;
  
  --chart-background: #ffffff;
  --chart-gridLine: #e0e0e0;
  --chart-axisLine: #333333;
  --chart-axisLabel: #666666;
  
  --wellLog-trackBackground: #fafafa;
  --wellLog-trackBorder: #cccccc;
  --wellLog-depthScale: #333333;
  --wellLog-curveDefault: #0066cc;
  --wellLog-curveSecondary: #cc6600;
  --wellLog-fillPositive: #4caf50;
  --wellLog-fillNegative: #f44336;
  
  --status-info: #0288d1;
  --status-success: #388e3c;
  --status-warning: #f57c00;
  --status-error: #d32f2f;
  
  --button-background: #0066cc;
  --button-foreground: #ffffff;
  --button-hoverBackground: #0052a3;
  --input-background: #ffffff;
  --input-foreground: #333333;
  --input-border: #cccccc;
}

/* Global styles using CSS variables */
body {
  background-color: var(--app-background);
  color: var(--app-foreground);
  transition: background-color 0.2s ease, color 0.2s ease;
}
```

#### 4. App Initialization (`src/routes/+layout.svelte`)

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { restoreTheme, setupOsThemeListener } from '$lib/theme/themeService';
  import '../app.css';
  
  onMount(() => {
    // Restore theme from localStorage
    restoreTheme();
    
    // Optional: Listen for OS theme changes
    const cleanup = setupOsThemeListener();
    
    return cleanup;
  });
</script>

<slot />
```

#### 5. Theme Toggle Component (`src/lib/components/ThemeToggle.svelte`)

```svelte
<script lang="ts">
  import { toggleTheme, themeType } from '$lib/theme/themeService';
  import { Sun, Moon } from 'lucide-svelte';
</script>

<button 
  class="theme-toggle"
  onclick={toggleTheme}
  aria-label={$themeType === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'}
>
  {#if $themeType === 'dark'}
    <Sun size={18} />
  {:else}
    <Moon size={18} />
  {/if}
</button>

<style>
  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 4px;
    background: var(--button-background);
    color: var(--button-foreground);
    cursor: pointer;
    transition: background-color 0.15s ease;
  }
  
  .theme-toggle:hover {
    background: var(--button-hoverBackground);
  }
</style>
```

#### 6. Per-View Styling Example (`src/lib/components/WellLogTrack.svelte`)

```svelte
<script lang="ts">
  import { currentTheme, getColor } from '$lib/theme/themeService';
  import type { ColorToken } from '$lib/theme/types';
  
  // Props for per-view color overrides
  export let trackId: string;
  export let curveColor: string | undefined = undefined;
  export let backgroundColor: string | undefined = undefined;
  
  // Reactive: use override or fall back to theme
  $: resolvedBg = backgroundColor ?? $currentTheme.colors['wellLog.trackBackground'];
  $: resolvedCurve = curveColor ?? $currentTheme.colors['wellLog.curveDefault'];
</script>

<div 
  class="well-log-track"
  style:--track-bg={resolvedBg}
  style:--curve-color={resolvedCurve}
  data-track-id={trackId}
>
  <div class="track-header">
    <slot name="header" />
  </div>
  <div class="track-content">
    <slot />
  </div>
</div>

<style>
  .well-log-track {
    display: flex;
    flex-direction: column;
    background-color: var(--track-bg, var(--wellLog-trackBackground));
    border: 1px solid var(--wellLog-trackBorder);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .track-header {
    padding: 8px 12px;
    background: var(--panel-headerBackground);
    border-bottom: 1px solid var(--wellLog-trackBorder);
    font-size: 12px;
    font-weight: 500;
    color: var(--panel-foreground);
  }
  
  .track-content {
    flex: 1;
    position: relative;
  }
  
  /* Curves inside this track will use the resolved color */
  .well-log-track :global(.curve-path) {
    stroke: var(--curve-color, var(--wellLog-curveDefault));
  }
</style>
```

### Call Stacks (Simplified App)

#### Startup Restoration

```
App.svelte onMount()
  └─ restoreTheme()
     └─ localStorage.getItem('app.colorTheme')
        └─ Returns 'default-dark' (or stored value)
     └─ currentThemeId.set(themeId)
     └─ applyThemeToDom(themeId)
        └─ theme = themes.get(themeId)
        └─ document.documentElement.setAttribute('data-theme', theme.type)
        └─ for (token, value) of theme.colors:
           └─ document.documentElement.style.setProperty(`--${token}`, value)
        └─ window.dispatchEvent(new CustomEvent('themechange'))
     └─ All $currentTheme subscribers reactively update
```

#### Runtime Toggle

```
User clicks ThemeToggle button
  └─ toggleTheme()
     └─ $current = get(currentTheme)
     └─ oppositeType = $current.type === 'dark' ? 'light' : 'dark'
     └─ oppositeTheme = themes.find(t => t.type === oppositeType)
     └─ setTheme(oppositeTheme.id)
        └─ currentThemeId.set(themeId)
        └─ persistTheme(themeId)
           └─ localStorage.setItem('app.colorTheme', themeId)
        └─ applyThemeToDom(themeId)
           └─ CSS variables updated on :root
           └─ data-theme attribute updated
           └─ 'themechange' event dispatched
        └─ Svelte reactivity propagates:
           └─ $currentTheme derived store updates
           └─ All components using $currentTheme re-render
           └─ Components using CSS vars repaint automatically (browser)
```

---

## Summary Comparison

| Aspect | VS Code Approach | Minimal App Approach |
|--------|------------------|---------------------|
| **Theme Model** | `ColorThemeData` class with JSON parsing, inheritance | Simple `ITheme` interface with `Record<string, string>` |
| **Color Registry** | `registerColor()` with defaults, descriptions | `COLOR_TOKENS` const with light/dark defaults |
| **Persistence** | `settings.json` via `ConfigurationService` | `localStorage` with simple key |
| **CSS Injection** | `--vscode-*` vars on `<body>`, `<style>` tag for rules | `--app-*` vars on `:root`, `[data-theme]` selectors |
| **Event System** | `_onDidColorThemeChange` Emitter pattern | Svelte stores + `CustomEvent` |
| **Component Styling** | `registerThemingParticipant` callbacks | CSS vars + reactive `$currentTheme` |
| **Per-View Override** | `workbench.colorCustomizations` per-theme | Props or scoped CSS variables |
| **Extension Support** | `contributes.themes` in `package.json` | `registerTheme()` function |
| **OS Theme Sync** | Electron `nativeTheme` IPC | `matchMedia('prefers-color-scheme')` |

---

## References

- [VS Code Theming API Documentation](https://code.visualstudio.com/api/extension-capabilities/theming)
- [VS Code Color Theme Reference](https://code.visualstudio.com/docs/getstarted/themes)
- [VS Code Source: workbench/services/themes](https://github.com/microsoft/vscode/tree/main/src/vs/workbench/services/themes)
- [VS Code Source: platform/theme](https://github.com/microsoft/vscode/tree/main/src/vs/platform/theme)

