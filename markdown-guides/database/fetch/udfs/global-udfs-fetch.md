# Global UDFs Fetch and Rendering Flow

## Overview

This document explains how UDFs (User-Defined Functions) are fetched, categorized, and rendered in the UI using the global state management pattern in Svelte 5.

## Architecture Flow

### 1. Global State Management (`udfs.svelte.ts`)

The `GlobalUDFLibrary` class manages the global state for all UDFs:

- **State Properties**:
  - `udfs`: Array of `UdfCategoryInfo[]` - UDFs organized by category
  - `allFunctions`: Array of `SqlFunction[]` - All functions including UDFs in SQL format
  - `isLoading`: Boolean flag for loading state
  - `isInitialized`: Boolean flag to prevent duplicate initialization

- **Initialization**:
  - Called via `setGlobalUDFLibrary()` in the parent layout
  - Automatically initializes on creation
  - Fetches UDFs from the backend service

### 2. Data Fetching (`udf-service.ts`)

The `UdfService` singleton handles communication with the backend:

- **Method**: `getUdfsByCategory()`
- **Backend Call**: Uses Tauri's `invoke("get_all_udfs")` to call Rust backend
- **NOT a Database Call**: This is a **frontend-to-backend call** (Tauri command), not a database query
- **Caching**: Service caches results after first fetch to avoid redundant calls

### 3. Backend Categorization (`src-tauri/src/udf_commands.rs`)

The Rust backend organizes UDFs by category:

- **Source**: Reads from in-memory UDF registry (`create_default_registry()`)
- **Categories**: Defined as enum in Rust:
  - `ShaleVolume` → "Shale Volume"
  - `Porosity` → "Porosity"
  - `Saturation` → "Saturation"
  - `Statistics` → "Statistics"
  - `UnitConversion` → "Unit Conversion"
  - `Quality` → "Quality"

- **Process**:
  1. Gets all UDF definitions from registry
  2. Groups them by `UdfCategory` enum
  3. Converts to `UdfCategoryInfo` with `name` and `display_name`
  4. Returns sorted array of categories

### 4. UI Rendering (`content-library-udfs.svelte`)

The component consumes the global state:

- **State Access**: Uses `getGlobalUDFLibrary()` to access global state
- **Reactive Rendering**: Automatically re-renders when w`udfLibrary.udfs` changes
- **Category Display**:
  - Shows category `display_name` (e.g., "Shale Volume")
  - Shows count of UDFs in each category
  - Uses Collapsible component for expand/collapse functionality

## Data Flow Summary

```
Rust UDF Registry (in-memory)
    ↓
Tauri Command: get_all_udfs()
    ↓
UdfService.getUdfsByCategory() (frontend service)
    ↓
GlobalUDFLibrary.initialize() (global state)
    ↓
content-library-udfs.svelte (UI component)
```

## Key Points

1. **No Database Calls**: All UDF definitions come from the Rust in-memory registry, not a database
2. **Single Fetch**: UDFs are fetched once on initialization and cached
3. **Categorization**: Happens in Rust backend based on `UdfCategory` enum
4. **Global State**: Uses Svelte 5's `$state` for reactive updates across components
5. **Type Safety**: Full TypeScript types from Rust backend via Tauri serialization
