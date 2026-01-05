# JupyterLab Theming System: Deep Dive Analysis

This document provides a comprehensive analysis of JupyterLab's theming system, covering architecture, implementation details, and guidance for building similar systems.

---

## Table of Contents

1. [Theme Architecture](#1-theme-architecture)
2. [Styling Mechanism](#2-styling-mechanism)
3. [Settings & Persistence](#3-settings--persistence)
4. [Call Stack: Theme Switching](#4-call-stack-theme-switching)
5. [Extensibility](#5-extensibility)
6. [Key Source Folders/Files](#6-key-source-foldersfiles)
7. [Implementation Guidance](#7-guidance-implementing-a-similar-theming-subsystem)

---

## 1. Theme Architecture

### ThemeManager Design

The `ThemeManager` is the central orchestrator for JupyterLab's theming system, located in the `@jupyterlab/apputils` package. It follows a **registry pattern** with the following core responsibilities:

```typescript
// Core ThemeManager interface (IThemeManager)
interface IThemeManager {
  // Signal emitted when theme changes
  readonly themeChanged: ISignal<this, IChangedArgs<string, string | null>>;
  
  // Currently active theme name
  readonly theme: string | null;
  
  // Whether current theme is light
  readonly isLight: boolean;
  
  // List of registered theme names
  readonly themes: ReadonlyArray<string>;
  
  // Register a new theme
  register(theme: IThemeManager.ITheme): void;
  
  // Set the active theme
  setTheme(name: string): Promise<void>;
  
  // Load CSS for a theme
  loadCSS(path: string): Promise<void>;
}
```

### Theme Registration (Extension-Based)

Themes are packaged as **JupyterLab extensions**. Each theme extension declares itself in `package.json`:

```json
{
  "name": "@jupyterlab/theme-dark-extension",
  "version": "4.x.x",
  "jupyterlab": {
    "extension": true,
    "themePath": "style/theme.css",
    "themeConfig": {
      "name": "JupyterLab Dark",
      "isLight": false
    }
  },
  "style": "style/index.css"
}
```

**Key fields:**

| Field | Purpose |
|-------|---------|
| `jupyterlab.themePath` | Path to the main theme CSS file |
| `jupyterlab.themeConfig.name` | Display name for the theme |
| `jupyterlab.themeConfig.isLight` | Boolean declaring light (`true`) or dark (`false`) |
| `style` | Entry point for the theme's CSS |

### Light vs Dark Determination

The `isLight` property in `themeConfig` is the authoritative source. The `ThemeManager` exposes this via:

```typescript
class ThemeManager implements IThemeManager {
  get isLight(): boolean {
    return this._isLight;
  }
  
  private async _loadTheme(name: string): Promise<void> {
    const theme = this._themes.get(name);
    this._isLight = theme?.isLight ?? true;
    // ...
  }
}
```

---

## 2. Styling Mechanism

### CSS Variable System

JupyterLab uses a comprehensive **CSS variable system** with the `--jp-` prefix:

```css
/* Core variables defined by themes */
:root {
  /* Layout colors */
  --jp-layout-color0: #111111;
  --jp-layout-color1: #212121;
  --jp-layout-color2: #424242;
  --jp-layout-color3: #616161;
  --jp-layout-color4: #757575;
  
  /* UI font colors */
  --jp-ui-font-color0: rgba(255, 255, 255, 1);
  --jp-ui-font-color1: rgba(255, 255, 255, 0.87);
  --jp-ui-font-color2: rgba(255, 255, 255, 0.54);
  --jp-ui-font-color3: rgba(255, 255, 255, 0.38);
  
  /* Brand colors */
  --jp-brand-color0: #2196f3;
  --jp-brand-color1: #1976d2;
  --jp-brand-color2: #1565c0;
  --jp-brand-color3: #0d47a1;
  
  /* Border colors */
  --jp-border-color0: #1e1e1e;
  --jp-border-color1: #3d3d3d;
  --jp-border-color2: #5a5a5a;
  --jp-border-color3: #8a8a8a;
  
  /* Syntax highlighting (CodeMirror) */
  --jp-mirror-editor-keyword-color: #c792ea;
  --jp-mirror-editor-string-color: #c3e88d;
  --jp-mirror-editor-number-color: #f78c6c;
  --jp-mirror-editor-comment-color: #546e7a;
  --jp-mirror-editor-variable-color: #82aaff;
  
  /* Cell colors */
  --jp-cell-editor-background: var(--jp-layout-color1);
  --jp-cell-editor-border-color: var(--jp-border-color1);
  --jp-cell-editor-active-background: var(--jp-layout-color0);
  
  /* Notebook colors */
  --jp-notebook-multiselected-color: rgba(33, 150, 243, 0.24);
  
  /* Scrollbar colors */
  --jp-scrollbar-thumb-color: rgba(255, 255, 255, 0.3);
  --jp-scrollbar-track-color: transparent;
}
```

### CSS Loading/Unloading Process

```typescript
class ThemeManager {
  private _links: Map<string, HTMLLinkElement> = new Map();
  
  /**
   * Load a CSS file by creating a <link> element and appending to <head>
   */
  async loadCSS(path: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const link = document.createElement('link');
      link.rel = 'stylesheet';
      link.type = 'text/css';
      link.href = path;
      link.onload = () => resolve();
      link.onerror = () => reject(new Error(`Failed to load ${path}`));
      document.head.appendChild(link);
      this._links.set(path, link);
    });
  }
  
  /**
   * Unload a CSS file by removing its <link> element from <head>
   */
  private _unloadCSS(path: string): void {
    const link = this._links.get(path);
    if (link) {
      document.head.removeChild(link);
      this._links.delete(path);
    }
  }
}
```

### Theme Asset Packaging

Theme extensions bundle assets in a `style/` directory:

```
@jupyterlab/theme-dark-extension/
├── package.json
├── style/
│   ├── theme.css          # Main theme variables
│   ├── index.css          # Entry point (imports theme.css)
│   ├── variables.css      # Additional variable definitions
│   └── icons/             # Theme-specific icons (optional)
│       ├── folder.svg
│       └── file.svg
└── src/
    └── index.ts           # Extension activation code
```

---

## 3. Settings & Persistence

### Storage Location

User theme preferences are stored in:

```
~/.jupyter/lab/user-settings/@jupyterlab/apputils-extension/themes.jupyterlab-settings
```

### Settings Schema

```json
{
  "theme": "JupyterLab Dark",
  "adaptive-theme": false,
  "theme-scrollbars": true,
  "preferred-dark-theme": "JupyterLab Dark",
  "preferred-light-theme": "JupyterLab Light"
}
```

| Setting | Type | Description |
|---------|------|-------------|
| `theme` | string | Currently selected theme name |
| `adaptive-theme` | boolean | Follow system light/dark preference |
| `theme-scrollbars` | boolean | Apply theme colors to scrollbars |
| `preferred-dark-theme` | string | Theme to use when system is in dark mode |
| `preferred-light-theme` | string | Theme to use when system is in light mode |

### Settings Registry Integration

The `@jupyterlab/settingregistry` package manages persistence:

```typescript
import { ISettingRegistry } from '@jupyterlab/settingregistry';

// Theme plugin reads/writes settings via ISettingRegistry
const PLUGIN_ID = '@jupyterlab/apputils-extension:themes';

async function activate(
  app: JupyterFrontEnd,
  settingRegistry: ISettingRegistry,
  themeManager: IThemeManager
): Promise<void> {
  // Load settings
  const settings = await settingRegistry.load(PLUGIN_ID);
  
  // Read current theme preference
  const themeName = settings.get('theme').composite as string;
  
  // Apply the theme
  await themeManager.setTheme(themeName);
  
  // Listen for settings changes
  settings.changed.connect(() => {
    const newTheme = settings.get('theme').composite as string;
    if (newTheme !== themeManager.theme) {
      themeManager.setTheme(newTheme);
    }
  });
}

// Update theme preference (persists automatically)
await settings.set('theme', 'JupyterLab Light');
```

---

## 4. Call Stack: Theme Switching

### Initialization Sequence (App Start - Restore Theme)

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. JupyterLab Application Bootstrap                              │
│    → JupyterFrontEnd.start()                                     │
├─────────────────────────────────────────────────────────────────┤
│ 2. Plugin System Activates Extensions                            │
│    → Plugins sorted by dependency order                          │
│    → Theme extensions register with ThemeManager                 │
├─────────────────────────────────────────────────────────────────┤
│ 3. ThemeManager Plugin Activates                                 │
│    → @jupyterlab/apputils-extension:themes                       │
│    → Receives IThemeManager, ISettingRegistry tokens             │
├─────────────────────────────────────────────────────────────────┤
│ 4. Load User Settings                                            │
│    → settingRegistry.load('@jupyterlab/apputils-extension:themes')│
│    → Reads ~/.jupyter/lab/user-settings/...                      │
│    → Returns settings object with 'theme' property               │
├─────────────────────────────────────────────────────────────────┤
│ 5. Determine Theme to Apply                                      │
│    → const themeName = settings.get('theme').composite           │
│    → If no stored theme, use default ('JupyterLab Light')        │
│    → If adaptive-theme enabled, check system preference          │
├─────────────────────────────────────────────────────────────────┤
│ 6. ThemeManager.setTheme(storedThemeName)                        │
│    → Validates theme exists in registry                          │
│    → Calls internal _loadTheme(name)                             │
├─────────────────────────────────────────────────────────────────┤
│ 7. ThemeManager._loadTheme(name)                                 │
│    → Lookup theme in internal _themes Map                        │
│    → Set this._isLight from theme.isLight config                 │
│    → Get CSS path from theme configuration                       │
├─────────────────────────────────────────────────────────────────┤
│ 8. ThemeManager.loadCSS(themePath)                               │
│    → Create <link rel="stylesheet"> element                      │
│    → Set href to theme CSS path                                  │
│    → Append to document.head                                     │
│    → Return Promise that resolves on link.onload                 │
├─────────────────────────────────────────────────────────────────┤
│ 9. CSS Applied to DOM                                            │
│    → Browser parses CSS                                          │
│    → CSS variables cascade to all elements                       │
│    → UI renders with theme colors                                │
├─────────────────────────────────────────────────────────────────┤
│ 10. Emit themeChanged Signal                                     │
│     → this._themeChanged.emit({ oldValue: null, newValue: name })│
│     → All connected listeners receive notification               │
│     → Widgets update icons, syntax highlighting, etc.            │
└─────────────────────────────────────────────────────────────────┘
```

### Runtime Switch Sequence (User Changes Theme)

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. User Action                                                   │
│    → Settings menu > Theme > "JupyterLab Dark"                   │
│    → Or: Command Palette > "Change Theme"                        │
│    → Or: Keyboard shortcut (if configured)                       │
├─────────────────────────────────────────────────────────────────┤
│ 2. Command Execution                                             │
│    → commands.execute('apputils:change-theme', { theme: name })  │
│    → Command handler invokes settings.set('theme', name)         │
├─────────────────────────────────────────────────────────────────┤
│ 3. Settings Update Triggers Theme Change                         │
│    → settings.changed signal emits                               │
│    → Theme plugin handler calls themeManager.setTheme(name)      │
├─────────────────────────────────────────────────────────────────┤
│ 4. ThemeManager.setTheme(name: string): Promise<void>            │
│    ├── Validate theme exists in registry                         │
│    ├── Store previous theme reference (oldTheme)                 │
│    ├── Check if theme is already active (early return if same)   │
│    └── Call _loadTheme(name)                                     │
├─────────────────────────────────────────────────────────────────┤
│ 5. ThemeManager._loadTheme(name)                                 │
│    ├── Get current theme's CSS path                              │
│    ├── Unload current theme: _unloadCSS(currentPath)             │
│    │   └── Remove <link> element from document.head              │
│    ├── Update _isLight property from new theme config            │
│    ├── Load new theme: loadCSS(newPath)                          │
│    │   └── Create and append new <link> element                  │
│    └── Update _theme property to new name                        │
├─────────────────────────────────────────────────────────────────┤
│ 6. Persist to Settings (Already Done in Step 2)                  │
│    → settings.set('theme', name) writes to JSON file             │
│    → ~/.jupyter/lab/user-settings/.../themes.jupyterlab-settings │
├─────────────────────────────────────────────────────────────────┤
│ 7. Emit themeChanged Signal                                      │
│    → this._themeChanged.emit({                                   │
│        oldValue: 'JupyterLab Light',                             │
│        newValue: 'JupyterLab Dark'                               │
│      })                                                          │
├─────────────────────────────────────────────────────────────────┤
│ 8. UI Components React to Signal                                 │
│    ├── Notebook widgets refresh cell styling                     │
│    ├── File browser updates folder/file icons                    │
│    ├── Terminal updates colors                                   │
│    ├── CodeMirror editors update syntax highlighting             │
│    ├── Sidebar icons swap to light/dark variants                 │
│    └── Status bar updates appearance                             │
├─────────────────────────────────────────────────────────────────┤
│ 9. Theme Switch Complete                                         │
│    → All CSS variables now reflect new theme                     │
│    → User sees updated UI without page reload                    │
└─────────────────────────────────────────────────────────────────┘
```

### Key Methods & Signals Reference

| Method/Signal | Location | Purpose |
|--------------|----------|---------|
| `register(theme)` | ThemeManager | Add theme to internal registry Map |
| `setTheme(name)` | ThemeManager | Public API to activate a theme |
| `_loadTheme(name)` | ThemeManager | Internal method to load theme CSS |
| `loadCSS(path)` | ThemeManager | Inject CSS `<link>` into DOM |
| `_unloadCSS(path)` | ThemeManager | Remove CSS `<link>` from DOM |
| `themeChanged` | ThemeManager | Signal emitted on theme switch |
| `isLight` | ThemeManager | Getter for current theme's light/dark state |
| `theme` | ThemeManager | Getter for current theme name |
| `themes` | ThemeManager | Getter for list of registered theme names |

---

## 5. Extensibility

### Creating a Custom Theme Extension

#### Step 1: Scaffold the Extension

```bash
# Using the official JupyterLab extension template
pip install copier jinja2-time
copier copy https://github.com/jupyterlab/extension-template my-custom-theme

# Or using cookiecutter
cookiecutter https://github.com/jupyterlab/extension-cookiecutter-ts
```

#### Step 2: Configure `package.json`

```json
{
  "name": "my-custom-theme",
  "version": "1.0.0",
  "description": "A custom JupyterLab theme",
  "keywords": [
    "jupyter",
    "jupyterlab",
    "jupyterlab-extension",
    "jupyterlab-theme"
  ],
  "license": "MIT",
  "author": {
    "name": "Your Name"
  },
  "files": [
    "lib/**/*.{d.ts,eot,gif,html,jpg,js,js.map,json,png,svg,woff2,ttf}",
    "style/**/*.{css,eot,gif,html,jpg,json,png,svg,woff2,ttf}"
  ],
  "main": "lib/index.js",
  "types": "lib/index.d.ts",
  "jupyterlab": {
    "extension": true,
    "themePath": "style/theme.css",
    "themeConfig": {
      "name": "My Custom Theme",
      "isLight": false
    }
  },
  "style": "style/index.css",
  "dependencies": {
    "@jupyterlab/application": "^4.0.0",
    "@jupyterlab/apputils": "^4.0.0"
  },
  "devDependencies": {
    "@jupyterlab/builder": "^4.0.0",
    "typescript": "~5.0.0"
  }
}
```

#### Step 3: Define CSS Variables (`style/theme.css`)

```css
/*
 * My Custom Theme - A dark theme with custom accent colors
 * 
 * Override JupyterLab's CSS variables to customize appearance.
 * Reference: https://github.com/jupyterlab/jupyterlab/blob/main/packages/theme-light-extension/style/variables.css
 */

:root {
  /* ==========================================================================
   * Layout Colors
   * ========================================================================== */
  --jp-layout-color0: #1a1a2e;      /* Darkest - main background */
  --jp-layout-color1: #16213e;      /* Dark - secondary background */
  --jp-layout-color2: #0f3460;      /* Medium - tertiary background */
  --jp-layout-color3: #1a1a4e;      /* Light - quaternary background */
  --jp-layout-color4: #2a2a5e;      /* Lightest - highlights */

  /* ==========================================================================
   * Brand Colors
   * ========================================================================== */
  --jp-brand-color0: #e94560;       /* Primary brand - buttons, links */
  --jp-brand-color1: #ff6b6b;       /* Hover state */
  --jp-brand-color2: #ff8585;       /* Active state */
  --jp-brand-color3: #ffa5a5;       /* Disabled state */

  /* ==========================================================================
   * UI Font Colors
   * ========================================================================== */
  --jp-ui-font-color0: #ffffff;
  --jp-ui-font-color1: rgba(255, 255, 255, 0.87);
  --jp-ui-font-color2: rgba(255, 255, 255, 0.60);
  --jp-ui-font-color3: rgba(255, 255, 255, 0.38);

  /* ==========================================================================
   * Content Font Colors
   * ========================================================================== */
  --jp-content-font-color0: #ffffff;
  --jp-content-font-color1: rgba(255, 255, 255, 0.87);
  --jp-content-font-color2: rgba(255, 255, 255, 0.60);
  --jp-content-font-color3: rgba(255, 255, 255, 0.38);

  /* ==========================================================================
   * Border Colors
   * ========================================================================== */
  --jp-border-color0: #0f3460;
  --jp-border-color1: #16213e;
  --jp-border-color2: #1a1a4e;
  --jp-border-color3: #2a2a5e;

  /* ==========================================================================
   * Inverse Colors (for light-on-dark elements)
   * ========================================================================== */
  --jp-inverse-layout-color0: #ffffff;
  --jp-inverse-layout-color1: #f5f5f5;
  --jp-inverse-layout-color2: #eeeeee;

  /* ==========================================================================
   * Accent Colors
   * ========================================================================== */
  --jp-accent-color0: #e94560;
  --jp-accent-color1: #ff6b6b;
  --jp-accent-color2: #ff8585;
  --jp-accent-color3: #ffa5a5;

  /* ==========================================================================
   * Status Colors
   * ========================================================================== */
  --jp-success-color0: #4caf50;
  --jp-success-color1: #81c784;
  --jp-info-color0: #2196f3;
  --jp-info-color1: #64b5f6;
  --jp-warn-color0: #ff9800;
  --jp-warn-color1: #ffb74d;
  --jp-error-color0: #f44336;
  --jp-error-color1: #e57373;

  /* ==========================================================================
   * Cell Colors
   * ========================================================================== */
  --jp-cell-editor-background: var(--jp-layout-color1);
  --jp-cell-editor-border-color: var(--jp-border-color1);
  --jp-cell-editor-active-background: var(--jp-layout-color0);
  --jp-cell-editor-active-border-color: var(--jp-brand-color0);
  --jp-cell-prompt-width: 64px;
  --jp-cell-prompt-font-family: var(--jp-code-font-family);
  --jp-cell-prompt-letter-spacing: 0px;
  --jp-cell-prompt-opacity: 1;
  --jp-cell-prompt-not-active-opacity: 0.5;
  --jp-cell-inprompt-font-color: var(--jp-brand-color0);
  --jp-cell-outprompt-font-color: var(--jp-accent-color0);

  /* ==========================================================================
   * Notebook Colors
   * ========================================================================== */
  --jp-notebook-padding: 10px;
  --jp-notebook-select-background: var(--jp-layout-color1);
  --jp-notebook-multiselected-color: rgba(233, 69, 96, 0.24);

  /* ==========================================================================
   * Syntax Highlighting (CodeMirror)
   * ========================================================================== */
  --jp-mirror-editor-keyword-color: #c792ea;
  --jp-mirror-editor-atom-color: #f78c6c;
  --jp-mirror-editor-number-color: #f78c6c;
  --jp-mirror-editor-def-color: #82aaff;
  --jp-mirror-editor-variable-color: #f07178;
  --jp-mirror-editor-variable-2-color: #eeffff;
  --jp-mirror-editor-variable-3-color: #decb6b;
  --jp-mirror-editor-punctuation-color: #89ddff;
  --jp-mirror-editor-property-color: #82aaff;
  --jp-mirror-editor-operator-color: #89ddff;
  --jp-mirror-editor-comment-color: #546e7a;
  --jp-mirror-editor-string-color: #c3e88d;
  --jp-mirror-editor-string-2-color: #f07178;
  --jp-mirror-editor-meta-color: #ffcb6b;
  --jp-mirror-editor-qualifier-color: #decb6b;
  --jp-mirror-editor-builtin-color: #ffcb6b;
  --jp-mirror-editor-bracket-color: #89ddff;
  --jp-mirror-editor-tag-color: #f07178;
  --jp-mirror-editor-attribute-color: #c792ea;
  --jp-mirror-editor-header-color: #c792ea;
  --jp-mirror-editor-quote-color: #c3e88d;
  --jp-mirror-editor-link-color: #f78c6c;
  --jp-mirror-editor-error-color: #ff5370;

  /* ==========================================================================
   * Scrollbar Colors
   * ========================================================================== */
  --jp-scrollbar-background-color: var(--jp-layout-color1);
  --jp-scrollbar-thumb-color: rgba(255, 255, 255, 0.3);
  --jp-scrollbar-endpad: 3px;

  /* ==========================================================================
   * Sidebar & Panel Colors
   * ========================================================================== */
  --jp-sidebar-min-width: 250px;

  /* ==========================================================================
   * Input/Output Colors
   * ========================================================================== */
  --jp-input-box-shadow: inset 0 0 2px var(--jp-brand-color0);
  --jp-input-active-background: var(--jp-layout-color1);
  --jp-input-hover-background: var(--jp-layout-color1);
  --jp-input-active-box-shadow-color: var(--jp-brand-color0);
}

/* Additional component-specific styles */
.jp-Toolbar {
  background: var(--jp-layout-color1);
  border-bottom: 1px solid var(--jp-border-color1);
}

.jp-SideBar {
  background: var(--jp-layout-color1);
}

/* Ensure proper contrast for selected items */
.jp-DirListing-item.jp-mod-selected {
  background: var(--jp-brand-color0);
  color: var(--jp-ui-font-color0);
}
```

#### Step 4: Create Entry Point (`style/index.css`)

```css
/* Import the main theme variables */
@import url('theme.css');

/* Any additional base styles can go here */
```

#### Step 5: Register in Extension Activation (`src/index.ts`)

```typescript
import {
  JupyterFrontEnd,
  JupyterFrontEndPlugin
} from '@jupyterlab/application';

import { IThemeManager } from '@jupyterlab/apputils';

/**
 * Custom theme extension for JupyterLab
 */
const plugin: JupyterFrontEndPlugin<void> = {
  id: 'my-custom-theme:plugin',
  description: 'A custom dark theme for JupyterLab',
  autoStart: true,
  requires: [IThemeManager],
  activate: (app: JupyterFrontEnd, manager: IThemeManager) => {
    console.log('JupyterLab extension my-custom-theme is activated!');
    
    const style = 'my-custom-theme/style/index.css';
    
    manager.register({
      name: 'My Custom Theme',
      isLight: false,
      themeScrollbars: true,
      load: () => manager.loadCSS(style),
      unload: () => Promise.resolve()
    });
  }
};

export default plugin;
```

#### Step 6: Build and Install

```bash
# Install dependencies
npm install

# Build the extension
npm run build

# Install in development mode
pip install -e .
jupyter labextension develop . --overwrite

# Or build for distribution
npm run build:prod
pip install .
```

### Compatibility Considerations

1. **Use CSS variables exclusively** - Don't hardcode colors; override `--jp-*` variables
2. **Test with core extensions** - Ensure compatibility with notebooks, terminals, file browser
3. **Handle both states** - If supporting adaptive themes, provide both light/dark variants
4. **Version compatibility** - CSS variable names may change between major JupyterLab versions
5. **Test scrollbars** - Use `themeScrollbars: true` if your theme customizes scrollbar colors
6. **Icon compatibility** - Some icons have light/dark variants; test icon visibility

---

## 6. Key Source Folders/Files

### Package Locations

| Package | Path | Purpose |
|---------|------|---------|
| `@jupyterlab/apputils` | `packages/apputils/` | ThemeManager, theme interfaces |
| `@jupyterlab/apputils-extension` | `packages/apputils-extension/` | Theme plugin activation |
| `@jupyterlab/settingregistry` | `packages/settingregistry/` | Settings persistence |
| `@jupyterlab/theme-light-extension` | `packages/theme-light-extension/` | Default light theme |
| `@jupyterlab/theme-dark-extension` | `packages/theme-dark-extension/` | Default dark theme |

### Key Files

| File | Location | Purpose |
|------|----------|---------|
| `thememanager.ts` | `packages/apputils/src/thememanager.ts` | ThemeManager class implementation |
| `tokens.ts` | `packages/apputils/src/tokens.ts` | IThemeManager interface definition |
| `themesplugins.ts` | `packages/apputils-extension/src/themesplugins.ts` | Theme plugin activation code |
| `index.ts` | `packages/settingregistry/src/index.ts` | Settings registry exports |
| `settingregistry.ts` | `packages/settingregistry/src/settingregistry.ts` | Settings implementation |
| `variables.css` | `packages/theme-light-extension/style/variables.css` | Light theme CSS variables |
| `variables.css` | `packages/theme-dark-extension/style/variables.css` | Dark theme CSS variables |

### User Settings Location

```
~/.jupyter/lab/user-settings/
└── @jupyterlab/
    └── apputils-extension/
        └── themes.jupyterlab-settings    # Theme preferences JSON
```

---

## 7. Guidance: Implementing a Similar Theming Subsystem

### Core Architecture Components

To implement a JupyterLab-style theming system in your own application, you need:

1. **Theme Interface** - Define what a theme contains
2. **Theme Manager** - Central registry and switcher
3. **CSS Variable System** - Dynamic styling mechanism
4. **Settings Persistence** - Remember user preferences
5. **Signal/Event System** - Notify components of changes

### Complete Implementation Example

#### 1. Theme Interface

```typescript
// types/theme.ts

/**
 * Interface representing a theme
 */
export interface ITheme {
  /** Unique identifier for the theme */
  name: string;
  
  /** Display name shown in UI */
  displayName: string;
  
  /** Whether this is a light theme (false = dark) */
  isLight: boolean;
  
  /** Path to the theme's CSS file */
  cssPath: string;
  
  /** Optional description */
  description?: string;
  
  /** Optional author information */
  author?: string;
}

/**
 * Arguments for theme change events
 */
export interface IThemeChangedArgs {
  oldTheme: string | null;
  newTheme: string;
}
```

#### 2. Signal Implementation (Framework-Agnostic)

```typescript
// utils/signal.ts

/**
 * A simple signal implementation for event handling
 */
export class Signal<T> {
  private _listeners: Set<(sender: any, args: T) => void> = new Set();
  
  /**
   * Connect a listener to this signal
   */
  connect(listener: (sender: any, args: T) => void): void {
    this._listeners.add(listener);
  }
  
  /**
   * Disconnect a listener from this signal
   */
  disconnect(listener: (sender: any, args: T) => void): void {
    this._listeners.delete(listener);
  }
  
  /**
   * Emit the signal to all connected listeners
   */
  emit(sender: any, args: T): void {
    this._listeners.forEach(listener => {
      try {
        listener(sender, args);
      } catch (error) {
        console.error('Error in signal listener:', error);
      }
    });
  }
  
  /**
   * Disconnect all listeners
   */
  disconnectAll(): void {
    this._listeners.clear();
  }
}
```

#### 3. Theme Manager Implementation

```typescript
// services/ThemeManager.ts

import { ITheme, IThemeChangedArgs } from '../types/theme';
import { Signal } from '../utils/signal';

const STORAGE_KEY = 'app-theme-preference';
const DEFAULT_THEME = 'light';

/**
 * Central manager for application theming
 * 
 * Handles theme registration, activation, CSS loading/unloading,
 * and persistence of user preferences.
 */
export class ThemeManager {
  private _themes: Map<string, ITheme> = new Map();
  private _currentTheme: string | null = null;
  private _styleLinks: Map<string, HTMLLinkElement> = new Map();
  private _themeChanged = new Signal<IThemeChangedArgs>();
  private _isInitialized = false;
  
  /**
   * Signal emitted when the theme changes
   */
  get themeChanged(): Signal<IThemeChangedArgs> {
    return this._themeChanged;
  }
  
  /**
   * Get the current theme name
   */
  get theme(): string | null {
    return this._currentTheme;
  }
  
  /**
   * Check if the current theme is a light theme
   */
  get isLight(): boolean {
    if (!this._currentTheme) return true;
    const theme = this._themes.get(this._currentTheme);
    return theme?.isLight ?? true;
  }
  
  /**
   * Get list of all registered theme names
   */
  get themes(): string[] {
    return Array.from(this._themes.keys());
  }
  
  /**
   * Get theme metadata by name
   */
  getTheme(name: string): ITheme | undefined {
    return this._themes.get(name);
  }
  
  /**
   * Register a new theme
   */
  register(theme: ITheme): void {
    if (this._themes.has(theme.name)) {
      console.warn(`Theme "${theme.name}" is already registered. Overwriting.`);
    }
    this._themes.set(theme.name, theme);
    console.log(`Theme "${theme.displayName}" registered`);
  }
  
  /**
   * Unregister a theme
   */
  unregister(name: string): boolean {
    if (this._currentTheme === name) {
      console.warn(`Cannot unregister active theme "${name}"`);
      return false;
    }
    return this._themes.delete(name);
  }
  
  /**
   * Set the active theme
   */
  async setTheme(name: string): Promise<void> {
    // Validate theme exists
    const theme = this._themes.get(name);
    if (!theme) {
      throw new Error(`Theme "${name}" not found. Available themes: ${this.themes.join(', ')}`);
    }
    
    // Skip if already active
    if (this._currentTheme === name) {
      console.log(`Theme "${name}" is already active`);
      return;
    }
    
    const oldTheme = this._currentTheme;
    
    // Unload current theme CSS
    if (oldTheme) {
      const currentTheme = this._themes.get(oldTheme);
      if (currentTheme) {
        this._unloadCSS(currentTheme.cssPath);
      }
    }
    
    // Load new theme CSS
    await this._loadCSS(theme.cssPath);
    
    // Update state
    this._currentTheme = name;
    
    // Update document attribute for CSS selectors
    document.documentElement.setAttribute('data-theme', name);
    document.documentElement.setAttribute('data-theme-is-light', String(theme.isLight));
    
    // Persist preference
    this._savePreference(name);
    
    // Emit change signal
    this._themeChanged.emit(this, { oldTheme, newTheme: name });
    
    console.log(`Theme switched: ${oldTheme ?? 'none'} → ${name}`);
  }
  
  /**
   * Initialize the theme manager and restore user preference
   */
  async initialize(): Promise<void> {
    if (this._isInitialized) {
      console.warn('ThemeManager already initialized');
      return;
    }
    
    // Get stored preference or use default
    const storedTheme = this._loadPreference();
    const themeToApply = storedTheme && this._themes.has(storedTheme) 
      ? storedTheme 
      : DEFAULT_THEME;
    
    // Apply the theme
    await this.setTheme(themeToApply);
    
    this._isInitialized = true;
    console.log('ThemeManager initialized');
  }
  
  /**
   * Check if system prefers dark mode
   */
  systemPrefersDark(): boolean {
    return window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? false;
  }
  
  /**
   * Listen for system theme preference changes
   */
  onSystemThemeChange(callback: (prefersDark: boolean) => void): () => void {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = (e: MediaQueryListEvent) => callback(e.matches);
    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }
  
  /**
   * Load a CSS file by creating a <link> element
   */
  private _loadCSS(path: string): Promise<void> {
    return new Promise((resolve, reject) => {
      // Check if already loaded
      if (this._styleLinks.has(path)) {
        resolve();
        return;
      }
      
      const link = document.createElement('link');
      link.rel = 'stylesheet';
      link.type = 'text/css';
      link.href = path;
      link.id = `theme-${path.replace(/[^a-z0-9]/gi, '-')}`;
      
      link.onload = () => {
        this._styleLinks.set(path, link);
        resolve();
      };
      
      link.onerror = () => {
        reject(new Error(`Failed to load theme CSS: ${path}`));
      };
      
      document.head.appendChild(link);
    });
  }
  
  /**
   * Unload a CSS file by removing its <link> element
   */
  private _unloadCSS(path: string): void {
    const link = this._styleLinks.get(path);
    if (link) {
      link.remove();
      this._styleLinks.delete(path);
    }
  }
  
  /**
   * Save theme preference to storage
   */
  private _savePreference(name: string): void {
    try {
      localStorage.setItem(STORAGE_KEY, name);
    } catch (error) {
      console.warn('Failed to save theme preference:', error);
    }
  }
  
  /**
   * Load theme preference from storage
   */
  private _loadPreference(): string | null {
    try {
      return localStorage.getItem(STORAGE_KEY);
    } catch (error) {
      console.warn('Failed to load theme preference:', error);
      return null;
    }
  }
}

// Singleton instance
export const themeManager = new ThemeManager();
```

#### 4. CSS Theme Files

```css
/* themes/light.css */
:root {
  /* Layout */
  --app-bg-primary: #ffffff;
  --app-bg-secondary: #f8f9fa;
  --app-bg-tertiary: #e9ecef;
  --app-bg-elevated: #ffffff;
  
  /* Text */
  --app-text-primary: #212529;
  --app-text-secondary: #495057;
  --app-text-tertiary: #6c757d;
  --app-text-muted: #adb5bd;
  
  /* Brand */
  --app-brand-primary: #0d6efd;
  --app-brand-hover: #0b5ed7;
  --app-brand-active: #0a58ca;
  
  /* Borders */
  --app-border-color: #dee2e6;
  --app-border-color-light: #e9ecef;
  --app-border-color-dark: #ced4da;
  
  /* Shadows */
  --app-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --app-shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);
  --app-shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);
  
  /* Status */
  --app-success: #198754;
  --app-warning: #ffc107;
  --app-error: #dc3545;
  --app-info: #0dcaf0;
  
  /* Scrollbar */
  --app-scrollbar-track: #f1f1f1;
  --app-scrollbar-thumb: #c1c1c1;
  --app-scrollbar-thumb-hover: #a8a8a8;
  
  /* Focus */
  --app-focus-ring: rgba(13, 110, 253, 0.25);
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--app-scrollbar-track);
}

::-webkit-scrollbar-thumb {
  background: var(--app-scrollbar-thumb);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--app-scrollbar-thumb-hover);
}
```

```css
/* themes/dark.css */
:root {
  /* Layout */
  --app-bg-primary: #1a1a1a;
  --app-bg-secondary: #242424;
  --app-bg-tertiary: #2d2d2d;
  --app-bg-elevated: #333333;
  
  /* Text */
  --app-text-primary: #f8f9fa;
  --app-text-secondary: #ced4da;
  --app-text-tertiary: #adb5bd;
  --app-text-muted: #6c757d;
  
  /* Brand */
  --app-brand-primary: #3b82f6;
  --app-brand-hover: #60a5fa;
  --app-brand-active: #2563eb;
  
  /* Borders */
  --app-border-color: #404040;
  --app-border-color-light: #4a4a4a;
  --app-border-color-dark: #333333;
  
  /* Shadows */
  --app-shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --app-shadow-md: 0 4px 6px rgba(0, 0, 0, 0.4);
  --app-shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.5);
  
  /* Status */
  --app-success: #22c55e;
  --app-warning: #eab308;
  --app-error: #ef4444;
  --app-info: #06b6d4;
  
  /* Scrollbar */
  --app-scrollbar-track: #2d2d2d;
  --app-scrollbar-thumb: #555555;
  --app-scrollbar-thumb-hover: #666666;
  
  /* Focus */
  --app-focus-ring: rgba(59, 130, 246, 0.35);
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--app-scrollbar-track);
}

::-webkit-scrollbar-thumb {
  background: var(--app-scrollbar-thumb);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--app-scrollbar-thumb-hover);
}
```

#### 5. Application Setup

```typescript
// main.ts - Application entry point

import { themeManager } from './services/ThemeManager';

// Register built-in themes
themeManager.register({
  name: 'light',
  displayName: 'Light',
  isLight: true,
  cssPath: '/themes/light.css',
  description: 'Default light theme'
});

themeManager.register({
  name: 'dark',
  displayName: 'Dark',
  isLight: false,
  cssPath: '/themes/dark.css',
  description: 'Default dark theme'
});

// Initialize theme manager (restores user preference)
await themeManager.initialize();

// Listen for theme changes
themeManager.themeChanged.connect((_, { oldTheme, newTheme }) => {
  console.log(`Theme changed from ${oldTheme} to ${newTheme}`);
  // Update any components that need manual refresh
});

// Optional: Follow system preference
themeManager.onSystemThemeChange((prefersDark) => {
  const theme = prefersDark ? 'dark' : 'light';
  themeManager.setTheme(theme);
});

// Mount your application
mountApp();
```

#### 6. Svelte Integration Example

```svelte
<!-- ThemeSwitcher.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { themeManager } from '../services/ThemeManager';
  
  let currentTheme = $state(themeManager.theme);
  let isLight = $state(themeManager.isLight);
  let themes = $state(themeManager.themes);
  
  function handleThemeChange(_: any, { newTheme }: { newTheme: string }) {
    currentTheme = newTheme;
    isLight = themeManager.isLight;
  }
  
  onMount(() => {
    themeManager.themeChanged.connect(handleThemeChange);
  });
  
  onDestroy(() => {
    themeManager.themeChanged.disconnect(handleThemeChange);
  });
  
  async function selectTheme(name: string) {
    await themeManager.setTheme(name);
  }
</script>

<div class="theme-switcher">
  <label for="theme-select">Theme:</label>
  <select 
    id="theme-select" 
    value={currentTheme}
    onchange={(e) => selectTheme(e.currentTarget.value)}
  >
    {#each themes as theme}
      <option value={theme}>
        {themeManager.getTheme(theme)?.displayName ?? theme}
      </option>
    {/each}
  </select>
  
  <span class="theme-indicator" class:light={isLight} class:dark={!isLight}>
    {isLight ? '☀️' : '🌙'}
  </span>
</div>

<style>
  .theme-switcher {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--app-bg-secondary);
    border-radius: 4px;
  }
  
  select {
    padding: 0.25rem 0.5rem;
    background: var(--app-bg-primary);
    color: var(--app-text-primary);
    border: 1px solid var(--app-border-color);
    border-radius: 4px;
  }
  
  .theme-indicator {
    font-size: 1.25rem;
  }
</style>
```

### Summary: Key Implementation Principles

| Principle | JupyterLab Approach | Recommended for Custom Apps |
|-----------|--------------------|-----------------------------|
| **Theme Definition** | Extensions with package.json metadata | Interface-based theme objects |
| **CSS Strategy** | CSS variables with `--jp-` prefix | CSS variables with custom prefix |
| **Loading Mechanism** | Dynamic `<link>` injection | Dynamic `<link>` injection |
| **Persistence** | JSON files in user settings directory | localStorage or IndexedDB |
| **Change Notification** | Lumino Signals | Custom Signal class or framework reactivity |
| **Light/Dark Detection** | `isLight` property in theme config | `isLight` property in theme interface |
| **Extensibility** | Plugin system with token injection | Registration API on ThemeManager |

By following these patterns, you can build a robust, extensible theming system that provides a great user experience with seamless runtime theme switching.

---

## References

- [JupyterLab GitHub Repository](https://github.com/jupyterlab/jupyterlab)
- [JupyterLab Extension Development](https://jupyterlab.readthedocs.io/en/latest/extension/extension_dev.html)
- [JupyterLab CSS Guide](https://jupyterlab.readthedocs.io/en/latest/developer/css.html)
- [@jupyterlab/apputils API](https://jsdocs.io/package/@jupyterlab/apputils)

